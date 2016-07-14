// Tagua VM
//
//
// New BSD License
//
// Copyright © 2016-2016, Ivan Enderlin.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//     * Redistributions of source code must retain the above copyright
//       notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//     * Neither the name of the Hoa nor the names of its contributors may be
//       used to endorse or promote products derived from this software without
//       specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS AND CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

//! List of lexemes.
//!
//! The list of all lexemes, aka tokens, is provided by the PHP Language
//! Specification in the [Grammar
//! chapter](https://github.com/php/php-langspec/blob/master/spec/19-grammar.md#lexical-grammar).
//!
//! All lexemes are declared as static string constants.

/// Helper to declare a token.
///
/// ### Examples
///
/// The following example declares the `FOO_BAR` token:
///
/// ```
/// token!(FOO_BAR: b"foobar"; "The `FOO_BAR` token, mostly used in example.");
/// ```
macro_rules! token {
    ($name:ident: $value:expr; $documentation:expr) => (
        #[doc=$documentation]
        const $name: &'static [u8] = $value;
    );

    (pub $name:ident: $value:expr; $documentation:expr) => (
        #[doc=$documentation]
        pub const $name: &'static [u8] = $value;
    )
}

token!(
    pub ABSTRACT: b"abstract";
    "The `ABSTRACT` token.\n\nRepresent an abstract entity, e.g. `abstract class C { … }`."
);
token!(
    pub ADD: b"+";
    "The `ADD` token.\n\nRepresent the addition operator, e.g. `$x + $y`."
);
token!(
    pub ADD_AND_ASSIGN: b"+=";
    "The `ADD_AND_ASSIGN` token.\n\nRepresent the addition assignment operator, e.g. `$x += $y;`."
);
token!(
    pub AND: b"and";
    "The `AND` token.\n\nRepresent the conjunction operator, used in a logical expression, e.g. `$x and $y`."
);
token!(
    pub ARRAY: b"array";
    "The `ARRAY` token.\n\nRepresent the array constructor, e.g. `array($x, $y)`."
);
token!(
    pub AS: b"as";
    "The `AS` token.\n\nRepresent the alias operator, e.g. `use Foo\\Bar as Baz`."
);
token!(
    pub ASSIGN: b"=";
    "The `ASSIGN` token.\n\nRepresent a binding of a value to a variable, e.g. `$x = 42`."
);
token!(
    pub BITWISE_AND: b"&";
    "The `BITWISE_AND` token.\n\nRepresent the bitwise conjunction operator, e.g. `$x & $y`."
);
token!(
    pub BITWISE_AND_AND_ASSIGN: b"&=";
    "The `BITWISE_AND_AND_ASSIGN` token.\n\nRepresent the bitwise conjunction assignment operator, e.g. `$x &= $y;`."
);
token!(
    pub BITWISE_LEFT_SHIFT: b"<<";
    "The `BITWISE_LEFT_SHIFT` token.\n\nRepresent the bitwise left shift operator, e.g. `$x << $y`."
);
token!(
    pub BITWISE_LEFT_SHIFT_AND_ASSIGN: b"<<=";
    "The `BITWISE_LEFT_SHIFT_AND_ASSIGN` token.\n\nRepresent the bitwise left shift assignment operator, e.g. `$x <<= $y;`."
);
token!(
    pub BITWISE_NOT: b"~";
    "The `BITWISE_NOT` token.\n\nRepresent the bitwise negation operator, e.g. `~$x`."
);
token!(
    pub BITWISE_OR: b"|";
    "The `BITWISE_OR` token.\n\nRepresent the inclusive bitwise disjunction operator, e.g. `$x | $y`."
);
token!(
    pub BITWISE_OR_AND_ASSIGN: b"|=";
    "The `BITWISE_OR_AND_ASSIGN` token.\n\nRepresent the inclusive bitwise disjunction assignment operator, e.g. `$x |= $y;`."
);
token!(
    pub BITWISE_RIGHT_SHIFT: b">>";
    "The `BITWISE_RIGHT_SHIFT` token.\n\nRepresent the bitwise right shift operator, e.g. `$x >> $y`."
);
token!(
    pub BITWISE_RIGHT_SHIFT_AND_ASSIGN: b"<<=";
    "The `BITWISE_RIGHT_SHIFT_AND_ASSIGN` token.\n\nRepresent the bitwise right shift assignment operator, e.g. `$x >>= $y;`."
);
token!(
    pub BITWISE_XOR: b"^";
    "The `BITWISE_XOR` token.\n\nRepresent the exclusive bitwise disjunction operator, e.g. `$x ^ $y`."
);
token!(
    pub BITWISE_XOR_AND_ASSIGN: b"^=";
    "The `BITWISE_XOR_AND_ASSIGN` token.\n\nRepresent the exclusive bitwise disjunction assignment operator, e.g. `$x ^= $y;`."
);
token!(
    pub BOOLEAN_AND: b"&&";
    "The `BOOLEAN_AND` token.\n\nRepresent the boolean conjunction operator, e.g. `$x && $y`."
);
token!(
    pub BOOLEAN_NOT: b"!";
    "The `NOT` token.\n\nRepresent the boolean negation operator, e.g. `!$x`."
);
token!(
    pub BOOLEAN_OR: b"||";
    "The `BOOLEAN_OR` token.\n\nRepresent the boolean disjunction operator, e.g. `$x || $y`."
);
token!(
    pub BREAK: b"break";
    "The `BREAK` token.\n\nRepresent the control flow breaker operator, e.g. `break 2`."
);
token!(
    pub CALLABLE: b"callable";
    "The `CALLABLE` token.\n\nRepresent the callable type, e.g. `function f(callable $x) { … }`."
);
token!(
    pub CASE: b"case";
    "The `CASE` token.\n\nRepresent a case in a `switch` control structure, e.g. `switch (…) { case …: …; }`."
);
token!(
    pub CATCH: b"catch";
    "The `CATCH` token.\n\nRepresent the `catch` block of a `try`/`catch` control structure, e.g. `try { … } catch (Exception $e) { … }`."
);
token!(
    pub CLASS: b"class";
    "The `CLASS` token.\n\nRepresent the class declaration operator, e.g. `class C { … }`."
);
token!(
    pub CLONE: b"clone";
    "The `CLONE` token.\n\nRepresent the clone operator, e.g. `clone $x`."
);
token!(
    pub COALESCE: b"??";
    "The `COALESCE` token.\n\nRepresent the null coalescing operator, e.g. `$x ?? $y`."
);
token!(
    pub COMMA: b",";
    "The `COMMA` token.\n\nRepresent the list item separator, e.g. `($x, $y, $z)`."
);
token!(
    pub COMPARE: b"<=>";
    "The `COMPARE` token.\n\nRepresent the comparison operator, e.g. `$x <=> $y`."
);
token!(
    pub CONCATENATE: b".";
    "The `CONCATENATE` token.\n\nRepresent the concatenation operator, e.g. `'foo' . 'bar'`."
);
token!(
    pub CONCATENATE_AND_ASSIGN: b".=";
    "The `CONCATENATE_AND_ASSIGN` token.\n\nRepresent the concatenation assignment operator, e.g. `$x .= $y;`."
);
token!(
    pub CONST: b"const";
    "The `CONST` token.\n\nRepresent the constant declaration operator, e.g. `const ANSWER = 42;`."
);
token!(
    pub CONTINUE: b"continue";
    "The `CONTINUE` token.\n\nRepresent the control flow continuer operator, e.g. `continue 2;`."
);
token!(
    pub DECLARE: b"declare";
    "The `DECLARE` token.\n\nRepresent the declaration operator, e.g. `declare(foo='bar');`."
);
token!(
    pub DECREMENT: b"--";
    "The `DECREMENT` token.\n\nRepresent the decrement operator, e.g. `$number--`."
);
token!(
    pub DEFAULT: b"default";
    "The `DEFAULT` token.\n\nRepresent the default case in a `switch` control structure, e.g. `switch (…) { … default: …; }`."
);
token!(
    pub DIE: b"die";
    "The `DIE` token.\n\nRepresent the termination operator, e.g. `die(42);`."
);
token!(
    pub DIVIDE: b"/";
    "The `DIVIDE` token.\n\nRepresent the division operator, e.g. `$x / $y`."
);
token!(
    pub DIVIDE_AND_ASSIGN: b"/=";
    "The `DIVIDE_AND_ASSIGN` token.\n\nRepresent the division assignment operator, e.g. `$x /= $y`."
);
token!(
    pub DO: b"do";
    "The `DO` token.\n\nRepresent the body of a `do`/`while` loop, e.g. `do { … } while (…);`."
);
token!(
    pub DYNAMIC_CALL: b"->";
    "The `DYNAMIC_CALL` token.\n\nRepresent the dynamic method call operator, e.g. `$object->method()`."
);
token!(
    pub ECHO: b"echo";
    "The `ECHO` token.\n\nRepresent the output writer operator, e.g. `echo 'foobar';`."
);
token!(
    pub ELLIPSIS: b"...";
    "The `ELLIPSIS` token.\n\nRepresent the ellipsis operator, e.g. `$x...`."
);
token!(
    pub ELSE: b"else";
    "The `ELSE` token.\n\nRepresent the falsy block of a condition control structure, e.g. `if (…) { … } else { … }`."
);
token!(
    pub ELSEIF: b"elseif";
    "The `ELSEIF` token.\n\nRepresent a `else if` block, e.g. `if (…) { … } elseif { … } else { … }`."
);
token!(
    pub EMPTY: b"empty";
    "The `EMPTY` token.\n\nRepresent the emptiness operator, e.g. `empty($x)`."
);
token!(
    pub ENDDECLARE: b"enddeclare";
    "The `ENDDECLARE` token.\n\nRepresent the end of a `declare` block, e.g. `declare: … enddeclare`."
);
token!(
    pub ENDFOR: b"endfor";
    "The `ENDFOR` token.\n\nRepresent the end of a `for` block, e.g. `for (…; …; …): … endfor`."
);
token!(
    pub ENDFOREACH: b"endforeach";
    "The `ENDFOREACH` token.\n\nRepresent the end of a `foreach` block, e.g. `foreach ($i as $k => $v): … endforeach`."
);
token!(
    pub ENDIF: b"endif";
    "The `ENDIF` token.\n\nRepresent the end of an `if` block, e.g. `if (…): … endif`."
);
token!(
    pub ENDSWITCH: b"endswitch";
    "The `ENDSWITCH` token.\n\nRepresent the end of a `switch` block, e.g. `switch(…): … endswitch`."
);
token!(
    pub ENDWHILE: b"endwhile";
    "The `ENDWHILE` token.\n\nRepresent the end of a `while` block, e.g. `while(…): … endwhile`."
);
token!(
    pub EQUAL: b"==";
    "The `EQUAL` token.\n\nRepresent the equality comparison operator, e.g. `$x == $y`."
);
token!(
    pub EVAL: b"eval";
    "The `EVAL` token.\n\nRepresent the late-evaluation operator, e.g. `eval($x)`."
);
token!(
    pub EXIT: b"exit";
    "The `EXIT` token.\n\nRepresent the termination operator, e.g. `exit(42);`."
);
token!(
    pub EXTENDS: b"extends";
    "The `EXTENDS` token.\n\nRepresent the inheritance operator, e.g. `class C extends D { … }`."
);
token!(
    pub FINAL: b"final";
    "The `FINAL` token.\n\nRepresent a final entity, e.g. `final class C { … }`."
);
token!(
    pub FINALLY: b"finally";
    "The `FINALLY` token.\n\nRepresent the finally block of a `try`/`catch` control structure, e.g. `try { … } catch (…) { … } finally { … }`."
);
token!(
    pub FOR: b"for";
    "The `FOR` token.\n\nRepresent a `for` loop, e.g. `for (…; …; …) { … }`."
);
token!(
    pub FOREACH: b"foreach";
    "The `FOREACH` token.\n\nRepresent a `foreach` loop, e.g. `foreach ($i as $k => $v) { … }`."
);
token!(
    pub FUNCTION: b"function";
    "The `FUNCTION` token.\n\nRepresent the function declaration operator, e.g. `function f(…) { … }`."
);
token!(
    pub FUNCTION_OUTPUT: COLON;
    "The `FUNCTION_OUTPUT` token.\n\nRepresent the function return type declaration operator, e.g. `function f(…): … { … }`."
);
token!(
    pub GLOBAL: b"global";
    "The `GLOBAL` token.\n\nRepresent the global visibility modifier, e.g. `global $x`."
);
token!(
    pub GOTO: b"goto";
    "The `GOTO` token.\n\nRepresent the jump operator, e.g. `goto x;`."
);
token!(
    pub GREATER_THAN: b">";
    "The `GREATER_THAN` token.\n\nRepresent the greater than comparison operator, e.g. `$x > $y`."
);
token!(
    pub GREATER_THAN_OR_EQUAL_TO: b">=";
    "The `GREATER_THAN_OR_EQUAL_TO` token.\n\nRepresent the greater than or equal to comparison operator, e.g. `$x >= $y`."
);
token!(
    pub IDENTICAL: b"===";
    "The `IDENTICAL` token.\n\nRepresent the strict equality comparison operator, e.g. `$x === $y`."
);
token!(
    pub IF: b"if";
    "The `IF` token.\n\nRepresent the truly block of a condition control structure, e.g. `if (…) { … }`."
);
token!(
    pub IMPLEMENTS: b"implements";
    "The `IMPLEMENTS` token.\n\nRepresent the implementation operator, e.g. `class C implements I { … }`."
);
token!(
    pub INCLUDE: b"include";
    "The `INCLUDE` token.\n\nRepresent the import operator, e.g. `include $x;`."
);
token!(
    pub INCLUDE_ONCE: b"include_once";
    "The `INCLUDE_ONCE` token.\n\nRepresent the import once operator, e.g. `include_once $x;`."
);
token!(
    pub INCREMENT: b"++";
    "The `INCREMENT` token.\n\nRepresent the increment operator, e.g. `$number++`."
);
token!(
    pub INSTANCEOF: b"instanceof";
    "The `INSTANCEOF` token.\n\nRepresent the subset operator, e.g. `$o instanceof C`."
);
token!(
    pub INSTEADOF: b"insteadof";
    "The `INSTEADOF` token.\n\nRepresent the conflict resolution operator, `use C, D { C::f insteadof D }`."
);
token!(
    pub INTERFACE: b"interface";
    "The `INTERFACE` token.\n\nRepresent the interface declaration operator, e.g. `interface I { … }`."
);
token!(
    pub ISSET: b"isset";
    "The `ISSET` token.\n\nRepresent the existence operator, e.g. `isset($x)`."
);
token!(
    pub LEFT_CURLY_BRACKET: b"{";
    "The `LEFT_CURLY_BRACKET` token.\n\nUsed to open a block, e.g. `if (…) { … }`."
);
token!(
    pub LEFT_PARENTHESIS: b"]";
    "The `LEFT_PARENTHESIS` token.\n\nUsed to open a group of something, e.g. `if (…)`."
);
token!(
    pub LEFT_SQUARE_BRACKET: b"[";
    "The `LEFT_SQUARE_BRACKET` token.\n\nUsed to open an array construction or an array access for instance, e.g. `[2, 4, 6, 9][0]`."
);
token!(
    pub LESS_THAN: b"<";
    "The `LESS_THAN` token.\n\nRepresent the less than comparison operator, e.g. `$x < $y`."
);
token!(
    pub LESS_THAN_OR_EQUAL_TO: b"<=";
    "The `LESS_THAN_OR_EQUAL_TO` token.\n\nRepresent the less than or equal to comparison operator, e.g. `$x <= $y`."
);
token!(
    pub LIST: b"list";
    "The `LIST` token.\n\nRepresent the destructuring operator, e.g. `list($x, $y) = $a`."
);
token!(
    pub MODULO: b"%";
    "The `MODULO` token.\n\nRepresent the modulus operator, e.g. `$x % $y`."
);
token!(
    pub MODULO_AND_ASSIGN: b"%=";
    "The `MODULO_AND_ASSIGN` token.\n\nRepresent the modulus assignment operator, e.g. `$x %= $y;`."
);
token!(
    pub MULTIPLY: b"*";
    "The `MULTIPLY` token.\n\nRepresent the multiplication operator, e.g. `$x * $y`."
);
token!(
    pub MULTIPLY_AND_ASSIGN: b"*=";
    "The `MULTIPLY_AND_ASSIGN` token.\n\nRepresent the multiplication assignment operator, e.g. `$x *= $y;`."
);
token!(
    pub NAMESPACE: b"namespace";
    "The `NAMESPACE` token.\n\nRepresent the namespace declaration operator or the current namespace name, e.g. `namespace N;`."
);
token!(
    pub NAMESPACE_SEPARATOR: b"\\";
    "The `NAMESPACE_SEPARATOR` token.\n\nRepresent the namespace separator, e.g. `A\\B\\C`."
);
token!(
    pub NEW: b"new";
    "The `NEW` token.\n\nRepresent the instanciation operator, e.g. `new C()`."
);
token!(
    pub NOT_EQUAL: b"!=";
    "The `NOT_EQUAL` token.\n\nRepresent the not equal comparison operator, e.g. `$x != $y`."
);
token!(
    pub NOT_IDENTICAL: b"!==";
    "The `NOT_IDENTICAL` token.\n\nRepresent the strict not equal comparison operator, e.g. `$x !== $y`."
);
token!(
    pub NULLABLE: QUESTION_MARK;
    "The `NULLABLE` token.\n\nRepresent the nullable operation, e.g. `function f(?int $x) { … }`."
);
token!(
    pub OR: b"or";
    "The `OR` token.\n\nRepresent the inclusive disjunction operator, used in a logical expression, e.g. `$x or $y`."
);
token!(
    pub POW: b"**";
    "The `POW` token.\n\nRepresent the power operator, e.g. `$x ** $y`."
);
token!(
    pub POW_AND_ASSIGN: b"**=";
    "The `POW_AND_ASSIGN` token.\n\nRepresent the power assignment operator, e.g. `$x **= $y;`."
);
token!(
    pub PRINT: b"print";
    "The `PRINT` token.\n\nRepresent another output writer operator, e.g. `print 'foobar';`, see `echo`."
);
token!(
    pub PRIVATE: b"private";
    "The `PRIVATE` token.\n\nRepresent the private visibility operator, e.g. `private $x`."
);
token!(
    pub PROTECTED: b"protected";
    "The `PROTECTED` token.\n\nRepresent the protected visibility operator, e.g. `protected $x`."
);
token!(
    pub PUBLIC: b"public";
    "The `PUBLIC` token.\n\nRepresent the public visibility operator, e.g. `public $x`."
);
token!(
    pub REFERENCE: b"&";
    "The `REFERENCE` token.\n\nRepresent the reference operator, e.g. `&$x`."
);
token!(
    pub REQUIRE: b"require";
    "The `REQUIRE` token.\n\nRepresent the import operator, e.g. `require $x;`."
);
token!(
    pub REQUIRE_ONCE: b"require_once";
    "The `REQUIRE_ONCE` token.\n\nRepresent the import once operator, e.g. `require_once $x;`."
);
token!(
    pub RETURN: b"return";
    "The `RETURN` token.\n\nRepresent the return operator, e.g. `return $x;`."
);
token!(
    pub RIGHT_CURLY_BRACKET: b"{";
    "The `RIGHT_CURLY_BRACKET` token.\n\nUsed to close a block, e.g. `if (…) { … }`."
);
token!(
    pub RIGHT_PARENTHESIS: b")";
    "The `RIGHT_PARENTHESIS` token.\n\nUsed to close a group of something, e.g. `if (…)`."
);
token!(
    pub RIGHT_SQUARE_BRACKET: b"]";
    "The `RIGHT_SQUARE_BRACKET` token.\n\nUsed to close an array construction or an array access for instance, e.g. `[2, 4, 6, 9][0]`."
);
token!(
    pub SEMICOLON: b";";
    "The `SEMICOLON` token.\n\nRepresent the end of an instruction, e.g. `$x = …;`."
);
token!(
    pub STATIC: b"static";
    "The `STATIC` token.\n\nRepresent the stack declaration operator, e.g. `static $x`."
);
token!(
    pub STATIC_CALL: b"::";
    "The `STATIC_CALL` token.\n\nRepresent the static method call operator, e.g. `class::method()`."
);
token!(
    pub SUBTRACT: b"-";
    "The `SUBTRACT` token.\n\nRepresent the subtraction operator, e.g. `$x - $y`."
);
token!(
    pub SUBTRACT_AND_ASSIGN: b"-=";
    "The `SUBTRACT_AND_ASSIGN` token.\n\nRepresent the subtraction assignment operator, e.g. `$x -= $y;`."
);
token!(
    pub SWITCH: b"switch";
    "The `SWITCH` token.\n\nRepresent the switch control structure, e.g. `switch ($x) { … }`."
);
token!(
    pub TERNARY_ELSE: COLON;
    "The `TERNARY_ELSE` token.\n\nRepresent the falsy block of a ternary condition, e.g. `$x ? … : …`."
);
token!(
    pub TERNARY_THEN: QUESTION_MARK;
    "The `TERNARY_THEN` token.\n\nRepresent the truly block of a ternary condition, e.g. `$x ? … : …`."
);
token!(
    pub THROW: b"throw";
    "The `THROW` token.\n\nRepresent the throw exception operator, e.g. `throw $e;`."
);
token!(
    pub TRAIT: b"trait";
    "The `TRAIT` token.\n\nRepresent the trait declaration operator, e.g. `trait T { … }`."
);
token!(
    pub TRY: b"try";
    "The `TRY` token.\n\nRepresent the `try` block of a `try`/`catch` control structure, e.g. `try { … } catch (Exception $e) { … }`."
);
token!(
    pub UNSET: b"unset";
    "The `UNSET` token.\n\nRepresent the destruction operator, e.g. `unset($x);`."
);
token!(
    pub USE: b"use";
    "The `USE` token.\n\nRepresent the importing operator (for namespaces or traits for instance), e.g. `use C\\D;`."
);
token!(
    pub VAR: b"var";
    "The `VAR` token.\n\nRepresent the variable declaration operator (for old PHP versions), e.g. `var $x = …;`."
);
token!(
    pub VARIABLE: b"$";
    "The `VARIABLE` token.\n\nRepresent the variable declaration operator, e.g. `$foo`."
);
token!(
    pub WHILE: b"while";
    "The `WHILE` token.\n\nRepresent a `while` loop, e.g. `while (…) { … }`."
);
token!(
    pub XOR: b"xor";
    "The `XOR` token.\n\nRepresent the exclusive disjunction operator, used in a logical expression, e.g. `$x xor $y`."
);
token!(
    pub YIELD: b"yield";
    "The `YIELD` token.\n\nRepresent the generator operator, e.g. `yield …;`."
);
token!(
    pub YIELD_FROM: b"yield from";
    "The `YIELD_FROM` token.\n\nRepresent the delegated generator operator, e.g. `yield from …;`."
);

token!(
    COLON: b":";
    "The `COLON` private token.\n\nSee `FUNCTION_OUTPUT` and `TERNARY_ELSE`."
);
token!(
    QUESTION_MARK: b"?";
    "The `QUESTION_MARK` private token.\n\nSee `NULLABLE` and `TERNARY_THEN`."
);
