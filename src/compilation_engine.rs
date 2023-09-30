use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

use crate::*;

pub struct CompilationEngine {
    output_ast_file: File,
    _output_ast_test_string: String,
    output_vm_file: File,
    _output_vm_test_string: String,
    tokenizer: Tokenizer,
}
impl CompilationEngine {
    pub fn new(filepath: &Path) -> io::Result<Self> {
        // output file settings
        let Some(filename) = filepath.file_name().unwrap().to_str() else {
            panic!();
        };
        let file_basename = &filename[..(filename.len() - (JACK_FILE_EXTENSION.len() + 1))];
        // ast file
        let output_ast_filename = file_basename.to_string() + "." + OUTPUT_AST_FILE_EXTENSION;
        let mut output_ast_file_path = filepath.parent().unwrap().to_path_buf();
        output_ast_file_path.extend([output_ast_filename].iter());
        let output_ast_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_ast_file_path)?;
        // vm file
        let output_vm_filename = file_basename.to_string() + "." + OUTPUT_VM_FILE_EXTENSION;
        let mut output_vm_file_path = filepath.parent().unwrap().to_path_buf();
        output_vm_file_path.extend([output_vm_filename].iter());
        let output_vm_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_vm_file_path)?;

        let tokenizer = Tokenizer::new(filepath)?;

        let engine = Self {
            output_ast_file,
            _output_ast_test_string: String::new(),
            output_vm_file,
            _output_vm_test_string: String::new(),
            tokenizer,
        };

        Ok(engine)
    }

    fn _get_next_token(&mut self) -> io::Result<()> {
        if !self.tokenizer.has_more_tokens() {
            report_syntax_error("need more token");
        }
        self.tokenizer.advance()?;
        Ok(())
    }

    pub fn compile_class(&mut self) -> io::Result<()> {
        // open class tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_CLASS))?;
        if self.tokenizer.token_type() != Some(TokenType::Keyword) {
            report_syntax_error("need a keyword");
        }
        if self.tokenizer.keyword() != "class" {
            report_syntax_error("need class");
        }
        self.print_to_ast(&format!("<{0}>{1}</{0}>\n", XML_TAG_KEYWORD, XML_TAG_CLASS))?;
        self._get_next_token()?; // eat `class`

        // className
        self._eat_identifier()?; // eat `className`

        // `{`
        self._eat_symbol()?;

        // classVarDec*
        while self.tokenizer.token_type() == Some(TokenType::Keyword)
            && (self.tokenizer.keyword() == "static" || self.tokenizer.keyword() == "field")
        {
            self.compile_class_var_dec()?;
        }

        // subroutineDec*
        while self.tokenizer.token_type() == Some(TokenType::Keyword)
            && (self.tokenizer.keyword() == "constructor"
                || self.tokenizer.keyword() == "function"
                || self.tokenizer.keyword() == "method")
        {
            self.compile_subroutine_dec()?;
        }

        // `}`
        self._eat_symbol()?;

        // close class tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_CLASS))?;
        Ok(())
    }

    pub fn compile_class_var_dec(&mut self) -> io::Result<()> {
        // open classVarDec tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_CLASS_VAR_DEC))?;

        // `static` or `field`
        self._eat_keyword()?;

        // type
        self.compile_type()?;

        // varName
        self._eat_identifier()?;

        // (`,` varName)*
        while self.tokenizer.token_type() == Some(TokenType::Symbol)
            && self.tokenizer.symbol() == ','
        {
            // `,`
            self._eat_symbol()?;

            // varName
            self._eat_identifier()?;
        }

        // `;`
        self._eat_symbol()?;

        // close classVarDec tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_CLASS_VAR_DEC))?;
        Ok(())
    }

    pub fn compile_subroutine_dec(&mut self) -> io::Result<()> {
        // open subroutineDec tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_SUBROUTINE_DEC))?;

        // `constructor` `function` or `method`
        self._eat_keyword()?;

        // void | type
        if self.tokenizer.token_type() == Some(TokenType::Keyword)
            && self.tokenizer.keyword() == "void"
        {
            // void
            self._eat_keyword()?;
        } else {
            // type
            self.compile_type()?;
        }

        // subroutineName
        self._eat_identifier()?;

        // `(`
        self._eat_symbol()?;

        // parameterList
        self.compile_parameter_list()?;

        // `)`
        self._eat_symbol()?;

        // subroutineBody
        self.compile_subroutine_body()?;

        // close subroutineDec tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_SUBROUTINE_DEC))?;

        Ok(())
    }

    pub fn compile_parameter_list(&mut self) -> io::Result<()> {
        // open parameterList tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_PARAMETER_LIST))?;

        // empty
        if self.tokenizer.token_type() != Some(TokenType::Symbol)
            || (self.tokenizer.token_type() == Some(TokenType::Symbol)
                && self.tokenizer.symbol() != ')')
        {
            // type
            self.compile_type()?;

            // varName
            self._eat_identifier()?;

            // (`,` type varName)*
            while self.tokenizer.token_type() == Some(TokenType::Symbol)
                && self.tokenizer.symbol() == ','
            {
                // `,`
                self._eat_symbol()?;

                // type
                self.compile_type()?;

                // varName
                self._eat_identifier()?;
            }
        }

        // close parameterList tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_PARAMETER_LIST))?;

        Ok(())
    }

    pub fn compile_subroutine_body(&mut self) -> io::Result<()> {
        // open subroutineBody tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_SUBROUTINE_BODY))?;

        // `{`
        self._eat_symbol()?;

        // varDec*
        while self.tokenizer.token_type() == Some(TokenType::Keyword)
            && self.tokenizer.keyword() == "var"
        {
            self.compile_var_dec()?;
        }

        // statements
        self.compile_statements()?;

        // `}`
        self._eat_symbol()?;

        // close subroutineBody tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_SUBROUTINE_BODY))?;
        Ok(())
    }

    pub fn compile_var_dec(&mut self) -> io::Result<()> {
        // open varDec tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_VAR_DEC))?;

        // `var`
        self._eat_keyword()?;

        // type
        self.compile_type()?;

        // varName (`,` varName)*
        loop {
            // varName
            self._eat_identifier()?;

            if self.tokenizer.token_type() != Some(TokenType::Symbol)
                || self.tokenizer.token_type() == Some(TokenType::Symbol)
                    && self.tokenizer.symbol() != ','
            {
                break;
            }

            // `,`
            self._eat_symbol()?;
        }

        // `;`
        self._eat_symbol()?;

        // close varDec tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_VAR_DEC))?;
        Ok(())
    }

    pub fn compile_statements(&mut self) -> io::Result<()> {
        // open statements tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_STATEMENTS))?;

        // statement*
        while self.tokenizer.token_type() == Some(TokenType::Keyword) {
            match self.tokenizer.keyword().as_str() {
                "let" => self.compile_let()?,
                "if" => self.compile_if()?,
                "while" => self.compile_while()?,
                "do" => self.compile_do()?,
                "return" => self.compile_return()?,
                _ => report_syntax_error("unknow statement"),
            }
        }

        // close statements tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_STATEMENTS))?;
        Ok(())
    }

    pub fn compile_let(&mut self) -> io::Result<()> {
        // open letStatement tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_STATEMENT_LET))?;

        // `let`
        self._eat_keyword()?;

        // varName
        self._eat_identifier()?;

        // `[`?
        if self.tokenizer.token_type() == Some(TokenType::Symbol) && self.tokenizer.symbol() == '['
        {
            // `[`
            self._eat_symbol()?;

            // expression
            self.compile_expression()?;

            // `]`
            self._eat_symbol()?;
        }

        // `=`
        self._eat_symbol()?;

        // expression
        self.compile_expression()?;

        // `;`
        self._eat_symbol()?;

        // close letStatement tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_STATEMENT_LET))?;
        Ok(())
    }

    pub fn compile_if(&mut self) -> io::Result<()> {
        // open ifStatement tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_STATEMENT_IF))?;

        // `if`
        self._eat_keyword()?;

        // `(`
        self._eat_symbol()?;

        // expression
        self.compile_expression()?;

        // `)`
        self._eat_symbol()?;

        // `{`
        self._eat_symbol()?;

        // statements
        self.compile_statements()?;

        // `}`
        self._eat_symbol()?;

        // optional `else`
        if self.tokenizer.token_type() == Some(TokenType::Keyword)
            && self.tokenizer.keyword() == "else"
        {
            // `else`
            self._eat_keyword()?;

            // `{`
            self._eat_symbol()?;

            // statements
            self.compile_statements()?;

            // `}`
            self._eat_symbol()?;
        }

        // close ifStatement tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_STATEMENT_IF))?;
        Ok(())
    }

    pub fn compile_while(&mut self) -> io::Result<()> {
        // open whileStatement tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_STATEMENT_WHILE))?;

        // `while`
        self._eat_keyword()?;

        // `(`
        self._eat_symbol()?;

        // expression
        self.compile_expression()?;

        // `)`
        self._eat_symbol()?;

        // `{`
        self._eat_symbol()?;

        // statements
        self.compile_statements()?;

        // `}`
        self._eat_symbol()?;

        // close whileStatement tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_STATEMENT_WHILE))?;
        Ok(())
    }

    pub fn compile_do(&mut self) -> io::Result<()> {
        // open doStatement tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_STATEMENT_DO))?;

        // `do`
        self._eat_keyword()?;

        // subroutineCall
        self.compile_subroutine_call()?;

        // `;`
        self._eat_symbol()?;

        // close doStatement tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_STATEMENT_DO))?;
        Ok(())
    }

    pub fn compile_return(&mut self) -> io::Result<()> {
        // open returnStatement tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_STATEMENT_RETURN))?;

        // `return`
        self._eat_keyword()?;

        // expression?
        if self.tokenizer.token_type() != Some(TokenType::Symbol)
            || self.tokenizer.token_type() == Some(TokenType::Symbol)
                && self.tokenizer.symbol() != ';'
        {
            self.compile_expression()?;
        }

        // `;`
        self._eat_symbol()?;

        // close returnStatement tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_STATEMENT_RETURN))?;
        Ok(())
    }

    pub fn compile_expression(&mut self) -> io::Result<()> {
        // open expression tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_EXPRESSION))?;

        // term (op term)*
        loop {
            // term
            self.compile_term()?;

            if self.tokenizer.token_type() != Some(TokenType::Symbol)
                || self.tokenizer.token_type() == Some(TokenType::Symbol)
                    && !CompilationEngine::_is_op(self.tokenizer.symbol())
            {
                break;
            }

            // op
            self._eat_symbol()?;
        }

        // close expression tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_EXPRESSION))?;
        Ok(())
    }

    pub fn compile_term(&mut self) -> io::Result<()> {
        // open term tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_TERM))?;

        // integer const
        if self.tokenizer.token_type() == Some(TokenType::IntConst) {
            self._eat_int_const()?;
        }
        // | string const
        else if self.tokenizer.token_type() == Some(TokenType::StringConst) {
            self._eat_string_const()?;
        }
        // | keyword const
        else if self.tokenizer.token_type() == Some(TokenType::Keyword)
            && (self.tokenizer.keyword() == "true"
                || self.tokenizer.keyword() == "false"
                || self.tokenizer.keyword() == "null"
                || self.tokenizer.keyword() == "this")
        {
            self._eat_keyword()?;
        }
        // | (expression)
        else if self.tokenizer.token_type() == Some(TokenType::Symbol)
            && self.tokenizer.symbol() == '('
        {
            // `(`
            self._eat_symbol()?;
            // expression
            self.compile_expression()?;
            // `)`
            self._eat_symbol()?;
        }
        // | unaryOp term
        else if self.tokenizer.token_type() == Some(TokenType::Symbol)
            && CompilationEngine::_is_unary_op(self.tokenizer.symbol())
        {
            // unaryOp
            self._eat_symbol()?;
            // term
            self.compile_term()?;
        }
        // look ahead two token
        else if self.tokenizer.token_type() == Some(TokenType::Identifier) {
            // first token: varName | className | subroutineName
            self._eat_identifier()?;

            // look ahead 2nd token
            // varName[expression]
            if self.tokenizer.token_type() == Some(TokenType::Symbol)
                && self.tokenizer.symbol() == '['
            {
                // `[`
                self._eat_symbol()?;

                // expression
                self.compile_expression()?;

                // `]`
                self._eat_symbol()?;
            }
            // subroutineName(expressionList)
            else if self.tokenizer.token_type() == Some(TokenType::Symbol)
                && self.tokenizer.symbol() == '('
            {
                // `(`
                self._eat_symbol()?;

                // expressionList
                self.compile_expression_list()?;

                // `)`
                self._eat_symbol()?;
            }
            // (className | varName).subroutineName(expressionList)
            else if self.tokenizer.token_type() == Some(TokenType::Symbol)
                && self.tokenizer.symbol() == '.'
            {
                // `.`
                self._eat_symbol()?;

                // subroutineName
                self._eat_identifier()?;

                // `(`
                self._eat_symbol()?;

                // expressionList
                self.compile_expression_list()?;

                // `)`
                self._eat_symbol()?;
            }
        } else {
            report_syntax_error("bad term");
        }

        // close term tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_TERM))?;
        Ok(())
    }

    pub fn compile_expression_list(&mut self) -> io::Result<()> {
        // open expressionList tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_EXPRESSION_LIST))?;

        // total optional
        if self.tokenizer.token_type() != Some(TokenType::Symbol)
            || self.tokenizer.token_type() == Some(TokenType::Symbol)
                && self.tokenizer.symbol() != ')'
        {
            // expression (`,` expression)*
            loop {
                // expression
                self.compile_expression()?;

                if self.tokenizer.token_type() != Some(TokenType::Symbol)
                    || self.tokenizer.token_type() == Some(TokenType::Symbol)
                        && self.tokenizer.symbol() != ','
                {
                    break;
                }

                // `,`
                self._eat_symbol()?;
            }
        }

        // close expressionList tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_EXPRESSION_LIST))?;
        Ok(())
    }

    fn compile_type(&mut self) -> io::Result<()> {
        if self.tokenizer.token_type() == Some(TokenType::Keyword) {
            // `int` `char` or `boolean`
            self._eat_keyword()?;
        } else if self.tokenizer.token_type() == Some(TokenType::Identifier) {
            // className
            self._eat_identifier()?;
        } else {
            report_syntax_error("");
        }
        Ok(())
    }

    fn compile_subroutine_call(&mut self) -> io::Result<()> {
        // subroutineName or (className | varName)
        self._eat_identifier()?;

        // look ahead 2nd token
        if self.tokenizer.symbol() == '(' {
            // `(`
            self._eat_symbol()?;

            // expressionList
            self.compile_expression_list()?;

            // `)`
            self._eat_symbol()?;
        } else if self.tokenizer.symbol() == '.' {
            // `.`
            self._eat_symbol()?;

            // subroutineName
            self._eat_identifier()?;

            // `(`
            self._eat_symbol()?;

            // expressionList
            self.compile_expression_list()?;

            // `)`
            self._eat_symbol()?;
        } else {
            report_syntax_error("syntax error")
        }
        Ok(())
    }

    fn print_to_ast(&mut self, s: &str) -> io::Result<()> {
        self._output_ast_test_string += s;
        self.output_ast_file.write_all(s.as_bytes())?;
        Ok(())
    }
    fn print_to_vm(&mut self, s: &str) -> io::Result<()> {
        self._output_vm_test_string += s;
        self.output_vm_file.write_all(s.as_bytes())?;
        Ok(())
    }

    fn _eat_keyword(&mut self) -> io::Result<()> {
        self.print_to_ast(&format!(
            "<{0}>{1}</{0}>\n",
            XML_TAG_KEYWORD,
            self.tokenizer.keyword()
        ))?;
        self._get_next_token()?;
        Ok(())
    }

    fn _eat_symbol(&mut self) -> io::Result<()> {
        let symbol = self.tokenizer.symbol();
        let mut symbol_str = symbol.to_string();

        if symbol == '<' {
            symbol_str = "&lt;".to_string();
        } else if symbol == '>' {
            symbol_str = "&gt;".to_string();
        } else if symbol == '&' {
            symbol_str = "&amp;".to_string();
        }

        self.print_to_ast(&format!("<{0}>{1}</{0}>\n", XML_TAG_SYMBOL, symbol_str))?;
        self._get_next_token()?;
        Ok(())
    }

    fn _eat_identifier(&mut self) -> io::Result<()> {
        self.print_to_ast(&format!(
            "<{0}>{1}</{0}>\n",
            XML_TAG_IDENTIFIER,
            self.tokenizer.identifier()
        ))?;
        self._get_next_token()?;

        Ok(())
    }

    fn _eat_int_const(&mut self) -> io::Result<()> {
        self.print_to_ast(&format!(
            "<{0}>{1}</{0}>\n",
            XML_TAG_INT_CONST,
            self.tokenizer.int_const()
        ))?;
        self._get_next_token()?;

        Ok(())
    }

    fn _eat_string_const(&mut self) -> io::Result<()> {
        self.print_to_ast(&format!(
            "<{0}>{1}</{0}>\n",
            XML_TAG_STRING_CONST,
            self.tokenizer.string_const()
        ))?;
        self._get_next_token()?;

        Ok(())
    }

    fn _is_op(op: char) -> bool {
        matches!(op, '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=')
    }

    fn _is_unary_op(op: char) -> bool {
        matches!(op, '-' | '~')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_file::*;

    #[test]
    fn test_compile_class_1() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("class Main {}")?;

        let mut engine = CompilationEngine::new(Path::new(&test_file.path))?;

        engine.compile_class()?;
        assert_eq!(
            engine._output_ast_test_string,
            "<class>
<keyword>class</keyword>
<identifier>Main</identifier>
<symbol>{</symbol>
<symbol>}</symbol>
</class>
"
        );

        Ok(())
    }

    #[test]
    fn test_compile_class_2() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("class Main { static boolean test; }")?;

        let mut engine = CompilationEngine::new(Path::new(&test_file.path))?;

        engine.compile_class()?;
        assert_eq!(
            engine._output_ast_test_string,
            "<class>
<keyword>class</keyword>
<identifier>Main</identifier>
<symbol>{</symbol>
<classVarDec>
<keyword>static</keyword>
<keyword>boolean</keyword>
<identifier>test</identifier>
<symbol>;</symbol>
</classVarDec>
<symbol>}</symbol>
</class>
"
        );

        Ok(())
    }

    #[test]
    fn test_compile_class_3() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line(
            "class Main { 
    static boolean test1;
    field boolean test2;
}",
        )?;

        let mut engine = CompilationEngine::new(Path::new(&test_file.path))?;

        engine.compile_class()?;
        assert_eq!(
            engine._output_ast_test_string,
            "<class>
<keyword>class</keyword>
<identifier>Main</identifier>
<symbol>{</symbol>
<classVarDec>
<keyword>static</keyword>
<keyword>boolean</keyword>
<identifier>test1</identifier>
<symbol>;</symbol>
</classVarDec>
<classVarDec>
<keyword>field</keyword>
<keyword>boolean</keyword>
<identifier>test2</identifier>
<symbol>;</symbol>
</classVarDec>
<symbol>}</symbol>
</class>
"
        );

        Ok(())
    }

    #[test]
    fn test_compile_class_var_dec() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("static boolean test;")?;

        let mut engine = CompilationEngine::new(Path::new(&test_file.path))?;
        engine.compile_class_var_dec()?;

        assert_eq!(
            engine._output_ast_test_string,
            "<classVarDec>
<keyword>static</keyword>
<keyword>boolean</keyword>
<identifier>test</identifier>
<symbol>;</symbol>
</classVarDec>
"
        );

        Ok(())
    }

    #[test]
    fn test_compile_parameter_list_1() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("int a, boolean b, char c)")?;

        let mut engine = CompilationEngine::new(Path::new(&test_file.path))?;
        engine.compile_parameter_list()?;

        assert_eq!(
            engine._output_ast_test_string,
            "<parameterList>
<keyword>int</keyword>
<identifier>a</identifier>
<symbol>,</symbol>
<keyword>boolean</keyword>
<identifier>b</identifier>
<symbol>,</symbol>
<keyword>char</keyword>
<identifier>c</identifier>
</parameterList>
"
        );

        Ok(())
    }

    #[test]
    fn test_compile_parameter_list_2() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line(")")?;

        let mut engine = CompilationEngine::new(Path::new(&test_file.path))?;
        engine.compile_parameter_list()?;

        assert_eq!(
            engine._output_ast_test_string,
            "<parameterList>
</parameterList>
"
        );

        Ok(())
    }

    #[test]
    fn test_compile_var_dec() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("var int game, game2;")?;

        let mut engine = CompilationEngine::new(Path::new(&test_file.path))?;
        engine.compile_var_dec()?;

        assert_eq!(
            engine._output_ast_test_string,
            "<varDec>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>game</identifier>
<symbol>,</symbol>
<identifier>game2</identifier>
<symbol>;</symbol>
</varDec>
"
        );

        Ok(())
    }

    #[test]
    fn test_compile_subroutine_body() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line(
            "{
    var SquareGame game;
    var int x,y;
    var char a,b,c;
}",
        )?;

        let mut engine = CompilationEngine::new(Path::new(&test_file.path))?;
        engine.compile_subroutine_body()?;

        assert_eq!(
            engine._output_ast_test_string,
            "<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<identifier>SquareGame</identifier>
<identifier>game</identifier>
<symbol>;</symbol>
</varDec>
<varDec>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>x</identifier>
<symbol>,</symbol>
<identifier>y</identifier>
<symbol>;</symbol>
</varDec>
<varDec>
<keyword>var</keyword>
<keyword>char</keyword>
<identifier>a</identifier>
<symbol>,</symbol>
<identifier>b</identifier>
<symbol>,</symbol>
<identifier>c</identifier>
<symbol>;</symbol>
</varDec>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
"
        );

        Ok(())
    }

    #[test]
    fn test_compile_subroutine_dec() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line(
            "function void main() {
    var SquareGame game;
}",
        )?;

        let mut engine = CompilationEngine::new(Path::new(&test_file.path))?;
        engine.compile_subroutine_dec()?;

        assert_eq!(
            engine._output_ast_test_string,
            "<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>main</identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<identifier>SquareGame</identifier>
<identifier>game</identifier>
<symbol>;</symbol>
</varDec>
<statements>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
"
        );

        Ok(())
    }
}
