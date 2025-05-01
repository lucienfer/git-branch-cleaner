use colored::Colorize;

pub fn info(msg: &str) {
    println!("[INFO] {}", msg);
}

pub fn warning(msg: &str) {
    println!("[{}] {}", "WARNING".yellow(), msg);
}

pub fn error_cli(msg: &str) {
    println!("[{}] {}", "ERROR".red(), msg);
}

pub fn success(msg: &str) {
    println!("[{}] {}", "SUCCESS".green(), msg);
}
