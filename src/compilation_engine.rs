use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

use crate::{symbol_table::SymbolTable, vm_writer::VmWriter, *};

pub struct CompilationEngine {
    output_ast_file: File,
    _output_ast_test_string: String,
    tokenizer: Tokenizer,
    vm_writer: vm_writer::VmWriter,
    class_symbol_table: SymbolTable,
    subroutine_symbol_table: SymbolTable,
    class_name: String,
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

        let tokenizer = Tokenizer::new(filepath)?;

        let engine = Self {
            output_ast_file,
            _output_ast_test_string: String::new(),
            tokenizer,
            vm_writer: VmWriter::new(output_vm_file_path.as_path())?,
            class_symbol_table: SymbolTable::new(),
            subroutine_symbol_table: SymbolTable::new(),
            class_name: filename[0..(filename.len() - 5)].to_string(),
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
        self.class_name = self.tokenizer.identifier();
        self._eat_identifier("delcare className")?; // eat `className`

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
        let var_kind = match self.tokenizer.keyword().as_str() {
            "static" => symbol_table::Kind::Static,
            "field" => symbol_table::Kind::Field,
            _ => panic!(),
        };
        self._eat_keyword()?;

        // type
        let var_type = self.compile_type()?;

        // varName
        self.class_symbol_table
            .define(&self.tokenizer.identifier(), &var_type, var_kind);
        self._eat_identifier("delcare varName in class")?;

        // (`,` varName)*
        while self.tokenizer.token_type() == Some(TokenType::Symbol)
            && self.tokenizer.symbol() == ','
        {
            // `,`
            self._eat_symbol()?;

            // varName
            self.class_symbol_table
                .define(&self.tokenizer.identifier(), &var_type, var_kind);
            self._eat_identifier("delcare varName in class")?;
        }

        // `;`
        self._eat_symbol()?;

        // close classVarDec tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_CLASS_VAR_DEC))?;
        Ok(())
    }

    pub fn compile_subroutine_dec(&mut self) -> io::Result<()> {
        // reset subroutine symbol table
        self.subroutine_symbol_table.reset();
        // open subroutineDec tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_SUBROUTINE_DEC))?;

        // `constructor` `function` or `method`
        let subroutine_type = self.tokenizer.keyword();
        self._eat_keyword()?;

        // void | type
        let mut return_type = "void".to_string();
        if self.tokenizer.token_type() == Some(TokenType::Keyword)
            && self.tokenizer.keyword() == "void"
        {
            // void
            self._eat_keyword()?;
        } else {
            // type
            return_type = self.compile_type()?;
        }

        // subroutineName
        let subroutine_name = self.tokenizer.identifier();
        self._eat_identifier(&format!(
            "delcare subroutineName in class, return type({})",
            return_type
        ))?;

        // `(`
        self._eat_symbol()?;

        // parameterList
        self.compile_parameter_list()?;

        // `)`
        self._eat_symbol()?;

        // vm function info
        let vm_fn_name = format!("{}.{}", self.class_name, subroutine_name);

        // subroutineBody
        self.compile_subroutine_body(&vm_fn_name, &subroutine_type, &return_type)?;

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
            loop {
                // type
                let type_ = self.compile_type()?;

                // varName
                self.subroutine_symbol_table.define(
                    &self.tokenizer.identifier(),
                    &type_,
                    symbol_table::Kind::Arg,
                );
                self._eat_identifier("delcare varName(arg) in parameterList")?;

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

        // close parameterList tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_PARAMETER_LIST))?;

        Ok(())
    }

    pub fn compile_subroutine_body(
        &mut self,
        subroutine_name: &str,
        subroutine_type: &str,
        subroutine_return_type: &str,
    ) -> io::Result<()> {
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

        // code gen
        // function xxx.yyy nVars
        self.vm_writer.writeFunction(
            subroutine_name,
            self.subroutine_symbol_table
                .var_count(symbol_table::Kind::Var),
        )?;

        // statements
        self.compile_statements(0)?;

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
        let type_ = self.compile_type()?;

        // varName (`,` varName)*
        loop {
            // varName
            self.subroutine_symbol_table.define(
                &self.tokenizer.identifier(),
                &type_,
                symbol_table::Kind::Var,
            );
            self._eat_identifier("delcare varName in subroutine")?;

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

    pub fn compile_statements(&mut self, deep: i32) -> io::Result<()> {
        // open statements tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_STATEMENTS))?;

        // statement*
        while self.tokenizer.token_type() == Some(TokenType::Keyword) {
            match self.tokenizer.keyword().as_str() {
                "let" => self.compile_let()?,
                "if" => self.compile_if(deep)?,
                "while" => self.compile_while(deep)?,
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
        let left = self.tokenizer.identifier();
        self._eat_identifier("use in let statement")?;

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

        // code gen: let left = xxx;
        let mut segment = self.subroutine_symbol_table.kind_of(&left);
        let mut index = self.subroutine_symbol_table.index_of(&left);
        if segment == None {
            segment = self.class_symbol_table.kind_of(&left);
            index = self.class_symbol_table.index_of(&left);
        }

        if let Some(seg) = segment {
            let seg_str = match seg {
                symbol_table::Kind::Static => "static",
                symbol_table::Kind::Field => "this",
                symbol_table::Kind::Arg => "argument",
                symbol_table::Kind::Var => "local",
            };
            self.vm_writer.writePop(seg_str, index.unwrap())?;
        }

        // `;`
        self._eat_symbol()?;

        // close letStatement tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_STATEMENT_LET))?;
        Ok(())
    }

    pub fn compile_if(&mut self, deep: i32) -> io::Result<()> {
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

        // code gen
        self.vm_writer.writeArithmetic("not")?;
        let else_label = format!("else_{deep}");
        let end_label = format!("end_{deep}");
        self.vm_writer.writeIf(&else_label)?;

        // statements
        self.compile_statements(deep + 1)?;

        self.vm_writer.writeGoto(&end_label)?;
        self.vm_writer.writeLabel(&else_label)?;

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
            self.compile_statements(deep + 1)?;

            // `}`
            self._eat_symbol()?;
        }

        self.vm_writer.writeLabel(&end_label)?;

        // close ifStatement tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_STATEMENT_IF))?;
        Ok(())
    }

    pub fn compile_while(&mut self, deep: i32) -> io::Result<()> {
        // open whileStatement tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_STATEMENT_WHILE))?;

        // `while`
        self._eat_keyword()?;

        let while_start_label = format!("while_start_{deep}");
        self.vm_writer.writeLabel(&while_start_label)?;

        // `(`
        self._eat_symbol()?;

        // expression
        self.compile_expression()?;

        self.vm_writer.writeArithmetic("not")?;
        let end_label = format!("while_end_{deep}");
        self.vm_writer.writeIf(&end_label)?;

        // `)`
        self._eat_symbol()?;

        // `{`
        self._eat_symbol()?;

        // statements
        self.compile_statements(deep + 1)?;

        // `}`
        self._eat_symbol()?;

        self.vm_writer.writeGoto(&while_start_label)?;
        self.vm_writer.writeLabel(&end_label)?;

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
        } else {
            // return void
            self.vm_writer.writePush("constant", 0)?;
        }

        // code gen
        self.vm_writer.writeReturn()?;

        // `;`
        self._eat_symbol()?;

        // close returnStatement tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_STATEMENT_RETURN))?;
        Ok(())
    }

    pub fn compile_expression(&mut self) -> io::Result<()> {
        // open expression tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_EXPRESSION))?;

        // // term (op term)*
        // loop {
        //     // term
        //     self.compile_term()?;

        //     if self.tokenizer.token_type() != Some(TokenType::Symbol)
        //         || self.tokenizer.token_type() == Some(TokenType::Symbol)
        //             && !CompilationEngine::_is_op(self.tokenizer.symbol())
        //     {
        //         break;
        //     }

        //     // op
        //     self._eat_symbol()?;
        // }

        // left term
        self.compile_term()?;

        if self.tokenizer.token_type() == Some(TokenType::Symbol)
            && CompilationEngine::_is_op(self.tokenizer.symbol())
        {
            // op
            let op = self.tokenizer.symbol();
            self._eat_symbol()?;

            // right term
            self.compile_expression()?;

            // code gen
            match op {
                '+' => {
                    self.vm_writer.writeArithmetic("add")?;
                }
                '-' => {
                    self.vm_writer.writeArithmetic("sub")?;
                }
                '*' => {
                    self.vm_writer.writeCall("Math.multiply", 2)?;
                }
                '/' => {
                    self.vm_writer.writeCall("Math.divide", 2)?;
                }
                '&' => {
                    self.vm_writer.writeArithmetic("and")?;
                }
                '|' => {
                    self.vm_writer.writeArithmetic("or")?;
                }
                '<' => {
                    self.vm_writer.writeArithmetic("lt")?;
                }
                '>' => {
                    self.vm_writer.writeArithmetic("gt")?;
                }
                '=' => {
                    self.vm_writer.writeArithmetic("eq")?;
                }
                _ => panic!(),
            };
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
            let const_ = self.tokenizer.int_const();
            self.vm_writer.writePush("constant", const_ as i32)?;
            self._eat_int_const()?;
        }
        // | string const
        else if self.tokenizer.token_type() == Some(TokenType::StringConst) {
            let string = self.tokenizer.string_const();
            todo!();
            self._eat_string_const()?;
        }
        // | keyword const
        else if self.tokenizer.token_type() == Some(TokenType::Keyword)
            && (self.tokenizer.keyword() == "true"
                || self.tokenizer.keyword() == "false"
                || self.tokenizer.keyword() == "null"
                || self.tokenizer.keyword() == "this")
        {
            match self.tokenizer.keyword().as_str() {
                "true" => {
                    self.vm_writer.writePush("constant", 1)?;
                    self.vm_writer.writeArithmetic("neg")?;
                }
                "false" => {
                    self.vm_writer.writePush("constant", 0)?;
                }
                "null" => {
                    self.vm_writer.writePush("constant", 0)?;
                }
                "this" => {
                    self.vm_writer.writePush("pointer", 0)?;
                }
                _ => panic!(),
            }
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
            let op = self.tokenizer.symbol();
            self._eat_symbol()?;
            // term
            self.compile_term()?;
            // code gen
            match op {
                '-' => {
                    self.vm_writer.writeArithmetic("neg")?;
                }
                '~' => {
                    self.vm_writer.writeArithmetic("not")?;
                }
                _ => panic!(),
            }
        }
        // look ahead two token
        else if self.tokenizer.token_type() == Some(TokenType::Identifier) {
            // first token: varName | className | subroutineName
            let first_identifier = self.tokenizer.identifier();
            self._eat_identifier("use in term varName|className|subroutineName")?;

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
                let n_args = self.compile_expression_list()?;

                // code gen: fn()
                self.vm_writer.writeCall(&first_identifier, n_args)?;

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
                let second_identifier = self.tokenizer.identifier();
                self._eat_identifier("use in xxx.subroutineName")?;

                // `(`
                self._eat_symbol()?;

                // expressionList
                let n_args = self.compile_expression_list()?;

                let fn_name = format!("{}.{}", first_identifier, second_identifier);
                self.vm_writer.writeCall(&fn_name, n_args)?;
                
                // `)`
                self._eat_symbol()?;
            } else {
                // code gen
                let mut kind = self.subroutine_symbol_table.kind_of(&first_identifier);
                let mut index = self.subroutine_symbol_table.index_of(&first_identifier);
                let mut segment = "";

                if kind == None {
                    kind = self.class_symbol_table.kind_of(&first_identifier);
                    index = self.subroutine_symbol_table.index_of(&first_identifier);
                }

                if let Some(k) = kind {
                    segment = match k {
                        symbol_table::Kind::Static => "static",
                        symbol_table::Kind::Field => "this",
                        symbol_table::Kind::Arg => "argument",
                        symbol_table::Kind::Var => "local",
                    }
                }
                self.vm_writer.writePush(segment, index.unwrap())?;
            }
        } else {
            report_syntax_error("bad term");
        }

        // close term tag
        self.print_to_ast(&format!("</{}>\n", XML_TAG_TERM))?;
        Ok(())
    }

    pub fn compile_expression_list(&mut self) -> io::Result<i32> {
        // open expressionList tag
        self.print_to_ast(&format!("<{}>\n", XML_TAG_EXPRESSION_LIST))?;

        let mut count = 0;

        // total optional
        if self.tokenizer.token_type() != Some(TokenType::Symbol)
            || self.tokenizer.token_type() == Some(TokenType::Symbol)
                && self.tokenizer.symbol() != ')'
        {
            // expression (`,` expression)*
            loop {
                // expression
                self.compile_expression()?;
                count += 1;

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
        Ok(count)
    }

    fn compile_type(&mut self) -> io::Result<String> {
        let mut the_type = String::new();
        if self.tokenizer.token_type() == Some(TokenType::Keyword) {
            // `int` `char` or `boolean`
            the_type = self.tokenizer.keyword();
            self._eat_keyword()?;
        } else if self.tokenizer.token_type() == Some(TokenType::Identifier) {
            // className
            the_type = self.tokenizer.identifier();
            self._eat_identifier("use as a user define type")?;
        } else {
            report_syntax_error("");
        }
        Ok(the_type.to_string())
    }

    fn compile_subroutine_call(&mut self) -> io::Result<()> {
        // subroutineName or (className | varName)
        let first_identifier = self.tokenizer.identifier();
        self._eat_identifier(
            "use as subroutineName or (className | varName) in a subroutine call",
        )?;

        // look ahead 2nd token
        if self.tokenizer.symbol() == '(' {
            // `(`
            self._eat_symbol()?;

            // expressionList
            let n_args = self.compile_expression_list()?;

            // code gen
            self.vm_writer.writeCall(&first_identifier, n_args)?;

            // `)`
            self._eat_symbol()?;
        } else if self.tokenizer.symbol() == '.' {
            // `.`
            self._eat_symbol()?;

            // subroutineName
            let second_identifier = self.tokenizer.identifier();
            self._eat_identifier("use as a xxx.subroutineName in a subroutine call")?;

            // `(`
            self._eat_symbol()?;

            // expressionList
            let n_args = self.compile_expression_list()?;

            // code gen
            self.vm_writer.writeCall(
                &format!("{}.{}", first_identifier, second_identifier),
                n_args,
            )?;

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

    fn _eat_identifier(&mut self, usage: &str) -> io::Result<()> {
        let name = self.tokenizer.identifier();
        let mut kind = self.subroutine_symbol_table.kind_of(&name);
        if kind == None {
            kind = self.class_symbol_table.kind_of(&name);
        }
        let mut index = self.subroutine_symbol_table.index_of(&name);
        if index == None {
            index = self.class_symbol_table.index_of(&name);
        }
        let mut type_ = self.subroutine_symbol_table.type_of(&name);
        if type_ == None {
            type_ = self.class_symbol_table.type_of(&name);
        }

        self.print_to_ast(&format!(
            "<{0}>{1}<{2}>{3}</{2}></{0}>\n",
            XML_TAG_IDENTIFIER,
            self.tokenizer.identifier(),
            "info",
            format!(
                "(name: {name}, kind: {kind:?}, type: {type_:?}, index: {index:?}, usage: {usage})"
            )
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
        // engine.compile_subroutine_body()?;

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
