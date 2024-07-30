# Gimura Preprocessor Lib
*A small library for file preprocessing.*

## Features
- commands from the C preprocessor (`include`, `define`, `ifdef`, ...)
- work with any files (shaders, configs, and other)

## Installing
Run `cargo add gimura-preprocessor-lib` or add to yours `Cargo.toml` line `gimura-preprocessor-lib = "0.1.0"`.

## Examples
Examples of using the library can be found in the [examples](/examples/) folder or by cloning the repository and running the command `cargo run --example example_name`.

At the moment there are the following examples:
- [`preprocess_glsl`](/examples/preprocess_glsl/main.rs) - simple example of preprocessing glsl shader.
- [`preprocess_json`](/examples/preprocess_json/main.rs) - a simple example of preprocessing a configuration in the JSON format (in fact, this is not a very good example since there are no comments in the JSON format and after preprocessing the file is considered that before it the editor's parser may complain)

I suggest we look at one example here:
```rust
use gimura_preprocessor_lib::prelude::*; // import prelude preprocessor

mod utils; // connection of the utility module, it is only needed to initialize the logger and to output files beautifully

fn main() {
    utils::init_logger(); // logger initialization

    let preprocessor_options = PreprocessorOptions::default(); // preprocessor options
    // setting the preprocessor command start operator, at the moment it is not required since the `//!` operator is set by default and since glsl uses the `//` comment operator, no replacement is needed
    // preprocessor_options.start_operator = ...

    let mut preprocessor = Preporcessor::new(preprocessor_options); // creating a preprocessor structure

    // adding libraries
    preprocessor.add_source("main".to_string(), CodeSource::from_path("examples/preprocess_glsl/shaders".to_string())); // adding main library

    let preprocessed = preprocessor.preprocess("main".to_string(), "main.glsl".to_string()); // preprocessing file `main.glsl` from `main` library

    println!("{}", utils::format_text(preprocessed)); // text output
}
```

You may have noticed that some "libraries" are mentioned in the code, this is described in detail in the next section.

## Preprocessor Concepts
This section describes the syntax and method for building a project using the preprocessor.

### Project organization
The minimum unit of code that a preprocessor can manage is a file, files can be reduced to libraries that can be added to the preprocessor structure in any quantity, this is necessary for the convenience of importing someone else's code.

For example, the project structure may look like this:
```
preprocessor-root
|- main library
|  |- main.glsl file
|  |- unils.glsl file
|- std library
|  |- rand.wgsl file
|  |- constant.wgsl file
```

### Command Syntax
In order for the preprocessor to be able to accept the command, the symbol specified in the `PreprocessorOptions` structure in the `start_operator` field must be at the beginning of the line; by default, this is `//!`.

Next should be the preprocessor command, basically the commands have the same syntax as in the C preprocessor, but below will be listed the commands that do not match it in syntax.

#### Include Command
Allows you to connect a file any file from any available library, below is an example of syntax:
```rust
//! include "lib-name" "file-name"
```

#### Define Command
The command allows you to declare a constant variable that can be used in code, the syntax is given below:
```rust
//! include VARIABLE_NAME "variable value"
```

<p align="center">
<br>
<snap>
2024 By _gimura_
</snap>
</p>