# Jack Compiler Frontend

Jack is a Java-like object-based simple programing language

- `jackc` transfer a `XXX.jack` program to a `XXX.my-token.xml` file, a `XXX.my-ast.xml` file and a `XXX.my-vm.vm` file.
- The `XXX.my-token.xml` file show output of the jack tokenizer.
- The `XXX.my-ast.xml` file show output of the jack parser.
- The `XXX.my-vm.vm` file is a simple stack-based VM language. The VM code will be translated to assembly code with Jack Compiler Backend [`vmtranslator`](https://github.com/cuppar/vmtranslator)

## Example

```bash
$ jackc Main.jack
```

### Souce code

Main.jack

```java
class Main {
   function void main() {
      var Array a; 
      var int length;
      var int i, sum;

      let length = Keyboard.readInt("How many numbers? ");
      let a = Array.new(length); // constructs the array
     
      let i = 0;
      while (i < length) {
         let a[i] = Keyboard.readInt("Enter a number: ");
         let sum = sum + a[i];
         let i = i + 1;
      }

      do Output.printString("The average is ");
      do Output.printInt(sum / length);
      return;
   }
}
```

### VM code

Main.my-vm.vm

```
function Main.main 4
push constant 18
call String.new 1
push constant 72
call String.appendChar 2
push constant 111
call String.appendChar 2
push constant 119
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 109
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 121
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 109
call String.appendChar 2
push constant 98
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 63
call String.appendChar 2
push constant 32
call String.appendChar 2
call Keyboard.readInt 1
pop local 1
push local 1
call Array.new 1
pop local 0
push constant 0
pop local 2
label while_start_1
push local 2
push local 1
lt
not
if-goto while_end_1
push local 0
push local 2
add
push constant 16
call String.new 1
push constant 69
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 116
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 110
call String.appendChar 2
push constant 117
call String.appendChar 2
push constant 109
call String.appendChar 2
push constant 98
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 58
call String.appendChar 2
push constant 32
call String.appendChar 2
call Keyboard.readInt 1
pop temp 0
pop pointer 1
push temp 0
pop that 0
push local 3
push local 0
push local 2
add
pop pointer 1
push that 0
add
pop local 3
push local 2
push constant 1
add
pop local 2
goto while_start_1
label while_end_1
push constant 15
call String.new 1
push constant 84
call String.appendChar 2
push constant 104
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 118
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 114
call String.appendChar 2
push constant 97
call String.appendChar 2
push constant 103
call String.appendChar 2
push constant 101
call String.appendChar 2
push constant 32
call String.appendChar 2
push constant 105
call String.appendChar 2
push constant 115
call String.appendChar 2
push constant 32
call String.appendChar 2
call Output.printString 1
push local 3
push local 1
call Math.divide 2
call Output.printInt 1
push constant 0
return
```

### Token

Main.my-token.xml

```
<tokens>
    <keyword>class</keyword>
    <identifier>Main</identifier>
    <symbol>{</symbol>
    <keyword>function</keyword>
    <keyword>void</keyword>
    <identifier>main</identifier>
    <symbol>(</symbol>
    <symbol>)</symbol>
    <symbol>{</symbol>
    <keyword>var</keyword>
    <identifier>Array</identifier>
    <identifier>a</identifier>
    <symbol>;</symbol>
    <keyword>var</keyword>
    <keyword>int</keyword>
    <identifier>length</identifier>
    <symbol>;</symbol>
    <keyword>var</keyword>
    <keyword>int</keyword>
    <identifier>i</identifier>
    <symbol>,</symbol>
    <identifier>sum</identifier>
    <symbol>;</symbol>
    <keyword>let</keyword>
    <identifier>length</identifier>
    <symbol>=</symbol>
    <identifier>Keyboard</identifier>
    <symbol>.</symbol>
    <identifier>readInt</identifier>
    <symbol>(</symbol>
    <stringConstant>How many numbers? </stringConstant>
    <symbol>)</symbol>
    <symbol>;</symbol>
    <keyword>let</keyword>
    <identifier>a</identifier>
    <symbol>=</symbol>
    <identifier>Array</identifier>
    <symbol>.</symbol>
    <identifier>new</identifier>
    <symbol>(</symbol>
    <identifier>length</identifier>
    <symbol>)</symbol>
    <symbol>;</symbol>
    <keyword>let</keyword>
    <identifier>i</identifier>
    <symbol>=</symbol>
    <integerConstant>0</integerConstant>
    <symbol>;</symbol>
    <keyword>while</keyword>
    <symbol>(</symbol>
    <identifier>i</identifier>
    <symbol>&lt;</symbol>
    <identifier>length</identifier>
    <symbol>)</symbol>
    <symbol>{</symbol>
    <keyword>let</keyword>
    <identifier>a</identifier>
    <symbol>[</symbol>
    <identifier>i</identifier>
    <symbol>]</symbol>
    <symbol>=</symbol>
    <identifier>Keyboard</identifier>
    <symbol>.</symbol>
    <identifier>readInt</identifier>
    <symbol>(</symbol>
    <stringConstant>Enter a number: </stringConstant>
    <symbol>)</symbol>
    <symbol>;</symbol>
    <keyword>let</keyword>
    <identifier>sum</identifier>
    <symbol>=</symbol>
    <identifier>sum</identifier>
    <symbol>+</symbol>
    <identifier>a</identifier>
    <symbol>[</symbol>
    <identifier>i</identifier>
    <symbol>]</symbol>
    <symbol>;</symbol>
    <keyword>let</keyword>
    <identifier>i</identifier>
    <symbol>=</symbol>
    <identifier>i</identifier>
    <symbol>+</symbol>
    <integerConstant>1</integerConstant>
    <symbol>;</symbol>
    <symbol>}</symbol>
    <keyword>do</keyword>
    <identifier>Output</identifier>
    <symbol>.</symbol>
    <identifier>printString</identifier>
    <symbol>(</symbol>
    <stringConstant>The average is </stringConstant>
    <symbol>)</symbol>
    <symbol>;</symbol>
    <keyword>do</keyword>
    <identifier>Output</identifier>
    <symbol>.</symbol>
    <identifier>printInt</identifier>
    <symbol>(</symbol>
    <identifier>sum</identifier>
    <symbol>/</symbol>
    <identifier>length</identifier>
    <symbol>)</symbol>
    <symbol>;</symbol>
    <keyword>return</keyword>
    <symbol>;</symbol>
    <symbol>}</symbol>
    <symbol>}</symbol>
</tokens>
```

### AST

Main.my-ast.xml

```
<class>
    <keyword>class</keyword>
    <identifier>Main<info>(name: Main, kind: None, type: None, index: None, usage: delcare
        className)</info></identifier>
    <symbol>{</symbol>
    <subroutineDec>
        <keyword>function</keyword>
        <keyword>void</keyword>
        <identifier>main<info>(name: main, kind: None, type: None, index: None, usage: delcare
            subroutineName in class, return type(void))</info></identifier>
        <symbol>(</symbol>
        <parameterList>
</parameterList>
        <symbol>)</symbol>
        <subroutineBody>
            <symbol>{</symbol>
            <varDec>
                <keyword>var</keyword>
                <identifier>Array<info>(name: Array, kind: None, type: None, index: None, usage: use
                    as a user define type)</info></identifier>
                <identifier>a<info>(name: a, kind: Some(Var), type: Some("Array"), index: Some(0),
                    usage: delcare varName in subroutine)</info></identifier>
                <symbol>;</symbol>
            </varDec>
            <varDec>
                <keyword>var</keyword>
                <keyword>int</keyword>
                <identifier>length<info>(name: length, kind: Some(Var), type: Some("int"), index:
                    Some(1), usage: delcare varName in subroutine)</info></identifier>
                <symbol>;</symbol>
            </varDec>
            <varDec>
                <keyword>var</keyword>
                <keyword>int</keyword>
                <identifier>i<info>(name: i, kind: Some(Var), type: Some("int"), index: Some(2),
                    usage: delcare varName in subroutine)</info></identifier>
                <symbol>,</symbol>
                <identifier>sum<info>(name: sum, kind: Some(Var), type: Some("int"), index: Some(3),
                    usage: delcare varName in subroutine)</info></identifier>
                <symbol>;</symbol>
            </varDec>
            <statements>
                <letStatement>
                    <keyword>let</keyword>
                    <identifier>length<info>(name: length, kind: Some(Var), type: Some("int"),
                        index: Some(1), usage: use in let statement)</info></identifier>
                    <symbol>=</symbol>
                    <expression>
                        <term>
                            <identifier>Keyboard<info>(name: Keyboard, kind: None, type: None,
                                index: None, usage: use in term varName|className|subroutineName)</info></identifier>
                            <symbol>.</symbol>
                            <identifier>readInt<info>(name: readInt, kind: None, type: None, index:
                                None, usage: use in xxx.subroutineName)</info></identifier>
                            <symbol>(</symbol>
                            <expressionList>
                                <expression>
                                    <term>
                                        <stringConstant>How many numbers? </stringConstant>
                                    </term>
                                </expression>
                            </expressionList>
                            <symbol>)</symbol>
                        </term>
                    </expression>
                    <symbol>;</symbol>
                </letStatement>
                <letStatement>
                    <keyword>let</keyword>
                    <identifier>a<info>(name: a, kind: Some(Var), type: Some("Array"), index:
                        Some(0), usage: use in let statement)</info></identifier>
                    <symbol>=</symbol>
                    <expression>
                        <term>
                            <identifier>Array<info>(name: Array, kind: None, type: None, index:
                                None, usage: use in term varName|className|subroutineName)</info></identifier>
                            <symbol>.</symbol>
                            <identifier>new<info>(name: new, kind: None, type: None, index: None,
                                usage: use in xxx.subroutineName)</info></identifier>
                            <symbol>(</symbol>
                            <expressionList>
                                <expression>
                                    <term>
                                        <identifier>length<info>(name: length, kind: Some(Var),
                                            type: Some("int"), index: Some(1), usage: use in term
                                            varName|className|subroutineName)</info></identifier>
                                    </term>
                                </expression>
                            </expressionList>
                            <symbol>)</symbol>
                        </term>
                    </expression>
                    <symbol>;</symbol>
                </letStatement>
                <letStatement>
                    <keyword>let</keyword>
                    <identifier>i<info>(name: i, kind: Some(Var), type: Some("int"), index: Some(2),
                        usage: use in let statement)</info></identifier>
                    <symbol>=</symbol>
                    <expression>
                        <term>
                            <integerConstant>0</integerConstant>
                        </term>
                    </expression>
                    <symbol>;</symbol>
                </letStatement>
                <whileStatement>
                    <keyword>while</keyword>
                    <symbol>(</symbol>
                    <expression>
                        <term>
                            <identifier>i<info>(name: i, kind: Some(Var), type: Some("int"), index:
                                Some(2), usage: use in term varName|className|subroutineName)</info></identifier>
                        </term>
                        <symbol>&lt;</symbol>
                        <expression>
                            <term>
                                <identifier>length<info>(name: length, kind: Some(Var), type:
                                    Some("int"), index: Some(1), usage: use in term
                                    varName|className|subroutineName)</info></identifier>
                            </term>
                        </expression>
                    </expression>
                    <symbol>)</symbol>
                    <symbol>{</symbol>
                    <statements>
                        <letStatement>
                            <keyword>let</keyword>
                            <identifier>a<info>(name: a, kind: Some(Var), type: Some("Array"),
                                index: Some(0), usage: use in let statement)</info></identifier>
                            <symbol>[</symbol>
                            <expression>
                                <term>
                                    <identifier>i<info>(name: i, kind: Some(Var), type: Some("int"),
                                        index: Some(2), usage: use in term
                                        varName|className|subroutineName)</info></identifier>
                                </term>
                            </expression>
                            <symbol>]</symbol>
                            <symbol>=</symbol>
                            <expression>
                                <term>
                                    <identifier>Keyboard<info>(name: Keyboard, kind: None, type:
                                        None, index: None, usage: use in term
                                        varName|className|subroutineName)</info></identifier>
                                    <symbol>.</symbol>
                                    <identifier>readInt<info>(name: readInt, kind: None, type: None,
                                        index: None, usage: use in xxx.subroutineName)</info></identifier>
                                    <symbol>(</symbol>
                                    <expressionList>
                                        <expression>
                                            <term>
                                                <stringConstant>Enter a number: </stringConstant>
                                            </term>
                                        </expression>
                                    </expressionList>
                                    <symbol>)</symbol>
                                </term>
                            </expression>
                            <symbol>;</symbol>
                        </letStatement>
                        <letStatement>
                            <keyword>let</keyword>
                            <identifier>sum<info>(name: sum, kind: Some(Var), type: Some("int"),
                                index: Some(3), usage: use in let statement)</info></identifier>
                            <symbol>=</symbol>
                            <expression>
                                <term>
                                    <identifier>sum<info>(name: sum, kind: Some(Var), type:
                                        Some("int"), index: Some(3), usage: use in term
                                        varName|className|subroutineName)</info></identifier>
                                </term>
                                <symbol>+</symbol>
                                <expression>
                                    <term>
                                        <identifier>a<info>(name: a, kind: Some(Var), type:
                                            Some("Array"), index: Some(0), usage: use in term
                                            varName|className|subroutineName)</info></identifier>
                                        <symbol>[</symbol>
                                        <expression>
                                            <term>
                                                <identifier>i<info>(name: i, kind: Some(Var), type:
                                                    Some("int"), index: Some(2), usage: use in term
                                                    varName|className|subroutineName)</info></identifier>
                                            </term>
                                        </expression>
                                        <symbol>]</symbol>
                                    </term>
                                </expression>
                            </expression>
                            <symbol>;</symbol>
                        </letStatement>
                        <letStatement>
                            <keyword>let</keyword>
                            <identifier>i<info>(name: i, kind: Some(Var), type: Some("int"), index:
                                Some(2), usage: use in let statement)</info></identifier>
                            <symbol>=</symbol>
                            <expression>
                                <term>
                                    <identifier>i<info>(name: i, kind: Some(Var), type: Some("int"),
                                        index: Some(2), usage: use in term
                                        varName|className|subroutineName)</info></identifier>
                                </term>
                                <symbol>+</symbol>
                                <expression>
                                    <term>
                                        <integerConstant>1</integerConstant>
                                    </term>
                                </expression>
                            </expression>
                            <symbol>;</symbol>
                        </letStatement>
                    </statements>
                    <symbol>}</symbol>
                </whileStatement>
                <doStatement>
                    <keyword>do</keyword>
                    <identifier>Output<info>(name: Output, kind: None, type: None, index: None,
                        usage: use as subroutineName or (className | varName) in a subroutine call)</info></identifier>
                    <symbol>.</symbol>
                    <identifier>printString<info>(name: printString, kind: None, type: None, index:
                        None, usage: use as a xxx.subroutineName in a subroutine call)</info></identifier>
                    <symbol>(</symbol>
                    <expressionList>
                        <expression>
                            <term>
                                <stringConstant>The average is </stringConstant>
                            </term>
                        </expression>
                    </expressionList>
                    <symbol>)</symbol>
                    <symbol>;</symbol>
                </doStatement>
                <doStatement>
                    <keyword>do</keyword>
                    <identifier>Output<info>(name: Output, kind: None, type: None, index: None,
                        usage: use as subroutineName or (className | varName) in a subroutine call)</info></identifier>
                    <symbol>.</symbol>
                    <identifier>printInt<info>(name: printInt, kind: None, type: None, index: None,
                        usage: use as a xxx.subroutineName in a subroutine call)</info></identifier>
                    <symbol>(</symbol>
                    <expressionList>
                        <expression>
                            <term>
                                <identifier>sum<info>(name: sum, kind: Some(Var), type: Some("int"),
                                    index: Some(3), usage: use in term
                                    varName|className|subroutineName)</info></identifier>
                            </term>
                            <symbol>/</symbol>
                            <expression>
                                <term>
                                    <identifier>length<info>(name: length, kind: Some(Var), type:
                                        Some("int"), index: Some(1), usage: use in term
                                        varName|className|subroutineName)</info></identifier>
                                </term>
                            </expression>
                        </expression>
                    </expressionList>
                    <symbol>)</symbol>
                    <symbol>;</symbol>
                </doStatement>
                <returnStatement>
                    <keyword>return</keyword>
                    <symbol>;</symbol>
                </returnStatement>
            </statements>
            <symbol>}</symbol>
        </subroutineBody>
    </subroutineDec>
    <symbol>}</symbol>
</class>
```

