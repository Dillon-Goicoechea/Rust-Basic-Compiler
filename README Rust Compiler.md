# Rust Programming Assignment: Square and Circle Lexical and Syntax Analysis


## Motivation
Rust is becoming a popular language. It was created to have high performance, reliability and productivity. The code is compiled and it claims to have advanced optimizations that produce stable and efficient programs. It has concurrent abilities, it provides memory safety without a runtime garbage collector.

This project consists in the development of the front end of a compiler. By programming the Lexical Analyzer (Scanner) and Syntax Analyzer (Parser) for the Square-Circle (SC) grammar you will gain further understanding of the lexical analysis and the production of tokens needed for the Syntax Analyzer (Parser), and how to consume those tokens by the Parser to verify that the syntax is correct.


## Description
Write a program in Rust that takes a program written in SC, and outputs:
1. If the program has lexical or syntax errors, the error that was found. Use panic version of error handling.
1. If the program is OK, depending on a command line flag the program will produce:
   1.	If the flag is `-s` the program will output a function call in Scheme that is going to be called by a program in Scheme that will execute the operations specified in the program.
   1. If the flag is `-p` the program will output a series of queries based in the operations specified in the program.

The program should run like this:
```
prompt>cargo run input.sc -s
; Processing Input File input.sc
; Lexical and Syntax analysis passed
(print-circle (makepoint 2 4) 3)
(print-square (makepoint 4 5) 4)
(contained-circle-square (makepoint 2 4) 3 (makepoint 4 5) 4)
prompt>
```

## Grammar

```
PROGRAM     -->   definitions: 
                     DEFS
                  operations:
                     OPERATIONS
                  end.
DEFS        -->   DEF | DEF; DEFS
DEF         -->   ID = point(NUM, NUM) |
                  ID = circle(ID, NUM) |
                  ID = square(ID, NUM)
OPERATIONS  -->   OPERATION | OPERATION; OPERATIONS
OPERATION   -->   print(ID) |
                  contained(ID, ID) |
                  intersects(ID, ID)
ID          -->   LETTER+
NUM         -->   DIGIT+
LETTER      -->   a | b | c | d | e | f | g | ... | z
NUM         -->   0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9
```

The tokens of this grammar are (some lexemes are examples):
| Token | Lexeme |
| ----- | ------ |
| `ID` | `alpha` |
| `NUM` |  `256` |
| `SEMICOLON` | `;` |
| `COLON` | `:` |
| `COMMA` | `,` |
| `PERIOD` | `.` |
| `LPAREN` | `(` |
| `RPAREN` | `)` |
| `ASSIGN` | `=` |
| `DEFINITIONS` | `definitions` |
| `OPERATIONS` | `operations` |
| `POINT` | `point` |
| `CIRCLE` | `circle` |
| `SQUARE` | `square` |
| `PRINT` | `print` |
| `CONTAINED` | `contained` |
| `INTERSECTS` | `intersects` |
| `END` | `end` |


Given the following program written in this language:
```
definitions:
   a = point(3,3);
   d = point(9,4);
   x = circle(a,1);
   y = square(d,2)
operations:
   print(x);
   print(y);
   intersects(x,y)
end.
```
The tokens that it would generate are:
1. `DEFINITIONS`
1. `COLON`
1. `ID a`
1. `ASSIGN`
1. `POINT`
1. `LPAREN`
1. `NUM 3`
1. `COMMA`
1. `NUM 3`
1. `RPAREN`
1. `SEMICOLON`
1. `ID d`
1. `ASSIGN`
1. `POINT`
1. `LPAREN`
1. `NUM 9`
1. `COMMA`
1. `NUM 4`
1. `RPAREN`
1. `SEMICOLON`
1. `ID x`
1. `ASSIGN`
1. `CIRCLE`
1. `LPAREN`
1. `ID a`
1. `COMMA`
1. `NUM 1`
1. `RPAREN`
1. `SEMICOLON`
1. `ID y`
1. `ASSIGN`
1. `SQUARE`
1. `LPAREN`
1. `ID d`
1. `COMMA`
1. `NUM 2`
1. `RPAREN`
1. `OPERATIONS`
1. `COLON`
1. `PRINT`
1. `LPAREN`
1. `ID x`
1. `RPAREN`
1. `SEMICOLON`
1. `PRINT`
1. `LPAREN`
1. `ID y`
1. `RPAREN`
1. `SEMICOLON`
1. `INTERSECTS`
1. `LPAREN`
1. `ID x`
1. `COMMA`
1. `ID y`
1. `RPAREN`
1. `END`
1. `PERIOD`

Notice that the ID and NUM tokens have their lexeme associated. Also notice that in the language the elements do not need to be separated by space, but they could.

## How to run the program

### Scheme Output
To generate scheme output you will add the `-s` flag at the end of the command:
```
prompt> cargo run input.sc -s
; processing input file input.sc
; Lexical and Syntax analysis passed
(print-circle (makepoint 2 4) 3)
(print-square (makepoint 4 5) 4)
(contained-circle-square (makepoint 2 4) 3 (makepoint 4 5) 4)
(intersects-circle-square (makepoint 2 4) 3 (makepoint 4 5) 4)

```
If the `contained` operation had two circles as parameter then the Scheme code would be: `(contained-circle-circle (makepoint 2 4) 3 (makepoint 1 1) 4)` For a circle with center (2,4) and radius 3, and another circle with center (1,1) and radius 4.

### Prolog Output
To generate prolog output you will add the `-p` flag at the end of the command:
```
prompt> cargo run input.sc -p
/* processing input file input.sc
   Lexical and Syntax analysis passed */
query(circle(point2d(2,4), 3)).
query(square(point2d(4,5), 4)).
query(contained(circle(point2d(2,4), 3), square(point2d(4,5), 4))).
query(intersects(circle(point2d(2,4), 3), square(point2d(4,5), 4))).
writeln(T) :- write(T), nl.
main:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),
      halt.

```

### Note about the Output
You are not expected to output the list of tokens. You can do it to check your work, but remember to remove them from the output before submitting your final version.

Later we will redirect the output to Scheme and Prolog programs respectively.

## Assignment Requirements
- Good programming practices
  - Indentation
  - Meaningful identifier naming
  - Consistent variable naming convention
  - Clean code, This means remove commented code that you are not using, and make sure that you are using appropriate and professional vocabulary in your variables and comments.
  - Commented code
- This activity is strictly individual

## Delivery
You will use this repository and commit and push to it. Remember to push your last version before the deadline.
What files should be in your repository:
- `src/main.rs` Source code in Rust for your lexical and syntax analysis
- `test0.sc`, `test1.sc`, `test2.sc`, `test3.sc`, `test4.sc`, `test5.sc` the test files provided for you to test. Notice that `test4.sc` has a lexical error and `test5.sc` has a syntax error.

## Assessment and Grading
Assessment will consider the following factors in the grading of this assignment:
-	Good programming practices
-	Your program will be tested with the five test programs that were provided and some others that will be made to test, some with lexical errors, some with syntax errors, some without any errors.
-	Adherence to instructions
-	Correct function of the program
-	No runtime errors (on any input!)
-	Late deliveries will have a zero mark
-	Plagiarism will have a double zero mark (in addition to losing 10% of your final grade, the person that plagiarizes will lose an additional 10% of their final grade), besides there will be a report filed in the students’ academic record.

## Extra Challenge

Create an additional file with a decorated (Tokens with Lexemes on the Leafs) parse tree. The file should be named like the input file but with the extension `.pt` (stands for parse tree), for instance if the input file is `test1.sc` the parse tree should be in file `test1.pt`.

## Additional Examples

Input File: `alpha-test.sc`
```
definitions:
   a = point(3,3);
   d = point(9,4);
   x = circle(a,1);
   y = square(d,2)
operations:
   print(x);
   print(y);
   intersects(x,y)
end.
```
Command: `cargo run alpha-test.sc -s`
Output:
```
; processing input file alpha-test.sc
; Lexical and Syntax analysis passed
(print (circle (makepoint 3 3) 1))
(print (square (makepoint 9 4) 2))
(intersects-circle-square (makepoint 3 3) 1 (makepoint 9 4) 2)
```
Command: `cargo run alpha-test.sc -p`
Output:
```
/* processing input file input.sc
   Lexical and Syntax analysis passed */
query(circle(point2d(3,3), 1)).
query(square(point2d(9,4), 2)).
query(intersects(circle(point2d(3,3), 1), square(point2d(9,4), 2))).
writeln(T) :- write(T), nl.
main:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),
      halt.
```

Input File: `beta-test.sc`
```
Definitions:
   a = point(3,3);
   d = point(9,4);
   x = circle(a,1);
   y = square(d,2)
operations:
   print(x);
   print(y);
   intersects(x,y)
end.
```
Command: `cargo run beta-test.sc -s` OR `cargo run beta-test.sc -s`
Output:
```
Lexical Error: D
```

Input File: `gamma-test.sc`
```
definitions:
   a = point(3,3);
   d = point(9,4);
   x = circle(a,1);
operations:
   y = square(d,2)
   print(x);
   print(y);
   intersects(x,y)
end.
```
Command: `cargo run gamma-test.sc -s` OR `cargo run gamma-test.sc -s`
Output:
```
Syntax Error: print, intersects or contained expected
```

**NOTE**
You just need to report the first error that you find (that is what panic version means), once you find a lexical error, report it and stop the program, if the program passes the lexical analysis, once it finds a syntax error, report it and stop the program. If the program passes both the lexical and syntax analyzers then proceed to generate the code.




## Academic Integrity
This programming assignment is to be done on an individual basis. At the same time, it is understood that learning from your peers is valid and you are encouraged to talk among yourselves about programming in general and current assignments in particular.  Keep in mind, however, that each individual student must do the work in order to learn.  Hence, the following guidelines are established:
- Feel free to discuss any and all programming assignments but do not allow other students to look at or copy your code. Do not give any student an electronic or printed copy of any program you write for this class.
- Gaining the ability to properly analyze common programming errors is an important experience. Do not deprive a fellow student of his/her opportunity to practice problem solving: control the urge to show them what to do by writing the code for them.
- If you’ve given the assignment a fair effort and still need help, see the instructor or a lab assistant.
- **If there is any evidence that a program or other written assignment was copied from another student, neither student will receive any credit for it. This rule will be enforced.**
- **If there is any evidence that a program or other written assignment was copied from any source, the student will not receive any credit for it. This rule will be enforced.**
- Protect yourself: Handle throw-away program listings carefully, keep your repository private.

Refer to the ECS Department Policy on Academic Integrity that is included in the class syllabus.
