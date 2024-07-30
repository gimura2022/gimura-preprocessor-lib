use gimura_preprocessor_lib::prelude::*;

mod utils;

fn main() {
    utils::init_logger();

    let preprocessor_options = PreprocessorOptions::default();
    // preprocessor_options.start_operator = ... // preprocessor start operator
    
    let mut preprocessor = Preporcessor::new(preprocessor_options);

    preprocessor.add_source("main".to_string(), CodeSource::from_path("examples/preprocess_glsl/shaders".to_string()));

    let preprocessed = preprocessor.preprocess("main".to_string(), "main.glsl".to_string());

    println!("{}", utils::format_text(preprocessed));
}