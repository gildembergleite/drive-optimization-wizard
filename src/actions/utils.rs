use inquire::Confirm;

pub fn check_sudo() {
    if std::env::var("USER").unwrap_or_default() != "root" {
        eprintln!("\x1b[31mErro: Esta operação requer permissões de administrador. Execute com 'sudo'.\x1b[0m");
        std::process::exit(1);
    }
}

pub fn confirm_action(question: &str) -> bool {
    Confirm::new(question)
        .with_default(false)
        .prompt()
        .unwrap_or(false)
}
