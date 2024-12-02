use inquire::Select;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::process::Command;
use std::str;
use std::time::Duration;

pub fn analyze_menu() {
    let options = vec![
        "Analisar disco",
        "Voltar",
    ];

    let selection = Select::new("Selecione uma análise:", options)
        .with_help_message("[Use as setas ↑↓ para navegar e Enter para selecionar]")
        .prompt()
        .expect("Falha ao exibir o menu");

    match selection {
        "Analisar disco" => analyze_disk(),
        "Voltar" => return,
        _ => unreachable!(),
    }
}

pub fn analyze_disk() {
    let root_path = "/";
    let depth = 4;

    println!("\x1b[33mAnalisando disco...\x1b[0m");

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.blue} {msg}")
            .expect("Erro ao configurar spinner"),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message(format!("Calculando tamanhos das pastas até {} níveis...", depth));

    let output = Command::new("du")
        .args(&["-m", &format!("-d{}", depth), root_path])
        .output()
        .expect("Erro ao executar comando du");

    spinner.finish_and_clear();

    let mut inaccessible_dirs: HashSet<String> = HashSet::new();

    if output.status.success() {
        let result = str::from_utf8(&output.stdout).expect("Erro ao converter saída");

        let mut lines: Vec<(&str, u64)> = result
            .lines()
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                let size = parts.next()?.parse::<u64>().ok()?;
                let path = parts.next()?;

                if line.contains("Operation not permitted") {
                    let top_dir = path.split('/').nth(1).unwrap_or("/").to_string();
                    inaccessible_dirs.insert(format!("/{}", top_dir));
                    None
                } else {
                    Some((path, size))
                }
            })
            .collect();

        lines.sort_by_key(|&(_, size)| size);
        lines.reverse();

        println!("{:<10} | {}", "Tamanho(MB)", "Caminho");
        println!("{}", "-".repeat(30));

        for (path, size) in lines {
            println!("{:<10} | {}", size, path);
        }

        if !inaccessible_dirs.is_empty() {
            println!("\n\x1b[33mDiretórios inacessíveis:\x1b[0m");
            for dir in inaccessible_dirs {
                println!("{}", dir);
            }
        }
    } else {
        let error_message = str::from_utf8(&output.stderr).unwrap_or("Erro desconhecido");
        eprintln!("\x1b[31mErro durante a execução do comando du:\x1b[0m\n{}", error_message);
    }
}
