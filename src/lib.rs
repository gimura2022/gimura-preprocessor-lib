use std::{collections::HashMap, fs};
use log::*;

pub mod prelude {
    pub use super::PreprocessorOptions;
    pub use super::Preporcessor;
    pub use super::CodeSource;
}

#[derive(Debug, Clone)]
pub struct PreprocessorOptions {
    pub start_operator: String,
    pub defines: HashMap<String, String>,
}

impl Default for PreprocessorOptions {
    fn default() -> Self {
        Self {
            start_operator: "//!".to_string(),
            defines: HashMap::new(),
        }
    }
}

pub struct CodeSource {
    sources: HashMap<String, String>,
}

impl CodeSource {
    pub fn new(sources: HashMap<String, String>) -> Self {
        Self {
            sources,
        }
    }

    pub fn from_path(path: String) -> Self {
        let mut sources = HashMap::<String, String>::new();

        for entry in glob::glob(format!("{}/**/*", path).as_str()).unwrap() {
            let entry = &entry.unwrap();

            let name = entry.file_name().unwrap().to_str().unwrap().to_string().replace("/", "__");
            let source = fs::read_to_string(entry.as_path()).unwrap();

            sources.insert(name, source);
        }

        Self {
            sources
        }
    }

    pub fn get_source(&self, name: String) -> &String {
        self.sources.get(&name).unwrap()
    }
}

#[derive(Debug)]
pub enum Token {
    Command(CommandType),
    Literal(LiteralType),
    Separator(SeparatorType),

    OtherCode(String)
}

#[derive(Debug)]
pub enum SeparatorType {
    Colon
}

#[derive(Debug)]
pub enum CommandType {
    Include,
    Define,
    UnDefine,
    IfDef,
    IfNotDef,
    Endif,
    Error,
    Warn
}

#[derive(Debug)]
pub enum LiteralType {
    NameLiteral(String),
    StringLiteral(String),
}


pub struct Preporcessor {
    sources: HashMap<String, CodeSource>,
    preprocessor_options: PreprocessorOptions,

    defines: HashMap<String, String>,
    ifs: Vec<bool>
}

impl Preporcessor {
    pub fn new(preprocessor_options: PreprocessorOptions) -> Self {
        Self {
            preprocessor_options: preprocessor_options.clone(),
            sources: HashMap::new(),
            defines: preprocessor_options.defines,
            ifs: Vec::new(),
        }
    }

    pub fn add_source(&mut self, name: String, code_source: CodeSource) {
        self.sources.insert(name, code_source);
    }

    pub fn tokenize_line(&mut self, line: String) -> Vec<Token> {
        let mut tokens = Vec::new();

        let trimmed_line = line.trim_start().trim_end();
        let line_words = trimmed_line.split(" ");

        let mut other_code_buffer = "".to_string();
        let mut other_code = true;

        let mut str_literal_buffer = "".to_string();
        let mut str_literal = false;

        for word in line_words {
            if word == self.preprocessor_options.start_operator {
                other_code = false;
                tokens.push(Token::OtherCode(other_code_buffer.clone()));

                continue;
            }

            if other_code {
                other_code_buffer += &(word.to_owned() + " ");
                continue;
            }

            if str_literal {
                str_literal_buffer += word;
            }

            if word.starts_with('"') {
                str_literal_buffer = "".to_string();
                str_literal = true;

                str_literal_buffer += word;
            }

            if word.ends_with('"') {
                str_literal = false;
                tokens.push(Token::Literal(LiteralType::StringLiteral(str_literal_buffer.replace("\"", "").clone())));

                continue;
            }

            if str_literal {
                str_literal_buffer += " ";
                continue;
            }

            match word {
                "include" => tokens.push(Token::Command(CommandType::Include)),
                "define" => tokens.push(Token::Command(CommandType::Define)),
                "undef" => tokens.push(Token::Command(CommandType::UnDefine)),
                "ifdef" => tokens.push(Token::Command(CommandType::IfDef)),
                "ifndef" => tokens.push(Token::Command(CommandType::IfNotDef)),
                "endif" => tokens.push(Token::Command(CommandType::Endif)),
                "error" => tokens.push(Token::Command(CommandType::Error)),
                "warn" => tokens.push(Token::Command(CommandType::Warn)),

                _ => tokens.push(Token::Literal(LiteralType::NameLiteral(word.to_string())))
            }
        }

        if other_code {
            tokens.push(Token::OtherCode(other_code_buffer.clone()));
        }

        tokens
    }

    pub fn preprocess_line(&mut self, line: String) -> Vec<String> {
        let mut strings = Vec::new();
        let mut tokens = self.tokenize_line(line).into_iter().peekable();

        while tokens.peek().is_some() {
            let command = tokens.next().unwrap();

            match command {
                Token::Command(CommandType::IfDef) => {
                    let name = tokens.next().unwrap();

                    if let Token::Literal(LiteralType::NameLiteral(name)) = name {
                        self.ifs.push(self.defines.contains_key(&name));
                    }
                },
                Token::Command(CommandType::IfNotDef) => {
                    let name = tokens.next().unwrap();

                    if let Token::Literal(LiteralType::NameLiteral(name)) = name {
                        self.ifs.push(!self.defines.contains_key(&name));
                    }
                },
                Token::Command(CommandType::Endif) => {
                    self.ifs.pop();
                },

                _ => {}
            }

            if self.ifs.len() != 0 {
                if !self.ifs[self.ifs.len() - 1] {
                    continue;
                }
            }

            match command {
                Token::Command(command) => match command {
                    CommandType::Include => {
                        let lib = tokens.next().unwrap();
                        let name = tokens.next().unwrap();

                        if let Token::Literal(LiteralType::StringLiteral(lib)) = lib {
                            if let Token::Literal(LiteralType::StringLiteral(name)) = name {
                                let source = self.preprocess(lib, name);

                                strings.append(&mut source.lines().map(|x| x.to_string()).collect());
                            } else {
                                panic!("Unexpected token");
                            }
                        } else {
                            panic!("Unexpected token");
                        }
                    },
                    CommandType::Define => {
                        let name = tokens.next().unwrap();
                        let value = tokens.next().unwrap();

                        if let Token::Literal(LiteralType::NameLiteral(name)) = name {
                            if let Token::Literal(LiteralType::StringLiteral(value)) = value {
                                self.defines.insert(name, value);
                            } else {
                                panic!("Unexpected token");
                            }
                        } else {
                            panic!("Unexpected token");
                        }
                    },
                    CommandType::UnDefine => {
                        let name = tokens.next().unwrap();

                        if let Token::Literal(LiteralType::NameLiteral(name)) = name {
                            self.defines.remove(&name);
                        } else {
                            panic!("Unexpected token");
                        }
                    },
                    CommandType::Error => {
                        let messange = tokens.next().unwrap();

                        if let Token::Literal(LiteralType::StringLiteral(messange)) = messange {
                            error!("Error: {}", messange);
                            panic!("Error: {}", messange);
                        } else {
                            panic!("Unexpected token");
                        }
                    },
                    CommandType::Warn => {
                        let messange = tokens.next().unwrap();

                        if let Token::Literal(LiteralType::StringLiteral(messange)) = messange {
                            warn!("Warning: {}", messange);
                        } else {
                            panic!("Unexpected token");
                        }
                    },
                    _ => {},
                },
                Token::OtherCode(code) => strings.push(self.replace_defines(code)),

                _ => panic!("Exepted command found {:?}", command)
            }
        }

        strings
    }

    pub fn replace_defines(&self, string: String) -> String {
        let mut string = string;

        for define in self.defines.keys() {
            let define = self.defines.get_key_value(define).unwrap();
            string = string.replace(define.0, define.1);
        }

        string
    }

    pub fn preprocess(&mut self, lib: String, name: String) -> String {
        let mut out_sources = Vec::<String>::new();
        
        let main_namespace = self.sources.get(lib.as_str()).unwrap();
        let main_file = main_namespace.sources.get(name.as_str()).unwrap().clone();
        let main_file_lines = main_file.lines();

        for line in main_file_lines {
            for preprocessed_line in self.preprocess_line(line.to_string()) {
                if preprocessed_line.trim() == "" {
                    continue;
                }

                out_sources.push(preprocessed_line);
            }
        }

        out_sources.join("\n")
    }
}