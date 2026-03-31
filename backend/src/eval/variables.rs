use crate::eval::eval;
use ultraviolet_core::{
    errors::SpannedError,
    types::{
        backend::{ControlFlow, EnvRef, Environment, Symbol},
        frontend::ast::{UVValue, VariableAccess, VariableAssign, VariableDefinition},
    },
};

/// Define variable
pub fn define_variable(
    var_def: &Box<VariableDefinition>,
    env: EnvRef,
) -> Result<ControlFlow, SpannedError> {
    if env.borrow().find(var_def.name.value.clone()).is_some() {
        return Err(SpannedError::new(
            format!("Variable `{}` already defined", var_def.name.value),
            var_def.span,
        ));
    }

    let new_env = Environment::new_child(env.clone());
    let binding = eval(&var_def.value.value, new_env)?;
    let value = binding.flatten();

    env.borrow_mut()
        .define_variable(var_def.name.value.clone(), value.clone());

    Ok(ControlFlow::Simple(UVValue::Void))
}

/// Access variable by value
pub fn access_variable(var_acc: &VariableAccess, env: EnvRef) -> Result<ControlFlow, SpannedError> {
    match env.borrow().find(var_acc.name.clone()) {
        Some(sym) => match sym {
            Symbol::Variable(val) => Ok(ControlFlow::Simple(val.borrow().clone())),
            _ => {
                return Err(SpannedError::new(
                    format!("`{}` not a variable", var_acc.name),
                    var_acc.span,
                ));
            }
        },
        None => {
            return Err(SpannedError::new(
                format!("Variable `{}` not defined", var_acc.name),
                var_acc.span,
            ));
        }
    }
}

/// Assign to a variable
pub fn assign_variable(
    assign_var: &VariableAssign,
    env: EnvRef,
) -> Result<ControlFlow, SpannedError> {
    let sym = env.borrow().find(assign_var.name.clone());
    match sym {
        Some(sym) => match sym {
            Symbol::Variable(val) => {
                // Create new inner scope for assign block
                let new_env = Environment::new_child(env);

                *val.borrow_mut() = eval(&assign_var.value, new_env)?.flatten().clone();
                Ok(ControlFlow::Simple(UVValue::Void))
            }
            _ => {
                return Err(SpannedError::new(
                    format!("`{}` not a variable", assign_var.name),
                    assign_var.span,
                ));
            }
        },
        None => {
            return Err(SpannedError::new(
                format!("Variable `{}` not defined", assign_var.name),
                assign_var.span,
            ));
        }
    }
}
