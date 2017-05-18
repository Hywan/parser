// Tagua VM
//
//
// New BSD License
//
// Copyright © 2016-2017, Ivan Enderlin.
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

//! Structures that will constitute the Abstract Syntax Tree.

use super::tokens::{
    Span,
    Token
};

/// A term.
#[derive(Debug, PartialEq)]
pub struct Term<'a> {
    /// The term value.
    pub t: Literal<'a>
}

/// An addition of two terms.
#[derive(Debug, PartialEq)]
pub struct Addition<'a> {
    /// The left-hand side of the addition.
    pub a: Term<'a>,
    /// The right-hand side of the addition.
    pub b: Term<'a>
}

/// A literal represents a fixed value, aka an atom.
#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    /// A boolean, either `true` or `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Literal;
    /// use tagua_parser::rules::literals::literal;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     literal(Span::new(b"true")),
    ///     Result::Done(
    ///         Span::new_at(b"", 4, 1, 5),
    ///         Literal::Boolean(Token::new(true, Span::new(b"true")))
    ///     )
    /// );
    /// assert_eq!(
    ///     literal(Span::new(b"false")),
    ///     Result::Done(
    ///         Span::new_at(b"", 5, 1, 6),
    ///         Literal::Boolean(Token::new(false, Span::new(b"false")))
    ///     )
    /// );
    /// # }
    /// ```
    Boolean(Token<'a, bool>),

    /// An integer, for instance a binary, octal, decimal or hexadecimal number.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Literal;
    /// use tagua_parser::rules::literals::literal;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     literal(Span::new(b"0b101010")),
    ///     Result::Done(
    ///         Span::new_at(b"", 8, 1, 9),
    ///         Literal::Integer(Token::new(42, Span::new(b"0b101010")))
    ///     )
    /// );
    /// assert_eq!(
    ///     literal(Span::new(b"052")),
    ///     Result::Done(
    ///         Span::new_at(b"", 3, 1, 4),
    ///         Literal::Integer(Token::new(42, Span::new(b"052")))
    ///     )
    /// );
    /// assert_eq!(
    ///     literal(Span::new(b"42")),
    ///     Result::Done(
    ///         Span::new_at(b"", 2, 1, 3),
    ///         Literal::Integer(Token::new(42, Span::new(b"42")))
    ///     )
    /// );
    /// assert_eq!(
    ///     literal(Span::new(b"0x2a")),
    ///     Result::Done(
    ///         Span::new_at(b"", 4, 1, 5),
    ///         Literal::Integer(Token::new(42, Span::new(b"0x2a")))
    ///     )
    /// );
    /// # }
    /// ```
    Integer(Token<'a, i64>),

    /// A null value.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Literal;
    /// use tagua_parser::rules::literals::literal;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     literal(Span::new(b"null")),
    ///     Result::Done(
    ///         Span::new_at(b"", 4, 1, 5),
    ///         Literal::Null(Token::new((), Span::new(b"null")))
    ///     )
    /// );
    /// # }
    /// ```
    Null(Token<'a, ()>),

    /// A real, for instance an exponential number.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Literal;
    /// use tagua_parser::rules::literals::literal;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     literal(Span::new(b"4.2")),
    ///     Result::Done(
    ///         Span::new_at(b"", 3, 1, 4),
    ///         Literal::Real(Token::new(4.2f64, Span::new(b"4.2")))
    ///     )
    /// );
    /// assert_eq!(
    ///     literal(Span::new(b".42e1")),
    ///     Result::Done(
    ///         Span::new_at(b"", 5, 1, 6),
    ///         Literal::Real(Token::new(4.2f64, Span::new(b".42e1")))
    ///     )
    /// );
    /// assert_eq!(
    ///     literal(Span::new(b"420e-2")),
    ///     Result::Done(
    ///         Span::new_at(b"", 6, 1, 7),
    ///         Literal::Real(Token::new(4.2f64, Span::new(b"420e-2")))
    ///     )
    /// );
    /// # }
    /// ```
    Real(Token<'a, f64>),

    /// A string.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Literal;
    /// use tagua_parser::rules::literals::literal;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     literal(Span::new(b"'foo\\'bar'")),
    ///     Result::Done(
    ///         Span::new_at(b"", 10, 1, 11),
    ///         Literal::String(Token::new(b"foo'bar".to_vec(), Span::new(b"'foo\\'bar'")))
    ///     )
    /// );
    /// # }
    /// ```
    String(Token<'a, Vec<u8>>)
}

/// A variable.
///
/// # Examples
///
/// ```
/// # extern crate tagua_parser;
/// use tagua_parser::Result;
/// use tagua_parser::ast::Variable;
/// use tagua_parser::rules::tokens::variable;
/// use tagua_parser::tokens::{
///     Span,
///     Token
/// };
///
/// # fn main() {
/// assert_eq!(
///     variable(Span::new(b"$foo")),
///     Result::Done(
///         Span::new_at(b"", 4, 1, 5),
///         Variable(Span::new_at(b"foo", 1, 1, 2))
///     )
/// );
/// # }
/// ```
/// Note that the `$` is not present.
#[derive(Debug, PartialEq)]
pub struct Variable<'a>(pub Span<'a>);

/// A name represents an entity name.
#[derive(Debug, PartialEq)]
pub enum Name<'a> {
    /// An unqualified name, i.e. a name without a namespace, like `Bar`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Name;
    /// use tagua_parser::rules::tokens::qualified_name;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     qualified_name(Span::new(b"Bar")),
    ///     Result::Done(
    ///         Span::new_at(b"", 3, 1, 4),
    ///         Name::Unqualified(Span::new(b"Bar"))
    ///     )
    /// );
    /// # }
    /// ```
    Unqualified(Span<'a>),

    /// A qualified name, i.e. a name in a relative namespace (aliased or not),
    /// like `Foo\Bar`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Name;
    /// use tagua_parser::rules::tokens::qualified_name;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     qualified_name(Span::new(b"Foo\\Bar")),
    ///     Result::Done(
    ///         Span::new_at(b"", 7, 1, 8),
    ///         Name::Qualified(vec![
    ///             Span::new(b"Foo"),
    ///             Span::new_at(b"Bar", 4, 1, 5)
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Qualified(Vec<Span<'a>>),

    /// A relative qualified name, i.e. a name in a relative namespace
    /// restricted to the current namespace, like `namespace\Foo\Bar`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Name;
    /// use tagua_parser::rules::tokens::qualified_name;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     qualified_name(Span::new(b"namespace\\Foo\\Bar")),
    ///     Result::Done(
    ///         Span::new_at(b"", 17, 1, 18),
    ///         Name::RelativeQualified(vec![
    ///             Span::new_at(b"Foo", 10, 1, 11),
    ///             Span::new_at(b"Bar", 14, 1, 15)
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    /// Note that the `namespace` part is not present.
    RelativeQualified(Vec<Span<'a>>),

    /// A fully qualified name, i.e. a name in an absolute namespace, like
    /// `\Foo\Bar`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Name;
    /// use tagua_parser::rules::tokens::qualified_name;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     qualified_name(Span::new(b"\\Foo\\Bar")),
    ///     Result::Done(
    ///         Span::new_at(b"", 8, 1, 9),
    ///         Name::FullyQualified(vec![
    ///             Span::new_at(b"Foo", 1, 1, 2),
    ///             Span::new_at(b"Bar", 5, 1, 6)
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    /// Note that the leading `\` part is not present.
    FullyQualified(Vec<Span<'a>>)
}

/// An expression.
#[derive(Debug, PartialEq)]
pub enum Expression<'a> {
    /// An anonymous function is defined like, and behaves like, a
    /// named function except that the former has no name, and an
    /// enclosed scope.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{
    ///     AnonymousFunction,
    ///     Arity,
    ///     Expression,
    ///     Name,
    ///     Parameter,
    ///     Scope,
    ///     Statement,
    ///     Ty,
    ///     Variable
    /// };
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"function (I $x, J &$y) use ($z): O { return; }"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::AnonymousFunction(
    ///             AnonymousFunction {
    ///                 declaration_scope: Scope::Dynamic,
    ///                 inputs           : Arity::Finite(vec![
    ///                     Parameter {
    ///                         ty   : Ty::Copy(Some(Name::Unqualified(&b"I"[..]))),
    ///                         name : Variable(&b"x"[..]),
    ///                         value: None
    ///                     },
    ///                     Parameter {
    ///                         ty   : Ty::Reference(Some(Name::Unqualified(&b"J"[..]))),
    ///                         name : Variable(&b"y"[..]),
    ///                         value: None
    ///                     }
    ///                 ]),
    ///                 output         : Ty::Copy(Some(Name::Unqualified(&b"O"[..]))),
    ///                 enclosing_scope: Some(vec![Expression::Variable(Variable(&b"z"[..]))]),
    ///                 body           : vec![Statement::Return]
    ///             }
    ///         )
    ///     )
    /// );
    /// # }
    /// ```
    AnonymousFunction(AnonymousFunction<'a>),

    /// A collection of heterogeneous pairs (key, value). The key is
    /// optional.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Literal, Variable};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"['foo', 42 => 'bar', 'baz' => $qux]"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::Array(vec![
    ///             (
    ///                 None,
    ///                 Expression::Literal(Literal::String(b"foo".to_vec()))
    ///             ),
    ///             (
    ///                 Some(Expression::Literal(Literal::Integer(42i64))),
    ///                 Expression::Literal(Literal::String(b"bar".to_vec()))
    ///             ),
    ///             (
    ///                 Some(Expression::Literal(Literal::String(b"baz".to_vec()))),
    ///                 Expression::Variable(Variable(&b"qux"[..]))
    ///             )
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Array(Vec<(Option<Expression<'a>>, Expression<'a>)>),

    /// Echo converts each of its expression's values into strings,
    /// concatenates them in order given, and writes the result to the
    /// output stream.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Literal, Variable};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"echo 'foobar', $bazqux, 42"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::Echo(vec![
    ///             Expression::Literal(Literal::String(b"foobar".to_vec())),
    ///             Expression::Variable(Variable(&b"bazqux"[..])),
    ///             Expression::Literal(Literal::Integer(42i64))
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Echo(Vec<Expression<'a>>),

    /// Returns `TRUE` if the variable or value designated by the
    /// expression is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Literal};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"empty('')"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::Empty(
    ///             Box::new(
    ///                 Expression::Literal(
    ///                     Literal::String(b"".to_vec())
    ///                 )
    ///             )
    ///         )
    ///     )
    /// );
    /// # }
    /// ```
    Empty(Box<Expression<'a>>),

    /// Late evaluation of a PHP program represented as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Literal};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"eval('1 + 2;')"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::Eval(
    ///             Box::new(
    ///                 Expression::Literal(
    ///                     Literal::String(b"1 + 2;".to_vec())
    ///                 )
    ///             )
    ///         )
    ///     )
    /// );
    /// # }
    /// ```
    Eval(Box<Expression<'a>>),

    /// Terminate the current script.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Literal};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"exit(42)"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::Exit(
    ///             Some(
    ///                 Box::new(
    ///                     Expression::Literal(
    ///                         Literal::Integer(42i64)
    ///                     )
    ///                 )
    ///             )
    ///         )
    ///     )
    /// );
    /// # }
    /// ```
    Exit(Option<Box<Expression<'a>>>),

    /// Return `TRUE` if all expressions set and their values are not
    /// `NULL`. Otherwise, it returns `FALSE`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Variable};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"isset($foo, $bar)"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::Isset(vec![
    ///             Expression::Variable(Variable(&b"foo"[..])),
    ///             Expression::Variable(Variable(&b"bar"[..]))
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Isset(Vec<Expression<'a>>),

    /// Match and assign one or more elements of the source array to
    /// the target variables.
    ///
    /// # Examples
    ///
    /// A keyed list:
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Literal, Variable};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"list('foo' => $foo, 'bar' => $bar, 'baz' => $baz)"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::List(vec![
    ///             Some((
    ///                 Some(Expression::Literal(Literal::String(b"foo".to_vec()))),
    ///                 Expression::Variable(Variable(&b"foo"[..]))
    ///             )),
    ///             Some((
    ///                 Some(Expression::Literal(Literal::String(b"bar".to_vec()))),
    ///                 Expression::Variable(Variable(&b"bar"[..]))
    ///             )),
    ///             Some((
    ///                 Some(Expression::Literal(Literal::String(b"baz".to_vec()))),
    ///                 Expression::Variable(Variable(&b"baz"[..]))
    ///             ))
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    /// An unkeyed list:
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Variable};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"list($foo, , , $bar, $baz)"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::List(vec![
    ///             Some((
    ///                 None,
    ///                 Expression::Variable(Variable(&b"foo"[..]))
    ///             )),
    ///             None,
    ///             None,
    ///             Some((
    ///                 None,
    ///                 Expression::Variable(Variable(&b"bar"[..]))
    ///             )),
    ///             Some((
    ///                 None,
    ///                 Expression::Variable(Variable(&b"baz"[..]))
    ///             ))
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    List(Vec<Option<(Option<Expression<'a>>, Expression<'a>)>>),

    /// A literal.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Literal};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"'Hello, World!'"),
    ///     Result::Done(&b""[..], Expression::Literal(Literal::String(b"Hello, World!".to_vec())))
    /// );
    /// # }
    /// ```
    Literal(Literal<'a>),

    /// A name.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Name};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"Foo\\Bar"),
    ///     Result::Done(&b""[..], Expression::Name(Name::Qualified(vec![&b"Foo"[..], &b"Bar"[..]])))
    /// );
    /// # }
    /// ```
    Name(Name<'a>),

    /// Unlike `echo`, `print` can be used in any context allowing an
    /// expression. It always returns the value `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Variable};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"print $foo"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::Print(
    ///             Box::new(
    ///                 Expression::Variable(Variable(&b"foo"[..])),
    ///             )
    ///         )
    ///     )
    /// );
    /// # }
    /// ```
    Print(Box<Expression<'a>>),

    /// Describe an expression assignment by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Literal, Variable};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"[7 => &$foo]"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::Array(vec![
    ///             (
    ///                 Some(Expression::Literal(Literal::Integer(7i64))),
    ///                 Expression::Reference(
    ///                     Box::new(Expression::Variable(Variable(&b"foo"[..])))
    ///                 )
    ///             )
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Reference(Box<Expression<'a>>),

    /// Unset the variables designated by each expression.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Variable};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"unset($foo, $bar)"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Expression::Unset(vec![
    ///             Expression::Variable(Variable(&b"foo"[..])),
    ///             Expression::Variable(Variable(&b"bar"[..]))
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Unset(Vec<Expression<'a>>),

    /// A variable.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{Expression, Variable};
    /// use tagua_parser::rules::expressions::expression;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     expression(b"$foo"),
    ///     Result::Done(&b""[..], Expression::Variable(Variable(&b"foo"[..])))
    /// );
    /// # }
    /// ```
    Variable(Variable<'a>)
}

/// A type declaration.
///
/// A type holds two informations: Name, and copy or reference. A type
/// can be native (fully qualified name), or user-defined with an
/// interface or a class (unqualified, qualified, or fully qualified
/// name). Note that the name is an `Option`: A binding can have no
/// type, but still hold the copy or reference constraint.
///
/// A type can be a copy type, it means the value must be
/// copied (at least on write), or it can be a reference type, it
/// means this is a new binding over an existing binding.
#[derive(Debug, PartialEq)]
pub enum Ty<'a> {
    /// A type representing a datum passed by copy.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{
    ///     Arity,
    ///     Name,
    ///     Parameter,
    ///     Ty,
    ///     Variable
    /// };
    /// use tagua_parser::rules::statements::function::parameters;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     parameters(b"(I $x)"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Arity::Finite(vec![
    ///             Parameter {
    ///                 ty   : Ty::Copy(Some(Name::Unqualified(&b"I"[..]))),
    ///                 name : Variable(&b"x"[..]),
    ///                 value: None
    ///             }
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Copy(Option<Name<'a>>),

    /// A type representing a datum passed by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{
    ///     Arity,
    ///     Name,
    ///     Parameter,
    ///     Ty,
    ///     Variable
    /// };
    /// use tagua_parser::rules::statements::function::parameters;
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     parameters(b"(I &$x)"),
    ///     Result::Done(
    ///         &b""[..],
    ///         Arity::Finite(vec![
    ///             Parameter {
    ///                 ty   : Ty::Reference(Some(Name::Unqualified(&b"I"[..]))),
    ///                 name : Variable(&b"x"[..]),
    ///                 value: None
    ///             }
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Reference(Option<Name<'a>>)
}

/// A parameter, aka input, of a function.
///
/// A parameter can:
///
///   * be typed or not,
///   * be passed by copy, or by reference,
///   * have a default constant value.
///
/// # Examples
///
/// ```
/// use tagua_parser::Result;
/// use tagua_parser::ast::{
///     Arity,
///     Expression,
///     Literal,
///     Name,
///     Parameter,
///     Ty,
///     Variable
/// };
/// use tagua_parser::rules::statements::function::parameters;
/// use tagua_parser::tokens::{
///     Span,
///     Token
/// };
///
/// # fn main() {
/// assert_eq!(
///     parameters(Span::new(b"($x = 42, I &$y)")),
///     Result::Done(
///         Span::new_at(b"", 16, 1, 17),
///         Arity::Finite(vec![
///             Parameter {
///                 ty   : Ty::Copy(None),
///                 name : Variable(Span::new_at(b"x", 2, 1, 3)),
///                 value: Some(Expression::Literal(Literal::Integer(Token::new(42i64, Span::new_at(b"42", 6, 1, 7)))))
///             },
///             Parameter {
///                 ty   : Ty::Reference(Some(Name::Unqualified(Span::new_at(b"I", 10, 1, 11)))),
///                 name : Variable(Span::new_at(b"y", 14, 1, 15)),
///                 value: None
///             }
///         ])
///     )
/// );
/// # }
/// ```
#[derive(Debug, PartialEq)]
pub struct Parameter<'a> {
    /// Type of the parameter.
    pub ty: Ty<'a>,

    /// Name of the parameter.
    pub name: Variable<'a>,

    /// Default value of the parameter.
    pub value: Option<Expression<'a>>
}

/// Arity of a function.
#[derive(Debug, PartialEq)]
pub enum Arity<'a> {
    /// A function with no parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::Arity;
    /// use tagua_parser::rules::statements::function::parameters;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     parameters(Span::new(b"()")),
    ///     Result::Done(Span::new_at(b"", 2, 1, 3), Arity::Constant)
    /// );
    /// # }
    /// ```
    Constant,

    /// A function with a finite number of parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{
    ///     Arity,
    ///     Parameter,
    ///     Ty,
    ///     Variable
    /// };
    /// use tagua_parser::rules::statements::function::parameters;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     parameters(Span::new(b"($x, $y)")),
    ///     Result::Done(
    ///         Span::new_at(b"", 8, 1, 9),
    ///         Arity::Finite(vec![
    ///             Parameter {
    ///                 ty   : Ty::Copy(None),
    ///                 name : Variable(Span::new_at(b"x", 2, 1, 3)),
    ///                 value: None
    ///             },
    ///             Parameter {
    ///                 ty   : Ty::Copy(None),
    ///                 name : Variable(Span::new_at(b"y", 6, 1, 7)),
    ///                 value: None
    ///             }
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Finite(Vec<Parameter<'a>>),

    /// A variadic function.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate tagua_parser;
    /// use tagua_parser::Result;
    /// use tagua_parser::ast::{
    ///     Arity,
    ///     Parameter,
    ///     Ty,
    ///     Variable
    /// };
    /// use tagua_parser::rules::statements::function::parameters;
    /// use tagua_parser::tokens::{
    ///     Span,
    ///     Token
    /// };
    ///
    /// # fn main() {
    /// assert_eq!(
    ///     parameters(Span::new(b"($x, ...$y)")),
    ///     Result::Done(
    ///         Span::new_at(b"", 11, 1, 12),
    ///         Arity::Infinite(vec![
    ///             Parameter {
    ///                 ty   : Ty::Copy(None),
    ///                 name : Variable(Span::new_at(b"x", 2, 1, 3)),
    ///                 value: None
    ///             },
    ///             Parameter {
    ///                 ty   : Ty::Copy(None),
    ///                 name : Variable(Span::new_at(b"y", 9, 1, 10)),
    ///                 value: None
    ///             }
    ///         ])
    ///     )
    /// );
    /// # }
    /// ```
    Infinite(Vec<Parameter<'a>>)
}

/// A function declaration.
///
/// # Examples
///
/// ```
/// use tagua_parser::Result;
/// use tagua_parser::ast::{
///     Arity,
///     Function,
///     Name,
///     Parameter,
///     Statement,
///     Ty,
///     Variable
/// };
/// use tagua_parser::rules::statements::function::function;
/// use tagua_parser::tokens::{
///     Span,
///     Token
/// };
///
/// # fn main() {
/// assert_eq!(
///     function(Span::new(b"function f(I $x): O { return; }")),
///     Result::Done(
///         Span::new_at(b"", 31, 1, 32),
///         Statement::Function(
///             Function {
///                 name  : Span::new_at(b"f", 9, 1, 10),
///                 inputs: Arity::Finite(vec![
///                     Parameter {
///                         ty   : Ty::Copy(Some(Name::Unqualified(Span::new_at(b"I", 11, 1, 12)))),
///                         name : Variable(Span::new_at(b"x", 14, 1, 15)),
///                         value: None
///                     }
///                 ]),
///                 output: Ty::Copy(Some(Name::Unqualified(Span::new_at(b"O", 18, 1, 19)))),
///                 body  : vec![Statement::Return]
///             }
///         )
///     )
/// );
/// # }
/// ```
#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    /// Name of the function.
    pub name: Span<'a>,

    /// Inputs, aka parameters, of the function.
    pub inputs: Arity<'a>,

    /// Output type of the function.
    pub output: Ty<'a>,

    /// Body of the function, i.e. a set of statements.
    pub body: Vec<Statement<'a>>
}

/// An anonymous function.
///
/// An anonymous function is defined like, and behaves like, a named
/// function `Function` except that the former has no name, and an
/// enclosed scope.
///
/// # Examples
///
/// ```
/// # extern crate tagua_parser;
/// use tagua_parser::Result;
/// use tagua_parser::ast::{
///     AnonymousFunction,
///     Arity,
///     Expression,
///     Name,
///     Parameter,
///     Scope,
///     Statement,
///     Ty,
///     Variable
/// };
/// use tagua_parser::rules::expressions::primaries::anonymous_function;
///
/// # fn main() {
/// assert_eq!(
///     anonymous_function(b"static function &(I ...$x) use (&$y, $z): O { return; }"),
///     Result::Done(
///         &b""[..],
///         Expression::AnonymousFunction(
///             AnonymousFunction {
///                 declaration_scope: Scope::Static,
///                 inputs           : Arity::Infinite(vec![
///                     Parameter {
///                         ty   : Ty::Copy(Some(Name::Unqualified(&b"I"[..]))),
///                         name : Variable(&b"x"[..]),
///                         value: None
///                     }
///                 ]),
///                 output         : Ty::Reference(Some(Name::Unqualified(&b"O"[..]))),
///                 enclosing_scope: Some(vec![
///                     Expression::Reference(
///                         Box::new(
///                             Expression::Variable(Variable(&b"y"[..]))
///                         )
///                     ),
///                     Expression::Variable(Variable(&b"z"[..]))
///                 ]),
///                 body: vec![Statement::Return]
///             }
///         )
///     )
/// );
/// # }
/// ```
#[derive(Debug, PartialEq)]
pub struct AnonymousFunction<'a> {
    /// Declaration scope of the anonymous function.
    pub declaration_scope: Scope,

    /// Inputs, aka parameters, of the anonymous function.
    pub inputs: Arity<'a>,

    /// Output type of the anonymous function.
    pub output: Ty<'a>,

    /// Enclosed scope (list of variables to be accessible from the
    /// body) of the anonymous function.
    pub enclosing_scope: Option<Vec<Expression<'a>>>,

    /// Body of the anonymous function, i.e. a set of statements.
    pub body: Vec<Statement<'a>>
}

/// A statement.
#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    /// A function declaration.
    Function(Function<'a>),

    /// A return.
    Return
}

/// A scope.
#[derive(Debug, PartialEq)]
pub enum Scope {
    /// A dynamic scope (default one).
    Dynamic,

    /// A static scope (when declared with the `static` keyword).
    Static
}
