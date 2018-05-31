//! The Sample Manipulation Language, SML for short, is a first-order, total,
//! typed programming language for generating and transforming samples.
//!
//! In this module you can find types for representing SML expressions, and
//! functions for analyzing and evaluating them.

use std::rc::Rc;

pub mod check;

/// A type in SML. Called a _schema_ because _type_ is a Rust keyword.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Schema
{
    /// Booleans.
    Bool,

    /// 32-bit floating-point numbers.
    Float,

    /// Filters.
    Filter,
}

/// An expression in SML.
#[derive(Clone, Debug)]
pub enum Expression
{
    /// The sample number.
    SampleNumber,

    /// The sample rate.
    SampleRate,

    /// A reference to a variable.
    Variable(Rc<str>),

    /// A local variable binding.
    Let(Rc<str>, Rc<Expression>, Rc<Expression>),

    /// An if expression.
    If(Rc<Expression>, Rc<Expression>, Rc<Expression>),

    /// A 32-bit floating-point number.
    Float(f32),

    /// An arithmetic operation.
    Arithmetic(Arithmetic, Rc<Expression>, Rc<Expression>),

    /// A trigonometric operation.
    Trigonometric(Trigonometric, Rc<Expression>),
}

/// An arithmetic operation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Arithmetic
{
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// A trigonometric operation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Trigonometric
{
    Sine,
    Cosine,
}
