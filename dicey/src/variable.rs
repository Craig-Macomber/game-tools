use super::Expression;
use crate::{
    Result,
    expression::parse_expression,
    parser::{RollParser, Rule},
};
use pest::Parser;
use std::collections::HashMap;

/// Parse a variable declaration.
pub(crate) fn parse_variable(s: &str, variables: &HashMap<String, Expression>) -> Result<Variable> {
    let mut pairs = RollParser::parse(Rule::variable_command, s)?;
    let variable = pairs.next().unwrap();
    assert_eq!(variable.as_rule(), Rule::variable);
    let identifier = variable.into_inner().next().unwrap();
    assert_eq!(identifier.as_rule(), Rule::variable_identifier);
    let variable_identifier = identifier.as_str().to_string();

    let expr_type = pairs.next().unwrap();
    assert_eq!(expr_type.as_rule(), Rule::expr);
    let expr = expr_type.into_inner();

    let expression = parse_expression(expr, variables)?;

    let reason = if let Some(reason) = pairs.next()
        && reason.as_rule() == Rule::reason_message
    {
        Some(reason.as_str().trim().to_owned())
    } else {
        None
    };

    Ok(Variable {
        identifier: variable_identifier,
        expression,
        reason,
    })
}

/// A parsed variable declaration.
#[derive(Debug)]
pub struct Variable {
    /// The expression for the value of the variable.
    pub expression: Expression,
    /// The identifier of the variable.
    pub identifier: String,
    /// The reason / comment associated with the variable, if any.
    pub reason: Option<String>,
}

impl Variable {
    /// Parse a command expression.
    pub fn parse(s: &str) -> Result<Variable> {
        Variable::parse_with_variables(s, &HashMap::default())
    }

    /// Parse a command expression.
    pub fn parse_with_variables(
        s: &str,
        variables: &HashMap<String, Expression>,
    ) -> Result<Variable> {
        parse_variable(s, variables)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Rollable, Verbosity, expression::FancyFormat};

    use super::*;

    #[test]
    fn minimal() {
        let variables = HashMap::default();
        let parsed = parse_variable("$x = 1", &variables);
        let variable = parsed.unwrap();
        assert_eq!(variable.identifier, "x");
        assert_eq!(variable.expression.roll().unwrap().total(), 1.0);
    }

    #[test]
    fn compact() {
        let variables = HashMap::default();
        let parsed = parse_variable("$x=2", &variables);
        let variable = parsed.unwrap();
        assert_eq!(variable.identifier, "x");
        assert_eq!(variable.expression.roll().unwrap().total(), 2.0);
    }

    #[test]
    fn basic() {
        let variables = HashMap::default();
        let variable = parse_variable("$xx = 5", &variables).unwrap();
        let result = variable.expression.roll().unwrap();
        assert_eq!(result.format_history(true, Verbosity::Medium), "5");
        assert_eq!(variable.identifier, "xx");
    }

    #[test]
    fn using_variable() {
        let mut variables = HashMap::default();
        let variable = parse_variable("$x = 5 : demo", &variables).unwrap();
        assert_eq!(variable.reason.unwrap(), "demo");
        variables.insert(variable.identifier, variable.expression);
        let variable = parse_variable("$y = $x + 2", &variables).unwrap();

        let result = variable.expression.roll().unwrap();
        assert_eq!(result.format(true, Verbosity::Short), "$x + 2 = **7**");
        assert_eq!(variable.identifier, "y");
        assert_eq!(
            result.format(true, Verbosity::Verbose),
            "($x: 5) + 2 = **7**"
        );
    }
}
