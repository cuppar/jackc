use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, Read},
    path::Path,
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenType {
    Keyword,
    Symbol,
    Identifier,
    IntConst,
    StringConst,
}

impl TokenType {
    fn token_type(token: &str) -> Self {
        use TokenType::*;
        if TokenType::is_keyword(token) {
            return Keyword;
        }
        if TokenType::is_symbol(token) {
            return Symbol;
        }
        if TokenType::is_identifier(token) {
            return Identifier;
        }
        if TokenType::is_int_const(token) {
            return IntConst;
        }
        if TokenType::is_string_const(token) {
            return StringConst;
        }

        panic!("<unknow token type>");
    }

    fn is_keyword(token: &str) -> bool {
        token == "class"
            || token == "constructor"
            || token == "function"
            || token == "method"
            || token == "field"
            || token == "static"
            || token == "var"
            || token == "int"
            || token == "char"
            || token == "boolean"
            || token == "void"
            || token == "true"
            || token == "false"
            || token == "null"
            || token == "this"
            || token == "let"
            || token == "do"
            || token == "if"
            || token == "else"
            || token == "while"
            || token == "return"
    }

    fn is_symbol(token: &str) -> bool {
        token == "{"
            || token == "}"
            || token == "("
            || token == ")"
            || token == "["
            || token == "]"
            || token == "."
            || token == ","
            || token == ";"
            || token == "+"
            || token == "-"
            || token == "*"
            || token == "/"
            || token == "&"
            || token == "|"
            || token == "<"
            || token == ">"
            || token == "="
            || token == "~"
    }

    fn is_int_const(token: &str) -> bool {
        if let Ok(_) = token.parse::<i32>() {
            return true;
        }
        false
    }

    fn is_string_const(token: &str) -> bool {
        for (i, ch) in token.chars().enumerate() {
            if i == 0 || i == token.len() - 1 {
                if ch != '"' {
                    return false;
                }
            } else {
                if ch == '"' || ch == '\n' {
                    return false;
                }
            }
        }
        true
    }

    fn is_identifier(token: &str) -> bool {
        let first = token.chars().nth(0).unwrap();
        first.is_ascii_lowercase() || first.is_ascii_uppercase() || first == '_'
    }
}

struct CharReader {
    next_char: Option<char>,
    reached_eof: bool,
    input_file_reader: BufReader<File>,
}
impl CharReader {
    fn new(file_path: &Path) -> io::Result<Self> {
        let input_file = OpenOptions::new().read(true).open(file_path)?;
        let input_file_reader = io::BufReader::new(input_file);
        let mut char_reader = Self {
            next_char: None,
            input_file_reader,
            reached_eof: false,
        };

        char_reader.read_next_char()?;

        Ok(char_reader)
    }

    fn has_more_char(&self) -> bool {
        !self.reached_eof
    }

    fn read_next_char(&mut self) -> io::Result<()> {
        let mut buf = [0; 1];
        let read_count = self.input_file_reader.read(&mut buf)?;
        if read_count == 0 {
            self.reached_eof = true;
            self.next_char = None;
        } else {
            self.next_char = Some(buf[0] as char)
        }
        Ok(())
    }
}

pub struct Tokenizer {
    next_token_type: Option<TokenType>,
    char_reader: CharReader,
    has_more_token: bool,
    symbol: char,
    identifier: String,
}
impl Tokenizer {
    pub fn new(file_path: &Path) -> io::Result<Self> {
        let char_reader = CharReader::new(file_path)?;
        let mut tokenizer = Self {
            char_reader,
            next_token_type: None,
            has_more_token: true,
            symbol: ' ',
            identifier: "".to_string(),
        };

        tokenizer.advance()?;

        Ok(tokenizer)
    }

    pub fn has_more_tokens(&self) -> bool {
        self.has_more_token
    }

    pub fn advance(&mut self) -> io::Result<()> {
        let mut ch;
        loop {
            if !self.char_reader.has_more_char() {
                self.has_more_token = false;
                self.next_token_type = None;
                return Ok(());
            }

            ch = self.char_reader.next_char.unwrap();

            // comments
            if ch == '/' {
                ch = self._get_next_ch()?.unwrap();
                // `//` comment
                if ch == '/' {
                    loop {
                        self.char_reader.read_next_char()?;
                        if !self.char_reader.has_more_char() {
                            // program end with `// ...(no newline)`
                            self.has_more_token = false;
                            self.next_token_type = None;
                            return Ok(());
                        }
                        ch = self.char_reader.next_char.unwrap();
                        if ch == '\n' {
                            self.char_reader.read_next_char()?;
                            break;
                        }
                    }
                }
                // `/* */` comment
                else if ch == '*' {
                    ch = self._get_next_ch()?.unwrap();
                    loop {
                        if ch == '*' {
                            ch = self._get_next_ch()?.unwrap();

                            if ch == '/' {
                                self.char_reader.read_next_char()?;
                                break;
                            }
                        } else {
                            ch = self._get_next_ch()?.unwrap();
                        }
                    }
                }
                // symbol `/`
                else {
                    self.next_token_type = Some(TokenType::Symbol);
                    self.symbol = '/';
                    return Ok(());
                }
            }
            // white spaces
            else if ch.is_whitespace() {
                loop {
                    let Some(ch) = self._get_next_ch()? else {
                        self.has_more_token = false;
                        self.next_token_type = None;
                        return Ok(());
                    };

                    if !ch.is_whitespace() {
                        break;
                    }
                }
            }
            // symbol
            else if TokenType::is_symbol(&ch.to_string()) {
                self.next_token_type = Some(TokenType::Symbol);
                self.char_reader.read_next_char()?;
                self.symbol = ch;
                return Ok(());
            }
            // identifier
            else if ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_' {
                self.identifier = ch.to_string();
                loop {
                    ch = self._get_next_ch()?.unwrap();
                    if !Tokenizer::_is_identifier_component(ch) {
                        // is keyword?
                        if TokenType::is_keyword(&self.identifier) {
                            self.next_token_type = Some(TokenType::Keyword);
                        } else {
                            self.next_token_type = Some(TokenType::Identifier);
                        }
                        return Ok(());
                    }
                    self.identifier += &ch.to_string();
                }
            }
            // int const
            else if ch.is_ascii_digit() {
                self.identifier = ch.to_string();
                loop {
                    ch = self._get_next_ch()?.unwrap();
                    if !ch.is_ascii_digit() {
                        self.next_token_type = Some(TokenType::IntConst);
                        return Ok(());
                    }
                    self.identifier += &ch.to_string();
                }
            }
            // string const
            else if ch == '"' {
                self.identifier = "".to_string();
                loop {
                    ch = self._get_next_ch()?.unwrap();
                    if ch == '\n' {
                        report_syntax_error("string shouldn't contains newline");
                    } else if ch == '"' {
                        self.next_token_type = Some(TokenType::StringConst);
                        self.char_reader.read_next_char()?; // comsume close `"`
                        return Ok(());
                    }
                    self.identifier += &ch.to_string();
                }
            }
            // syntax error
            else {
                report_syntax_error("unknow char");
            }
        }
    }

    pub fn token_type(&self) -> Option<TokenType> {
        self.next_token_type
    }

    pub fn symbol(&self) -> char {
        assert_eq!(self.token_type(), Some(TokenType::Symbol));
        self.symbol
    }

    pub fn identifier(&self) -> String {
        assert_eq!(self.token_type(), Some(TokenType::Identifier));
        self.identifier.clone()
    }

    pub fn keyword(&self) -> String {
        assert_eq!(self.token_type(), Some(TokenType::Keyword));
        self.identifier.clone()
    }

    pub fn int_const(&self) -> u32 {
        assert_eq!(self.token_type(), Some(TokenType::IntConst));
        self.identifier.parse::<u32>().unwrap()
    }

    pub fn string_const(&self) -> String {
        assert_eq!(self.token_type(), Some(TokenType::StringConst));
        self.identifier.clone()
    }

    fn _get_next_ch(&mut self) -> io::Result<Option<char>> {
        if !self.char_reader.has_more_char() {
            report_syntax_error("bad ending");
        }
        self.char_reader.read_next_char()?;
        Ok(self.char_reader.next_char)
    }

    fn _is_identifier_component(ch: char) -> bool {
        ch.is_ascii_uppercase() || ch.is_ascii_lowercase() || ch == '_' || ch.is_ascii_digit()
    }
}

fn report_syntax_error(msg: &str) {
    panic!("{}", msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_file::*;

    #[test]
    fn test_token_symbol() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("(")?;
        test_file.add_line("&")?;

        let mut tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::Symbol));
        assert_eq!(tokenizer.symbol(), '(');

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::Symbol));
        assert_eq!(tokenizer.symbol(), '&');

        tokenizer.advance()?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_token_keyword() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("class return")?;
        test_file.add_line("var")?;

        let mut tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::Keyword));
        assert_eq!(tokenizer.keyword(), "class");

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::Keyword));
        assert_eq!(tokenizer.keyword(), "return");

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::Keyword));
        assert_eq!(tokenizer.keyword(), "var");

        tokenizer.advance()?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_token_identifier() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("_abc123 Class")?;
        test_file.add_line("Va_32ab_423")?;

        let mut tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::Identifier));
        assert_eq!(tokenizer.identifier(), "_abc123");

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::Identifier));
        assert_eq!(tokenizer.identifier(), "Class");

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::Identifier));
        assert_eq!(tokenizer.identifier(), "Va_32ab_423");

        tokenizer.advance()?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_token_string_const() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line(r#""hello world""#)?;
        test_file.add_line(r#""class""123""#)?;

        let mut tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::StringConst));
        assert_eq!(tokenizer.string_const(), "hello world");

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::StringConst));
        assert_eq!(tokenizer.string_const(), "class");

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::StringConst));
        assert_eq!(tokenizer.string_const(), "123");

        tokenizer.advance()?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_token_int_const() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("123 00456")?;
        test_file.add_line("0789abc")?;

        let mut tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::IntConst));
        assert_eq!(tokenizer.int_const(), 123);

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::IntConst));
        assert_eq!(tokenizer.int_const(), 456);

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::IntConst));
        assert_eq!(tokenizer.int_const(), 789);

        tokenizer.advance()?;
        assert!(tokenizer.has_more_tokens());
        assert_eq!(tokenizer.token_type(), Some(TokenType::Identifier));
        assert_eq!(tokenizer.identifier(), "abc");

        tokenizer.advance()?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_has_more_token_empty() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;

        let tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_has_more_token_comments1() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("//")?;

        let tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_has_more_token_comments2() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("// comments")?;

        let tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_has_more_token_comments3() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("// comments 1")?;
        test_file.add_line("// comments 2")?;

        let tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_has_more_token_comments4() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("/**/")?;

        let tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_has_more_token_comments5() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("/***/")?;

        let tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_has_more_token_comments6() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("/** abc */")?;

        let tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_has_more_token_comments7() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("/** ab")?;
        test_file.add_line("c */")?;

        let tokenizer = Tokenizer::new(Path::new(&test_file.path))?;
        assert!(!tokenizer.has_more_tokens());

        Ok(())
    }

    #[test]
    fn test_read_next_char_empty() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;

        let char_reader = CharReader::new(Path::new(&test_file.path))?;

        assert_eq!(char_reader.next_char, None);
        assert!(!char_reader.has_more_char());

        Ok(())
    }

    #[test]
    fn test_read_next_char() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("hello")?;

        let mut char_reader = CharReader::new(Path::new(&test_file.path))?;

        assert_eq!(char_reader.next_char, Some('h'));

        char_reader.read_next_char()?;
        assert_eq!(char_reader.next_char, Some('e'));

        char_reader.read_next_char()?;
        assert_eq!(char_reader.next_char, Some('l'));

        char_reader.read_next_char()?;
        assert_eq!(char_reader.next_char, Some('l'));

        char_reader.read_next_char()?;
        assert_eq!(char_reader.next_char, Some('o'));
        assert!(char_reader.has_more_char());

        char_reader.read_next_char()?;
        assert_eq!(char_reader.next_char, Some('\n'));
        assert!(char_reader.has_more_char());

        char_reader.read_next_char()?;
        assert_eq!(char_reader.next_char, None);
        assert!(!char_reader.has_more_char());

        Ok(())
    }

    #[test]
    fn test_is_keyword() {
        assert!(TokenType::is_keyword("class"));
        assert!(TokenType::is_keyword("constructor"));
        assert!(TokenType::is_keyword("function"));
        assert!(TokenType::is_keyword("method"));
        assert!(TokenType::is_keyword("field"));
        assert!(TokenType::is_keyword("static"));
        assert!(TokenType::is_keyword("var"));
        assert!(TokenType::is_keyword("int"));
        assert!(TokenType::is_keyword("char"));
        assert!(TokenType::is_keyword("boolean"));
        assert!(TokenType::is_keyword("void"));
        assert!(TokenType::is_keyword("true"));
        assert!(TokenType::is_keyword("false"));
        assert!(TokenType::is_keyword("null"));
        assert!(TokenType::is_keyword("this"));
        assert!(TokenType::is_keyword("let"));
        assert!(TokenType::is_keyword("do"));
        assert!(TokenType::is_keyword("if"));
        assert!(TokenType::is_keyword("else"));
        assert!(TokenType::is_keyword("while"));
        assert!(TokenType::is_keyword("return"));
        assert!(!TokenType::is_keyword("Class"));
        assert!(!TokenType::is_keyword("123"));
        assert!(!TokenType::is_keyword(r#""string""#));
    }

    #[test]
    fn test_is_symbol() {
        assert!(TokenType::is_symbol("{"));
        assert!(TokenType::is_symbol("}"));
        assert!(TokenType::is_symbol("("));
        assert!(TokenType::is_symbol(")"));
        assert!(TokenType::is_symbol("["));
        assert!(TokenType::is_symbol("]"));
        assert!(TokenType::is_symbol("."));
        assert!(TokenType::is_symbol(","));
        assert!(TokenType::is_symbol(";"));
        assert!(TokenType::is_symbol("+"));
        assert!(TokenType::is_symbol("-"));
        assert!(TokenType::is_symbol("*"));
        assert!(TokenType::is_symbol("/"));
        assert!(TokenType::is_symbol("&"));
        assert!(TokenType::is_symbol("|"));
        assert!(TokenType::is_symbol("<"));
        assert!(TokenType::is_symbol(">"));
        assert!(TokenType::is_symbol("="));
        assert!(TokenType::is_symbol("~"));
        assert!(!TokenType::is_symbol("x"));
        assert!(!TokenType::is_symbol("%"));
    }

    #[test]
    fn test_is_int_const() {
        assert!(TokenType::is_int_const("123"));
        assert!(!TokenType::is_int_const("abc"));
    }

    #[test]
    fn test_is_string_const() {
        assert!(TokenType::is_string_const(r#""123""#));
        assert!(TokenType::is_string_const(r#""string""#));
        assert!(!TokenType::is_string_const("abc"));
    }

    #[test]
    fn test_is_identifier() {
        assert!(TokenType::is_identifier("xyz"));
        assert!(TokenType::is_identifier("a123"));
        assert!(TokenType::is_identifier("_123"));
        assert!(!TokenType::is_identifier("123a"));
    }

    #[test]
    fn test_token_type() {
        assert_eq!(TokenType::token_type("class"), TokenType::Keyword);
        assert_eq!(TokenType::token_type("("), TokenType::Symbol);
        assert_eq!(TokenType::token_type("abc123"), TokenType::Identifier);
        assert_eq!(TokenType::token_type("123"), TokenType::IntConst);
        assert_eq!(TokenType::token_type(r#""abc123""#), TokenType::StringConst);
    }
}
