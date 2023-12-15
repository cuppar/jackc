# Jack Compiler Frontend

Jack is a Java-like object-based simple programing language

- `jackc` transfer a `XXX.jack` program to a `XXX.my-token.xml` file, a `XXX.my-ast.xml` file and a `XXX.my-vm.vm` file.
- The `XXX.my-token.xml` file show output of the jack tokenizer.
- The `XXX.my-ast.xml` file show output of the jack parser.
- The `XXX.my-vm.vm` file is a simple stack-based VM language. The VM code will be translated to assembly code with Jack Compiler Backend [`vmtranslator`](https://github.com/cuppar/vmtranslator)

## Example

### Souce code

```java
class Main {
    
    /**
     * Initializes RAM[8001]..RAM[8016] to -1,
     * and converts the value in RAM[8000] to binary.
     */
    function void main() {
	    var int value;
        do Main.fillMemory(8001, 16, -1); // sets RAM[8001]..RAM[8016] to -1
        let value = Memory.peek(8000);    // reads a value from RAM[8000]
        do Main.convert(value);           // performs the conversion
        return;
    }
    
    /** Converts the given decimal value to binary, and puts 
     *  the resulting bits in RAM[8001]..RAM[8016]. */
    function void convert(int value) {
    	var int mask, position;
    	var boolean loop;
    	
    	let loop = true;
    	while (loop) {
    	    let position = position + 1;
    	    let mask = Main.nextMask(mask);
    	
    	    if (~(position > 16)) {
    	
    	        if (~((value & mask) = 0)) {
    	            do Memory.poke(8000 + position, 1);
       	        }
    	        else {
    	            do Memory.poke(8000 + position, 0);
      	        }    
    	    }
    	    else {
    	        let loop = false;
    	    }
    	}
    	return;
    }
 
    /** Returns the next mask (the mask that should follow the given mask). */
    function int nextMask(int mask) {
    	if (mask = 0) {
    	    return 1;
    	}
    	else {
	    return mask * 2;
    	}
    }
    
    /** Fills 'length' consecutive memory locations with 'value',
      * starting at 'startAddress'. */
    function void fillMemory(int startAddress, int length, int value) {
        while (length > 0) {
            do Memory.poke(startAddress, value);
            let length = length - 1;
            let startAddress = startAddress + 1;
        }
        return;
    }
}
```

### VM code

```
function Main.main 1
push constant 8001
push constant 16
push constant 1
neg
call Main.fillMemory 3
push constant 8000
call Memory.peek 1
pop local 0
push local 0
call Main.convert 1
push constant 0
return
function Main.convert 3
push constant 1
neg
pop local 2
label while_start_1
push local 2
not
if-goto while_end_1
push local 1
push constant 1
add
pop local 1
push local 0
call Main.nextMask 1
pop local 0
push local 1
push constant 16
gt
not
not
if-goto else_2
push argument 0
push local 0
and
push constant 0
eq
not
not
if-goto else_3
push constant 8000
push local 1
add
push constant 1
call Memory.poke 2
goto end_3
label else_3
push constant 8000
push local 1
add
push constant 0
call Memory.poke 2
label end_3
goto end_2
label else_2
push constant 0
pop local 2
label end_2
goto while_start_1
label while_end_1
push constant 0
return
function Main.nextMask 0
push argument 0
push constant 0
eq
not
if-goto else_1
push constant 1
return
goto end_1
label else_1
push argument 0
push constant 2
call Math.multiply 2
return
label end_1
function Main.fillMemory 0
label while_start_1
push argument 1
push constant 0
gt
not
if-goto while_end_1
push argument 0
push argument 2
call Memory.poke 2
push argument 1
push constant 1
sub
pop argument 1
push argument 0
push constant 1
add
pop argument 0
goto while_start_1
label while_end_1
push constant 0
return
```

### Token

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
<keyword>int</keyword>
<identifier>value</identifier>
<symbol>;</symbol>
<keyword>do</keyword>
<identifier>Main</identifier>
<symbol>.</symbol>
<identifier>fillMemory</identifier>
<symbol>(</symbol>
<integerConstant>8001</integerConstant>
<symbol>,</symbol>
<integerConstant>16</integerConstant>
<symbol>,</symbol>
<symbol>-</symbol>
<integerConstant>1</integerConstant>
<symbol>)</symbol>
<symbol>;</symbol>
<keyword>let</keyword>
<identifier>value</identifier>
<symbol>=</symbol>
<identifier>Memory</identifier>
<symbol>.</symbol>
<identifier>peek</identifier>
<symbol>(</symbol>
<integerConstant>8000</integerConstant>
<symbol>)</symbol>
<symbol>;</symbol>
<keyword>do</keyword>
<identifier>Main</identifier>
<symbol>.</symbol>
<identifier>convert</identifier>
<symbol>(</symbol>
<identifier>value</identifier>
<symbol>)</symbol>
<symbol>;</symbol>
<keyword>return</keyword>
<symbol>;</symbol>
<symbol>}</symbol>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>convert</identifier>
<symbol>(</symbol>
<keyword>int</keyword>
<identifier>value</identifier>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>mask</identifier>
<symbol>,</symbol>
<identifier>position</identifier>
<symbol>;</symbol>
<keyword>var</keyword>
<keyword>boolean</keyword>
<identifier>loop</identifier>
<symbol>;</symbol>
<keyword>let</keyword>
<identifier>loop</identifier>
<symbol>=</symbol>
<keyword>true</keyword>
<symbol>;</symbol>
<keyword>while</keyword>
<symbol>(</symbol>
<identifier>loop</identifier>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>let</keyword>
<identifier>position</identifier>
<symbol>=</symbol>
<identifier>position</identifier>
<symbol>+</symbol>
<integerConstant>1</integerConstant>
<symbol>;</symbol>
<keyword>let</keyword>
<identifier>mask</identifier>
<symbol>=</symbol>
<identifier>Main</identifier>
<symbol>.</symbol>
<identifier>nextMask</identifier>
<symbol>(</symbol>
<identifier>mask</identifier>
<symbol>)</symbol>
<symbol>;</symbol>
<keyword>if</keyword>
<symbol>(</symbol>
<symbol>~</symbol>
<symbol>(</symbol>
<identifier>position</identifier>
<symbol>&gt;</symbol>
<integerConstant>16</integerConstant>
<symbol>)</symbol>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>if</keyword>
<symbol>(</symbol>
<symbol>~</symbol>
<symbol>(</symbol>
<symbol>(</symbol>
<identifier>value</identifier>
<symbol>&amp;</symbol>
<identifier>mask</identifier>
<symbol>)</symbol>
<symbol>=</symbol>
<integerConstant>0</integerConstant>
<symbol>)</symbol>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>do</keyword>
<identifier>Memory</identifier>
<symbol>.</symbol>
<identifier>poke</identifier>
<symbol>(</symbol>
<integerConstant>8000</integerConstant>
<symbol>+</symbol>
<identifier>position</identifier>
<symbol>,</symbol>
<integerConstant>1</integerConstant>
<symbol>)</symbol>
<symbol>;</symbol>
<symbol>}</symbol>
<keyword>else</keyword>
<symbol>{</symbol>
<keyword>do</keyword>
<identifier>Memory</identifier>
<symbol>.</symbol>
<identifier>poke</identifier>
<symbol>(</symbol>
<integerConstant>8000</integerConstant>
<symbol>+</symbol>
<identifier>position</identifier>
<symbol>,</symbol>
<integerConstant>0</integerConstant>
<symbol>)</symbol>
<symbol>;</symbol>
<symbol>}</symbol>
<symbol>}</symbol>
<keyword>else</keyword>
<symbol>{</symbol>
<keyword>let</keyword>
<identifier>loop</identifier>
<symbol>=</symbol>
<keyword>false</keyword>
<symbol>;</symbol>
<symbol>}</symbol>
<symbol>}</symbol>
<keyword>return</keyword>
<symbol>;</symbol>
<symbol>}</symbol>
<keyword>function</keyword>
<keyword>int</keyword>
<identifier>nextMask</identifier>
<symbol>(</symbol>
<keyword>int</keyword>
<identifier>mask</identifier>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>if</keyword>
<symbol>(</symbol>
<identifier>mask</identifier>
<symbol>=</symbol>
<integerConstant>0</integerConstant>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>return</keyword>
<integerConstant>1</integerConstant>
<symbol>;</symbol>
<symbol>}</symbol>
<keyword>else</keyword>
<symbol>{</symbol>
<keyword>return</keyword>
<identifier>mask</identifier>
<symbol>*</symbol>
<integerConstant>2</integerConstant>
<symbol>;</symbol>
<symbol>}</symbol>
<symbol>}</symbol>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>fillMemory</identifier>
<symbol>(</symbol>
<keyword>int</keyword>
<identifier>startAddress</identifier>
<symbol>,</symbol>
<keyword>int</keyword>
<identifier>length</identifier>
<symbol>,</symbol>
<keyword>int</keyword>
<identifier>value</identifier>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>while</keyword>
<symbol>(</symbol>
<identifier>length</identifier>
<symbol>&gt;</symbol>
<integerConstant>0</integerConstant>
<symbol>)</symbol>
<symbol>{</symbol>
<keyword>do</keyword>
<identifier>Memory</identifier>
<symbol>.</symbol>
<identifier>poke</identifier>
<symbol>(</symbol>
<identifier>startAddress</identifier>
<symbol>,</symbol>
<identifier>value</identifier>
<symbol>)</symbol>
<symbol>;</symbol>
<keyword>let</keyword>
<identifier>length</identifier>
<symbol>=</symbol>
<identifier>length</identifier>
<symbol>-</symbol>
<integerConstant>1</integerConstant>
<symbol>;</symbol>
<keyword>let</keyword>
<identifier>startAddress</identifier>
<symbol>=</symbol>
<identifier>startAddress</identifier>
<symbol>+</symbol>
<integerConstant>1</integerConstant>
<symbol>;</symbol>
<symbol>}</symbol>
<keyword>return</keyword>
<symbol>;</symbol>
<symbol>}</symbol>
<symbol>}</symbol>
</tokens>
```

### AST

```
<class>
<keyword>class</keyword>
<identifier>Main<info>(name: Main, kind: None, type: None, index: None, usage: delcare className)</info></identifier>
<symbol>{</symbol>
<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>main<info>(name: main, kind: None, type: None, index: None, usage: delcare subroutineName in class, return type(void))</info></identifier>
<symbol>(</symbol>
<parameterList>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>value<info>(name: value, kind: Some(Var), type: Some("int"), index: Some(0), usage: delcare varName in subroutine)</info></identifier>
<symbol>;</symbol>
</varDec>
<statements>
<doStatement>
<keyword>do</keyword>
<identifier>Main<info>(name: Main, kind: None, type: None, index: None, usage: use as subroutineName or (className | varName) in a subroutine call)</info></identifier>
<symbol>.</symbol>
<identifier>fillMemory<info>(name: fillMemory, kind: None, type: None, index: None, usage: use as a xxx.subroutineName in a subroutine call)</info></identifier>
<symbol>(</symbol>
<expressionList>
<expression>
<term>
<integerConstant>8001</integerConstant>
</term>
</expression>
<symbol>,</symbol>
<expression>
<term>
<integerConstant>16</integerConstant>
</term>
</expression>
<symbol>,</symbol>
<expression>
<term>
<symbol>-</symbol>
<term>
<integerConstant>1</integerConstant>
</term>
</term>
</expression>
</expressionList>
<symbol>)</symbol>
<symbol>;</symbol>
</doStatement>
<letStatement>
<keyword>let</keyword>
<identifier>value<info>(name: value, kind: Some(Var), type: Some("int"), index: Some(0), usage: use in let statement)</info></identifier>
<symbol>=</symbol>
<expression>
<term>
<identifier>Memory<info>(name: Memory, kind: None, type: None, index: None, usage: use in term varName|className|subroutineName)</info></identifier>
<symbol>.</symbol>
<identifier>peek<info>(name: peek, kind: None, type: None, index: None, usage: use in xxx.subroutineName)</info></identifier>
<symbol>(</symbol>
<expressionList>
<expression>
<term>
<integerConstant>8000</integerConstant>
</term>
</expression>
</expressionList>
<symbol>)</symbol>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
<doStatement>
<keyword>do</keyword>
<identifier>Main<info>(name: Main, kind: None, type: None, index: None, usage: use as subroutineName or (className | varName) in a subroutine call)</info></identifier>
<symbol>.</symbol>
<identifier>convert<info>(name: convert, kind: None, type: None, index: None, usage: use as a xxx.subroutineName in a subroutine call)</info></identifier>
<symbol>(</symbol>
<expressionList>
<expression>
<term>
<identifier>value<info>(name: value, kind: Some(Var), type: Some("int"), index: Some(0), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
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
<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>convert<info>(name: convert, kind: None, type: None, index: None, usage: delcare subroutineName in class, return type(void))</info></identifier>
<symbol>(</symbol>
<parameterList>
<keyword>int</keyword>
<identifier>value<info>(name: value, kind: Some(Arg), type: Some("int"), index: Some(0), usage: delcare varName(arg) in parameterList)</info></identifier>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<varDec>
<keyword>var</keyword>
<keyword>int</keyword>
<identifier>mask<info>(name: mask, kind: Some(Var), type: Some("int"), index: Some(0), usage: delcare varName in subroutine)</info></identifier>
<symbol>,</symbol>
<identifier>position<info>(name: position, kind: Some(Var), type: Some("int"), index: Some(1), usage: delcare varName in subroutine)</info></identifier>
<symbol>;</symbol>
</varDec>
<varDec>
<keyword>var</keyword>
<keyword>boolean</keyword>
<identifier>loop<info>(name: loop, kind: Some(Var), type: Some("boolean"), index: Some(2), usage: delcare varName in subroutine)</info></identifier>
<symbol>;</symbol>
</varDec>
<statements>
<letStatement>
<keyword>let</keyword>
<identifier>loop<info>(name: loop, kind: Some(Var), type: Some("boolean"), index: Some(2), usage: use in let statement)</info></identifier>
<symbol>=</symbol>
<expression>
<term>
<keyword>true</keyword>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
<whileStatement>
<keyword>while</keyword>
<symbol>(</symbol>
<expression>
<term>
<identifier>loop<info>(name: loop, kind: Some(Var), type: Some("boolean"), index: Some(2), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
</expression>
<symbol>)</symbol>
<symbol>{</symbol>
<statements>
<letStatement>
<keyword>let</keyword>
<identifier>position<info>(name: position, kind: Some(Var), type: Some("int"), index: Some(1), usage: use in let statement)</info></identifier>
<symbol>=</symbol>
<expression>
<term>
<identifier>position<info>(name: position, kind: Some(Var), type: Some("int"), index: Some(1), usage: use in term varName|className|subroutineName)</info></identifier>
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
<letStatement>
<keyword>let</keyword>
<identifier>mask<info>(name: mask, kind: Some(Var), type: Some("int"), index: Some(0), usage: use in let statement)</info></identifier>
<symbol>=</symbol>
<expression>
<term>
<identifier>Main<info>(name: Main, kind: None, type: None, index: None, usage: use in term varName|className|subroutineName)</info></identifier>
<symbol>.</symbol>
<identifier>nextMask<info>(name: nextMask, kind: None, type: None, index: None, usage: use in xxx.subroutineName)</info></identifier>
<symbol>(</symbol>
<expressionList>
<expression>
<term>
<identifier>mask<info>(name: mask, kind: Some(Var), type: Some("int"), index: Some(0), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
</expression>
</expressionList>
<symbol>)</symbol>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
<ifStatement>
<keyword>if</keyword>
<symbol>(</symbol>
<expression>
<term>
<symbol>~</symbol>
<term>
<symbol>(</symbol>
<expression>
<term>
<identifier>position<info>(name: position, kind: Some(Var), type: Some("int"), index: Some(1), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
<symbol>&gt;</symbol>
<expression>
<term>
<integerConstant>16</integerConstant>
</term>
</expression>
</expression>
<symbol>)</symbol>
</term>
</term>
</expression>
<symbol>)</symbol>
<symbol>{</symbol>
<statements>
<ifStatement>
<keyword>if</keyword>
<symbol>(</symbol>
<expression>
<term>
<symbol>~</symbol>
<term>
<symbol>(</symbol>
<expression>
<term>
<symbol>(</symbol>
<expression>
<term>
<identifier>value<info>(name: value, kind: Some(Arg), type: Some("int"), index: Some(0), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
<symbol>&amp;</symbol>
<expression>
<term>
<identifier>mask<info>(name: mask, kind: Some(Var), type: Some("int"), index: Some(0), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
</expression>
</expression>
<symbol>)</symbol>
</term>
<symbol>=</symbol>
<expression>
<term>
<integerConstant>0</integerConstant>
</term>
</expression>
</expression>
<symbol>)</symbol>
</term>
</term>
</expression>
<symbol>)</symbol>
<symbol>{</symbol>
<statements>
<doStatement>
<keyword>do</keyword>
<identifier>Memory<info>(name: Memory, kind: None, type: None, index: None, usage: use as subroutineName or (className | varName) in a subroutine call)</info></identifier>
<symbol>.</symbol>
<identifier>poke<info>(name: poke, kind: None, type: None, index: None, usage: use as a xxx.subroutineName in a subroutine call)</info></identifier>
<symbol>(</symbol>
<expressionList>
<expression>
<term>
<integerConstant>8000</integerConstant>
</term>
<symbol>+</symbol>
<expression>
<term>
<identifier>position<info>(name: position, kind: Some(Var), type: Some("int"), index: Some(1), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
</expression>
</expression>
<symbol>,</symbol>
<expression>
<term>
<integerConstant>1</integerConstant>
</term>
</expression>
</expressionList>
<symbol>)</symbol>
<symbol>;</symbol>
</doStatement>
</statements>
<symbol>}</symbol>
<keyword>else</keyword>
<symbol>{</symbol>
<statements>
<doStatement>
<keyword>do</keyword>
<identifier>Memory<info>(name: Memory, kind: None, type: None, index: None, usage: use as subroutineName or (className | varName) in a subroutine call)</info></identifier>
<symbol>.</symbol>
<identifier>poke<info>(name: poke, kind: None, type: None, index: None, usage: use as a xxx.subroutineName in a subroutine call)</info></identifier>
<symbol>(</symbol>
<expressionList>
<expression>
<term>
<integerConstant>8000</integerConstant>
</term>
<symbol>+</symbol>
<expression>
<term>
<identifier>position<info>(name: position, kind: Some(Var), type: Some("int"), index: Some(1), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
</expression>
</expression>
<symbol>,</symbol>
<expression>
<term>
<integerConstant>0</integerConstant>
</term>
</expression>
</expressionList>
<symbol>)</symbol>
<symbol>;</symbol>
</doStatement>
</statements>
<symbol>}</symbol>
</ifStatement>
</statements>
<symbol>}</symbol>
<keyword>else</keyword>
<symbol>{</symbol>
<statements>
<letStatement>
<keyword>let</keyword>
<identifier>loop<info>(name: loop, kind: Some(Var), type: Some("boolean"), index: Some(2), usage: use in let statement)</info></identifier>
<symbol>=</symbol>
<expression>
<term>
<keyword>false</keyword>
</term>
</expression>
<symbol>;</symbol>
</letStatement>
</statements>
<symbol>}</symbol>
</ifStatement>
</statements>
<symbol>}</symbol>
</whileStatement>
<returnStatement>
<keyword>return</keyword>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<subroutineDec>
<keyword>function</keyword>
<keyword>int</keyword>
<identifier>nextMask<info>(name: nextMask, kind: None, type: None, index: None, usage: delcare subroutineName in class, return type(int))</info></identifier>
<symbol>(</symbol>
<parameterList>
<keyword>int</keyword>
<identifier>mask<info>(name: mask, kind: Some(Arg), type: Some("int"), index: Some(0), usage: delcare varName(arg) in parameterList)</info></identifier>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<ifStatement>
<keyword>if</keyword>
<symbol>(</symbol>
<expression>
<term>
<identifier>mask<info>(name: mask, kind: Some(Arg), type: Some("int"), index: Some(0), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
<symbol>=</symbol>
<expression>
<term>
<integerConstant>0</integerConstant>
</term>
</expression>
</expression>
<symbol>)</symbol>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<integerConstant>1</integerConstant>
</term>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
<keyword>else</keyword>
<symbol>{</symbol>
<statements>
<returnStatement>
<keyword>return</keyword>
<expression>
<term>
<identifier>mask<info>(name: mask, kind: Some(Arg), type: Some("int"), index: Some(0), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
<symbol>*</symbol>
<expression>
<term>
<integerConstant>2</integerConstant>
</term>
</expression>
</expression>
<symbol>;</symbol>
</returnStatement>
</statements>
<symbol>}</symbol>
</ifStatement>
</statements>
<symbol>}</symbol>
</subroutineBody>
</subroutineDec>
<subroutineDec>
<keyword>function</keyword>
<keyword>void</keyword>
<identifier>fillMemory<info>(name: fillMemory, kind: None, type: None, index: None, usage: delcare subroutineName in class, return type(void))</info></identifier>
<symbol>(</symbol>
<parameterList>
<keyword>int</keyword>
<identifier>startAddress<info>(name: startAddress, kind: Some(Arg), type: Some("int"), index: Some(0), usage: delcare varName(arg) in parameterList)</info></identifier>
<symbol>,</symbol>
<keyword>int</keyword>
<identifier>length<info>(name: length, kind: Some(Arg), type: Some("int"), index: Some(1), usage: delcare varName(arg) in parameterList)</info></identifier>
<symbol>,</symbol>
<keyword>int</keyword>
<identifier>value<info>(name: value, kind: Some(Arg), type: Some("int"), index: Some(2), usage: delcare varName(arg) in parameterList)</info></identifier>
</parameterList>
<symbol>)</symbol>
<subroutineBody>
<symbol>{</symbol>
<statements>
<whileStatement>
<keyword>while</keyword>
<symbol>(</symbol>
<expression>
<term>
<identifier>length<info>(name: length, kind: Some(Arg), type: Some("int"), index: Some(1), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
<symbol>&gt;</symbol>
<expression>
<term>
<integerConstant>0</integerConstant>
</term>
</expression>
</expression>
<symbol>)</symbol>
<symbol>{</symbol>
<statements>
<doStatement>
<keyword>do</keyword>
<identifier>Memory<info>(name: Memory, kind: None, type: None, index: None, usage: use as subroutineName or (className | varName) in a subroutine call)</info></identifier>
<symbol>.</symbol>
<identifier>poke<info>(name: poke, kind: None, type: None, index: None, usage: use as a xxx.subroutineName in a subroutine call)</info></identifier>
<symbol>(</symbol>
<expressionList>
<expression>
<term>
<identifier>startAddress<info>(name: startAddress, kind: Some(Arg), type: Some("int"), index: Some(0), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
</expression>
<symbol>,</symbol>
<expression>
<term>
<identifier>value<info>(name: value, kind: Some(Arg), type: Some("int"), index: Some(2), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
</expression>
</expressionList>
<symbol>)</symbol>
<symbol>;</symbol>
</doStatement>
<letStatement>
<keyword>let</keyword>
<identifier>length<info>(name: length, kind: Some(Arg), type: Some("int"), index: Some(1), usage: use in let statement)</info></identifier>
<symbol>=</symbol>
<expression>
<term>
<identifier>length<info>(name: length, kind: Some(Arg), type: Some("int"), index: Some(1), usage: use in term varName|className|subroutineName)</info></identifier>
</term>
<symbol>-</symbol>
<expression>
<term>
<integerConstant>1</integerConstant>
</term>
</expression>
</expression>
<symbol>;</symbol>
</letStatement>
<letStatement>
<keyword>let</keyword>
<identifier>startAddress<info>(name: startAddress, kind: Some(Arg), type: Some("int"), index: Some(0), usage: use in let statement)</info></identifier>
<symbol>=</symbol>
<expression>
<term>
<identifier>startAddress<info>(name: startAddress, kind: Some(Arg), type: Some("int"), index: Some(0), usage: use in term varName|className|subroutineName)</info></identifier>
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

