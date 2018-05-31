//! SML type checking.

use sml::{Expression, Schema};

use std::collections::HashMap;
use std::rc::Rc;

/// The type checking environment stores the type of each variable that is in
/// scope.
#[derive(Clone, Debug)]
pub struct Environment<'e>
{
    pub parent: Option<&'e Environment<'e>>,
    pub variables: HashMap<Rc<str>, Schema>,
}

impl<'e> Environment<'e>
{
    /// Create a child environment.
    pub fn derive(&'e self) -> Self
    {
        Environment{parent: Some(self), variables: HashMap::new()}
    }

    /// Define a new variable.
    pub fn define(&mut self, name: Rc<str>, schema: Schema)
    {
        self.variables.insert(name, schema);
    }

    /// Look up the type of a variable by name.
    pub fn schema_of(&self, name: &str) -> Option<Schema>
    {
        if let Some(schema) = self.variables.get(name)
        {
            Some(*schema)
        }
        else if let Some(parent) = self.parent
        {
            parent.schema_of(name)
        }
        else
        {
            None
        }
    }
}

/// Type errors.
#[derive(Clone, Debug)]
pub enum Error
{
    UnknownVariable(Rc<str>),
    TypeMismatch,
}

/// Type check an expression, returning its type.
pub fn check<'g>(g: &'g Environment, e: &Expression) -> Result<Schema, Error>
{
    use self::Expression::*;
    match e
    {
        SampleNumber =>
            Ok(Schema::Float),

        SampleRate =>
            Ok(Schema::Float),

        Variable(name) =>
            g.schema_of(name).ok_or(Error::UnknownVariable(name.clone())),

        Let(name, value, body) => {
            let schema = check(g, value)?;
            let mut inner_g = g.derive();
            inner_g.define(name.clone(), schema.clone());
            check(&inner_g, body)
        },

        If(condition, then, otherwise) => {
            let condition_schema = check(g, condition)?;
            let then_schema = check(g, then)?;
            let otherwise_schema = check(g, otherwise)?;
            require_schema(condition_schema, Schema::Bool)?;
            require_schema(then_schema, otherwise_schema)?;
            Ok(then_schema)
        },

        Float(_) =>
            Ok(Schema::Float),

        Arithmetic(_, a, b) => {
            let a_schema = check(g, a)?;
            let b_schema = check(g, b)?;
            require_schema(a_schema, Schema::Float)?;
            require_schema(b_schema, Schema::Float)?;
            Ok(Schema::Float)
        },

        Trigonometric(_, a) => {
            let a_schema = check(g, a)?;
            require_schema(a_schema, Schema::Float)?;
            Ok(Schema::Float)
        },
    }
}

fn require_schema(a: Schema, b: Schema) -> Result<(), Error>
{
    if a == b
    {
        Ok(())
    }
    else
    {
        Err(Error::TypeMismatch)
    }
}
