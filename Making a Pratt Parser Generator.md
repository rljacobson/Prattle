# Making a Pratt Parser Generator

## A brief history of the Pratt parsing algorithm
The history of programming language parsers is dominated by the thorny challenge of parsing expressions, mathematical expressions in particular, taking into account the pecedence of operators in the expressions. Modern formal language theory began with the work of Noam Chomsky in the 1950s, in which Chomsky lays out a mathematical framework for linguistics. Under this mathematical framework, languages exist within a heirarchy of langauges defined according to how difficult the language is to parse.^[1] But computer programmers needed practical, efficient algorithms to parse computer programs for translation to machine code. Parsers of the 1950s relied on ad hoc logic rather than systematic algorithms (a feature which persists to this day, though to a much lesser degree). The 1960s was a golden age of parsing algorithm research when nearly all of the concepts and algorithms we use today were discovered and rigorously studied. By the early 1970s, parsing theory had evolved to the point that  Stephen C. Johnson, a computer scientist at Bell Labs / AT&T, was able to start work on YACC (now "Yacc"), "Yet Another Compiler Compiler."^[2] YACC was first publically described in 1975 and shipped with Unix version 3^[3]  and is still in use today.

The thorny challenge of parsing expressions was partially solved in 1961 by the venerable shunting-yard algorithm described by Dutch computer scientist Edsger W. Dijkstra, which algorithm could efficiently parse binary infix operator expressions with a value stack and an operator stack, creating nodes from the bottom up. Vaughan R. Pratt generalized Dijkstra's sunting-yard algorithm to parsing of entire languages, this time using a single stack, or using recursive descent with the call stack as an implicit stack, creating nodes from the top down. Pratt's parsing algorithm overcomes a number of limitations with the shunting-yard algorithm and is simpler.

Precedence climbing was apparently first invented by Martin Richards in 1979^[4] for his BCPL compiler. Precedence climbing uses a single recursive function and a single table mapping token IDs to their precedence instead of Pratt's mutual recursive descent and multiple tables. In fact, precedence climbing can be seen as a special case of Pratt parsing, though historically they have been understood as related but not identical.^[5,6]  

Vaughan Pratt had described his algorithm six years earlier in 1973 at the very first meeting of POPL, the Symposium on Principles of Programming Languages, which remains among the most important conferences in the field. It is interesting to see what other papers are published in the 1973 POPL Proceedings. One finds, for example, Aho, S. C. Johnson, and J. D. Ullman's "Deterministic parsing of ambiguous grammars,“^[8] and James H. Morris, Jr.’s “Types are not sets,”^[9] among papers by several other influential luminaries. Vaughan Pratt had been developing an alternative expression syntax for MACLISP called CGOL,^[10] which he needed to parse. 

## Parser design
### The typical design

There are already many articles on the web describing the Pratt parsing algorithm. (I recommend [5].) If you are not familiar with the algorithm, go read up on it before returning here.

A typical object oriented design is to have a node class for each kind of AST node, each class implementing their own "parselet" method, traditionally named `led`  for "left denotation" after Pratt's original article, that is called by a driver algorithm and is responsible for parsing the node instance's operands (children) by calling back into the driver before returning. Each class also keeps track of its associativity and precedence. The driver algorithm consumes a token, looks up the appropriate class in a table, creates an instance and calls its parslet method.

We can be a little bit more efficient by having only a handful of superclasses corresponding to each required (affix, associativity) combination. In the typical object-oriented Pratt-parser design, every operator would need a subclass of the form

```c++
class Multiply: public InfixLeftAssoc{
  Multiply(Parser parser, ASTNode left, Token operator): 
  	precedence(40){
		super(parser, left, operator);
  }
  
  T MultiplyMethodA(U param1, V param2){...}
  W MultiplyMethodB(X param1, Y param2){...}
  // etc.
}
```

This class establishes the Multiply operator as an infix, left associative operator. We have also initialized our operator precedence to 40. Again, the InfixLeftAssoc superclass and other ancestor classes compute left and right binding power (LBP and RBP) from the value of precedence and associativity and implement the `led` method ("left denotation" parselette method) and any utility methods and members. This concrete subclass serves the following purposes:

1. encodes the affix (by specifying its superclass)
2. encodes the associativity  (by specifying its superclass)
3. records the precedence
4. provides a home for `MultiplyMethodA` and `MultiplyMethodB`

But why are we using different classes at all? This OOP design has several flaws:

* It violates the principle of separation of concerns: Why are AST nodes doing the work of the parser?
* It violates the DRY Principle: Unless you autogenerate the code, you need to write a class for every operator—even if you relegate the parslet code to a handful of superclasses.
* This parser design is littered with static data: operator tokens, constants for precedence, associativity, affix, and token IDs, all of which is redundant, as it exists in a table used by the driver algorithm anyway. (Ironically, it is precisely because of its object-oriented design that the code and the data it acts upon are so disparate. This is not entirely the fault of OOP per se but rather of a poor choice of what concepts should be materialized as objects.)
* Generalizing the previous point: This design fixes the language at compile time. If you want to change the precedence of an operator, you need to rewrite, recompile, and redeploy the parser.
* It is cumbersome to write an operator table statically: Unless the code is automatically generated, writing "`parser.registeroperator(op, prec, assoc, whatever)`," the code that line depends on, and every subclass for every single operator is a bummer. Even if you autogenerate code, you have to write a code generator.

The temptation to write a code generator is often a sign that a more flexible design exists, a design that exploits whatever regularity exists in the code that makes programmatically generating the code possible in the first place. *In principle*, if code can automatically be generated, it can also be automatically compiled and executed. So maybe the (hypothetical) generate-compile-run pipeline (usually called a JIT or jitter) can be refactored to eliminate the compile step. In our case, instead of writing a bespoke Pratt parser in which the operator table is both encoded in the class hierarchy and generated again at runtime, why not write a generic Pratt parser that reads in the operator database at startup? As a bonus, modifying the language does not require a recompile: You can add, remove, or modify operators at *runtime* if you'd like, and maintaining the expression grammar is as simple as editing a value in a spreadsheet. (Indeed, it could be literally that!)


### Operator Database

As a toy example, we might have an operator database as follows.

| TokenID | Operator | NameString | Precedence | Associativity | Affix | Arity   |
| ------- | -------- | ---------- | ---------- | ------------- | ----- | ------- |
| 1       | `"123"`  | `"Number"` | 0          | None          | Null  | Nullary |
| 2       | `"^"`    | `"Power"`  | 10         | Right         | Infix | Binary  |
| 3       | `"*"`    | `"Times"`  | 20         | Full          | Infix | Binary  |
| 4       | `"/"`    | `"Divide"` | 20         | Left          | Infix | Binary  |
| 5       | `"+"`    | `"Plus"`   | 30         | Full          | Infix | Binary  |
| 6       | `"-"`    | `"Minus"`  | 30         | Left          | Infix | Binary  |

The `TokenID` might be supplied by the lexer/scanner (many Pratt parsers are scanner-less) and will be used as the identifier. `Operator` and `NameString` are only used for printing output. The remaining columns are required to compute the left and right binding powers of each operator. In this example language, every operator is either a terminal (number) or a binary infix operator.


### More Sophisticated Operators

Suppose we have ternary, mixfix, or matchfix operators. Then we need to modify the operator database to reflect how the operator tokens appear in an expression. A portion of our operator table might now look like this.

| TokenID  | LToken   | NToken   | OToken   | NameString      | Precedence | Associativity | Affix    | Arity    |
| -------- | -------- | -------- | -------- | --------------- | ---------- | ------------- | -------- | -------- |
| 10       | `"("`    |          | `")"`    | `"Parentheses"` | 10         | Non           | Matchfix | Unary    |
| &vellip; | &vellip; | &vellip; | &vellip; | &vellip;        | &vellip;   | &vellip;      | &vellip; | &vellip; |
| 43       | `"["`    |          | `"]"`    | `"Index"`       | 30         | Left          | Infix    | Binary   |
| 44       |          | `"!"`    |          | `"Factorial"`   | 40         | Left          | Postfix  | Unary    |
| 46       |          | `"-"`    |          | `"UnaryMinus"`  | 50         | Right         | Prefix   | Unary    |
| 49       | `"/"`    |          |          | `"Divide"`      | 60         | Left          | Infix    | Binary   |
| 55       | `"?"`    |          | `":"`    | `"IfThenElse"`  | 70         | Left          | Infix    | Ternary  |
| 57       | `"+"`    |          |          | `"Plus"`        | 80         | Full          | Infix    | Binary   |
| 60       | `"-"`    |          |          | `"Minus"`       | 90         | Left          | Infix    | Binary   |

In this design, the database includes which tokens of the operator can take a left operand (`LToken`), can begin an expression (no left operand, `NToken`), or are included in some other position (`OToken`).

!!! note
    The `LToken`, `NToken`, `OToken`, `Affix`, and `Arity` can all be infered from a single example usage. For example:

    ```c
    op1 ? op2 : op3
    ```

To reiterate the point, this table of operators might live in a plaintext CSV file. At startup—not at compile time—the Pratt parser reads in the operator table. AST nodes know their identity by their `TokenID` (which is really an operator ID) or string representation and perform identity-specific actions via dynamic dispatch. 

### Dynamic Dispatch

That last sentence should have raised your suspicion. A fundamental benefit of this design, I claim, is that it keeps you from having to write boilerplate for every operator. Are we just shifting the boilerplate from the operator AST subclasses to a jump table implementation? 

No. Until computers can read our minds, there is no getting around having to write operator-specific code. But we _are_ avoiding writing the surrounding class definition and whatever code is used to statically define the operator table. The latter is replaced with a simple and generic function that reads the table into memory from disk. There are several alternatives for replacing the former.

What we need is a mechanism to allow new functions to be created/loaded at runtime and associated with an operator action. The program will then dispatch the call dynamically, choosing the appropriate function for each operator at runtime. Since both the choice of which function to call _and_ the set of available choices of functions is determined at runtime, let's call this *double dynamic dispatch*. Regular dynamic dispatch is as old as the hills and just as common: a simple jump table will do. Interpreters for dynamic programming languages usually implement some kind of double dynamic dispatch, often in two different forms: by calling functions created dynamically in the interpreted language, and by calling out to external library functions written in C, for example. Which technique to use will depend on the nature of the operator-specific functions. To take two examples:

- If your operator-specific functions can be decomposed into simple primitives, say, AST transformations, then they can be described in an encoding that can be consumed by combinators or a simple state machine. This is essentially a thinly veiled DSL and corresponds philosophically to calling functions created dynamically in the interpreted language.
- Dynamically load compiled libraries of functions with a predetermined API implementing operator actions pointers to which can be stored in a lookup table. This is how interpreted languages implement their FFIs.

Neither of these techniques are as difficult as they appear. Don't let them intimidate you.

## Skeleton code



<hr>
<span style="font-size:9pt;">[1] The theory is much richer than I am able to present here. It is utterly fascinating, existing at the intersection of linguistics, mathematics, and computer science. Computer scientist Jeffery Kegler calls Chomsky's book *Syntactic Structures* "one of the most important books of all time."</span><br>
<span style="font-size:9pt;">Chomsky, Noam. 1957. *Syntactic Structures*. Mouton & Co., 1957.</span>

<span style="font-size:9pt;">[2] See Jeffery Kegler's charming [*"Parsing: a timeline"*](https://jeffreykegler.github.io/personal/timeline_v3) from which most of my historical information is derived.</span>

<span style="font-size:9pt;">[3] The name is telling of the state of the art in compiler construction at the time, as there were several other "compiler compilers" in use at Bell Labs.<br>Johnson, Stephen C. 1975. "Yacc: Yet Another Compiler-Compiler". AT&T Bell Laboratories Technical Reports. AT&T Bell Laboratories Murray Hill, New Jersey 07974 (32). Retrieved 22 December 2018.</span>

<span style="font-size:9pt;">[4] Theodore Norvell, Parsing Expressions by Recursive Descent. 1999.
http://www.engr.mun.ca/~theo/Misc/exp_parsing.htm</span>

<span style="font-size:9pt;">[5] Theodore Norvell, From Precedence Climbing to Pratt Parsing. 2016.
http://www.engr.mun.ca/~theo/Misc/pratt_parsing.htm</span>

<span style="font-size:9pt;">[6] Andy Chu, Pratt Parsing and Precedence Climbing Are the Same Algorithm. 2016.
https://www.oilshell.org/blog/2016/11/01.html</span>

<span style="font-size:9pt;">[7] Vaughan R. Pratt. 1973. Top down operator precedence. In *Proceedings of the 1st annual ACM SIGACT-SIGPLAN symposium on Principles of programming languages* (POPL '73). ACM, New York, NY, USA, 41-51. DOI=http://dx.doi.org/10.1145/512927.512931. Online: https://tdop.github.io/.</span>

<span style="font-size:9pt;">[8] A. V. Aho, S. C. Johnson, and J. D. Ullman. 1973. Deterministic parsing of ambiguous grammars. In *Proceedings of the 1st annual ACM SIGACT-SIGPLAN symposium on Principles of programming languages* (POPL '73). ACM, New York, NY, USA, 1-21. DOI=http://dx.doi.org/10.1145/512927.512928</span>

<span style="font-size:9pt;">[9]  James H. Morris, Jr.. 1973. Types are not sets. In *Proceedings of the 1st annual ACM SIGACT-SIGPLAN symposium on Principles of programming languages* (POPL '73). ACM, New York, NY, USA, 120-124. DOI=http://dx.doi.org/10.1145/512927.512938</span>

<span style="font-size:9pt;">[10] Pratt, Vaughan R. 1976. CGOL: An Alternative External Representation for LISP Users. AI Working Paper 121, MIT Artificial Intelligence Laboratory, Cambridge, MA, Mar. 1976. Online: https://dspace.mit.edu/bitstream/handle/1721.1/41951/AI_WP_121.pdf?sequence=1.</span>
