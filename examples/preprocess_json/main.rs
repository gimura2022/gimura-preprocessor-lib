use gimura_preprocessor_lib::prelude::*;

mod utils;

fn main() {
    utils::init_logger();

    let preprocessor_options = PreprocessorOptions::default();
    let mut preprocessor = Preporcessor::new(preprocessor_options);

    preprocessor.add_source("main".to_string(), CodeSource::from_path("examples/preprocess_json/config".to_string()));

    let preprocessed = preprocessor.preprocess("main".to_string(), "main.json".to_string());

    println!("{}", utils::format_text(preprocessed));
}