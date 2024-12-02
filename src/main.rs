mod actions;

use actions::{analyze::analyze_menu, clean::clean_menu, utils::check_sudo};
use inquire::Select;

fn main() {
    check_sudo();

    loop {
        let options = vec![
            "Análises",
            "Limpeza",
            "Sair",
        ];

        let selection = Select::new("Selecione uma categoria:", options)
            .with_help_message("[Use as setas ↑↓ para navegar e Enter para selecionar]")
            .prompt()
            .expect("Falha ao exibir o menu");

        match selection {
            "Análises" => analyze_menu(),
            "Limpeza" => clean_menu(),
            "Sair" => {
                println!("Saindo...");
                break;
            }
            _ => unreachable!(),
        }
    }
}
