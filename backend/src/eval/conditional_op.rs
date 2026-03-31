use ultraviolet_core::{
    errors::SpannedError,
    types::{
        backend::{ControlFlow, EnvRef, Environment},
        frontend::ast::{ConditionalOperator, UVValue},
    },
};

use crate::eval::{eval, eval_every};

/// Evaluate conditional operator
pub fn eval_conditional_op(
    co: &Box<ConditionalOperator>,
    env: EnvRef,
) -> Result<ControlFlow, SpannedError> {
    let binding = eval(&co.test, env.clone())?;
    let evaluated_test = binding.flatten();

    let test_result = match evaluated_test {
        UVValue::Boolean(b) => b,
        _ => {
            return Err(SpannedError::new(
                "Unexpected type for `test` expression. Expected `bool`",
                co.span,
            ));
        }
    };

    let branch = if *test_result {
        &co.then_body
    } else {
        &co.else_body
    };

    if let Some(body) = branch {
        let new_env = Environment::new_child(env.clone());
        return eval_every(&body.value, new_env);
    }

    Ok(ControlFlow::Simple(UVValue::Void))
}
