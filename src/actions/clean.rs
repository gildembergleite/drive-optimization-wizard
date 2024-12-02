use inquire::Select;
use std::process::Command;
use crate::actions::utils::confirm_action;

pub fn clean_menu() {
    let options = vec![
        "Limpar Docker",
        "Remover builds temporários do iOS",
        "Limpar cache de disco",
        "Limpar Lixeira",
        "Limpar logs do sistema",
        "Limpar cache de navegadores",
        "Limpar cache de atualizações do sistema",
        "Limpar arquivos temporários",
        "Executar todos os scripts",
        "Voltar",
    ];

    let selection = Select::new("Selecione uma limpeza:", options)
        .with_help_message("[Use as setas ↑↓ para navegar e Enter para selecionar]")
        .prompt()
        .expect("Falha ao exibir o menu");

    match selection {
        "Limpar Docker" => {
            if confirm_action("Deseja limpar o Docker?") {
                clean_docker();
            }
        }
        "Remover builds temporários do iOS" => {
            if confirm_action("Deseja remover builds temporários do iOS?") {
                clean_ios_builds();
            }
        }
        "Limpar cache de disco" => {
            if confirm_action("Deseja limpar o cache de disco?") {
                clean_disk_cache();
            }
        }
        "Limpar Lixeira" => {
            if confirm_action("Deseja limpar a lixeira?") {
                clean_trash();
            }
        }
        "Limpar logs do sistema" => {
            if confirm_action("Deseja limpar os logs do sistema?") {
                clean_system_logs();
            }
        }
        "Limpar cache de navegadores" => {
            if confirm_action("Deseja limpar o cache dos navegadores?") {
                clean_browser_cache();
            }
        }
        "Limpar cache de atualizações do sistema" => {
            if confirm_action("Deseja limpar o cache de atualizações do sistema?") {
                clean_system_updates_cache();
            }
        }
        "Limpar arquivos temporários" => {
            if confirm_action("Deseja limpar arquivos temporários?") {
                clean_temp_files();
            }
        }
        "Executar todos os scripts" => {
            if confirm_action("Deseja executar todos os scripts de limpeza?") {
                run_all_cleanups();
            }
        }
        "Voltar" => return,
        _ => unreachable!(),
    }
}

pub fn clean_docker() {
    println!("\x1b[33mLimpando containers e imagens Docker...\x1b[0m");
    Command::new("docker")
        .args(&["system", "prune", "-af"])
        .status()
        .expect("Falha ao executar o comando Docker");
    println!("\x1b[32mDocker limpo com sucesso!\x1b[0m");
}

pub fn clean_ios_builds() {
    println!("\x1b[33mRemovendo builds temporários do iOS...\x1b[0m");
    Command::new("rm")
        .args(&["-rf", "~/Library/Developer/Xcode/DerivedData"])
        .status()
        .expect("Falha ao limpar builds do iOS");
    println!("\x1b[32mBuilds temporários removidos com sucesso!\x1b[0m");
}

pub fn clean_disk_cache() {
    println!("\x1b[33mLimpando cache de disco...\x1b[0m");
    Command::new("rm")
        .args(&["-rf", "~/Library/Caches"])
        .status()
        .expect("Falha ao limpar cache de disco");
    println!("\x1b[32mCache de disco limpo com sucesso!\x1b[0m");
}

pub fn clean_trash() {
    println!("\x1b[33mLimpando lixeira...\x1b[0m");
    Command::new("rm")
        .args(&["-rf", "~/.Trash/*"])
        .status()
        .expect("Falha ao limpar a lixeira");
    println!("\x1b[32mLixeira limpa com sucesso!\x1b[0m");
}

pub fn clean_system_logs() {
    println!("\x1b[33mLimpando logs do sistema...\x1b[0m");
    Command::new("sudo")
        .args(&["rm", "-rf", "/var/log/*"])
        .status()
        .expect("Falha ao limpar logs do sistema");
    println!("\x1b[32mLogs do sistema limpos com sucesso!\x1b[0m");
}

pub fn clean_browser_cache() {
    println!("\x1b[33mLimpando cache de navegadores...\x1b[0m");
    Command::new("rm")
        .args(&["-rf", "~/Library/Caches/com.apple.Safari"])
        .status()
        .expect("Falha ao limpar cache do Safari");
    Command::new("rm")
        .args(&["-rf", "~/Library/Application\\ Support/Google/Chrome/Default/Cache"])
        .status()
        .expect("Falha ao limpar cache do Chrome");
    println!("\x1b[32mCache de navegadores limpo com sucesso!\x1b[0m");
}

pub fn clean_system_updates_cache() {
    println!("\x1b[33mLimpando cache de atualizações do sistema...\x1b[0m");
    Command::new("sudo")
        .args(&["rm", "-rf", "/Library/Updates/*"])
        .status()
        .expect("Falha ao limpar cache de atualizações do sistema");
    println!("\x1b[32mCache de atualizações do sistema limpo com sucesso!\x1b[0m");
}

pub fn clean_temp_files() {
    println!("\x1b[33mLimpando arquivos temporários...\x1b[0m");
    Command::new("sudo")
        .args(&["rm", "-rf", "/private/var/tmp/*"])
        .status()
        .expect("Falha ao limpar arquivos temporários");
    println!("\x1b[32mArquivos temporários limpos com sucesso!\x1b[0m");
}

pub fn run_all_cleanups() {
    println!("\x1b[33mExecutando todos os scripts de limpeza...\x1b[0m");

    clean_docker();
    clean_ios_builds();
    clean_disk_cache();
    clean_trash();
    clean_system_logs();
    clean_browser_cache();
    clean_system_updates_cache();
    clean_temp_files();

    println!("\x1b[32mTodos os scripts de limpeza foram executados com sucesso!\x1b[0m");
}
