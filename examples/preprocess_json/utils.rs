use env_logger::{Env, init_from_env};

pub fn init_logger() {
    let logger_env = Env::new()
        .filter_or("RUST_LOG", "gimura_preprocessor_lib=warn");

    init_from_env(logger_env);
}

pub fn format_text(text: String) -> String {
    text.lines().enumerate().map(|(line, x)| format!("{}| {}", line, x)).collect::<Vec<String>>().join("\n")
}