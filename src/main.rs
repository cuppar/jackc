mod compilation_engine;
mod test_file;
mod tokenizer;
mod symbol_table;
mod vm_writer;

use std::{env::args, error::Error, ffi::OsStr, fs::OpenOptions, io::Write, path::Path, result};

use crate::{compilation_engine::CompilationEngine, tokenizer::*};

fn main() -> result::Result<(), Box<dyn Error>> {
    let Some(source) = args().nth(1) else {
        panic!("jackc need a `source` argument");
    };

    let path = Path::new(&source);

    if path.is_file() && path.extension() == Some(OsStr::new(JACK_FILE_EXTENSION)) {
        // process a .jack file
        compile(path)?;
    } else if path.is_dir() {
        // process a folder
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let child_path = entry.path();
            if child_path.is_file()
                && child_path.extension() == Some(OsStr::new(JACK_FILE_EXTENSION))
            {
                compile(&child_path)?;
            }
        }
    }

    Ok(())
}

fn compile(filepath: &Path) -> result::Result<(), Box<dyn Error>> {
    let Some(filename) = filepath.file_name().unwrap().to_str() else {
        panic!();
    };
    println!("complie {}", filename);

    // output file settings
    let file_basename = &filename[..(filename.len() - (JACK_FILE_EXTENSION.len() + 1))];
    let output_token_filename = file_basename.to_string() + "." + OUTPUT_TOKEN_FILE_EXTENSION;
    let mut output_token_file_path = filepath.parent().unwrap().to_path_buf();
    output_token_file_path.extend([output_token_filename].iter());

    // output to a token xml file
    let mut output_token_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(output_token_file_path)?;

    // construct xml file content
    let mut buf = String::new();
    buf += "<tokens>\n";

    // test tokenizer
    let mut tokenizer = Tokenizer::new(filepath)?;
    while tokenizer.has_more_tokens() {
        use TokenType::*;
        match tokenizer.token_type() {
            Some(Keyword) => {
                buf += &format!("<{}>", XML_TAG_KEYWORD);
                let keyword = tokenizer.keyword();
                buf += &keyword;
                buf += &format!("</{}>\n", XML_TAG_KEYWORD);
            }
            Some(Symbol) => {
                buf += &format!("<{}>", XML_TAG_SYMBOL);
                let symbol = tokenizer.symbol();
                let mut symbol_string = symbol.to_string();
                if symbol == '<' {
                    symbol_string = "&lt;".to_string();
                } else if symbol == '>' {
                    symbol_string = "&gt;".to_string();
                } else if symbol == '&' {
                    symbol_string = "&amp;".to_string();
                }
                buf += &symbol_string;

                buf += &format!("</{}>\n", XML_TAG_SYMBOL);
            }
            Some(IntConst) => {
                buf += &format!("<{}>", XML_TAG_INT_CONST);
                let int_const = tokenizer.int_const();
                buf += &int_const.to_string();
                buf += &format!("</{}>\n", XML_TAG_INT_CONST);
            }
            Some(StringConst) => {
                buf += &format!("<{}>", XML_TAG_STRING_CONST);
                let string_const = tokenizer.string_const();
                buf += &string_const;
                buf += &format!("</{}>\n", XML_TAG_STRING_CONST);
            }
            Some(Identifier) => {
                buf += &format!("<{}>", XML_TAG_IDENTIFIER);
                let identifier = tokenizer.identifier();
                buf += &identifier;
                buf += &format!("</{}>\n", XML_TAG_IDENTIFIER);
            }
            None => panic!(),
        }
        tokenizer.advance()?;
    }

    buf += r#"</tokens>"#;

    // write to xml file
    output_token_file.write_all(buf.as_bytes())?;

    // use compilation engine to genrate ast and vm file
    let mut engine = CompilationEngine::new(filepath)?;
    engine.compile_class()?;

    Ok(())
}

fn report_syntax_error(msg: &str) {
    panic!("{}", msg);
}

const JACK_FILE_EXTENSION: &str = "jack";
const OUTPUT_TOKEN_FILE_EXTENSION: &str = "my-token.xml";
const OUTPUT_AST_FILE_EXTENSION: &str = "my-ast.xml";
const OUTPUT_VM_FILE_EXTENSION: &str = "my-vm.vm";

// TOKEN
const XML_TAG_KEYWORD: &str = "keyword";
const XML_TAG_SYMBOL: &str = "symbol";
const XML_TAG_INT_CONST: &str = "integerConstant";
const XML_TAG_STRING_CONST: &str = "stringConstant";
const XML_TAG_IDENTIFIER: &str = "identifier";

// AST
const XML_TAG_CLASS: &str = "class";
const XML_TAG_CLASS_VAR_DEC: &str = "classVarDec";
const XML_TAG_SUBROUTINE_DEC: &str = "subroutineDec";
const XML_TAG_PARAMETER_LIST: &str = "parameterList";
const XML_TAG_SUBROUTINE_BODY: &str = "subroutineBody";
const XML_TAG_STATEMENTS: &str = "statements";
const XML_TAG_VAR_DEC: &str = "varDec";
const XML_TAG_EXPRESSION: &str = "expression";
const XML_TAG_STATEMENT_IF: &str = "ifStatement";
const XML_TAG_STATEMENT_LET: &str = "letStatement";
const XML_TAG_STATEMENT_WHILE: &str = "whileStatement";
const XML_TAG_STATEMENT_DO: &str = "doStatement";
const XML_TAG_STATEMENT_RETURN: &str = "returnStatement";
const XML_TAG_TERM: &str = "term";
const XML_TAG_EXPRESSION_LIST: &str = "expressionList";

