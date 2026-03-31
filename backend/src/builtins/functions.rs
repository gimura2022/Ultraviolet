use lazy_static::lazy_static;
use std::collections::HashMap;
use ultraviolet_core::{
    errors::SpannedError,
    types::{
        backend::{ControlFlow, EnvRef},
        frontend::ast::{FunctionCall, UVValue},
    },
};

use crate::eval::eval;

type BuiltinFunctionSignature =
    fn(fc: &FunctionCall, env: EnvRef) -> Result<ControlFlow, SpannedError>;

lazy_static! {
    static ref BUILTIN_FUNCTIONS: HashMap<&'static str, BuiltinFunctionSignature> = {
        let mut m = HashMap::new();
        m.insert("print", print as BuiltinFunctionSignature);
        m.insert("println", println as BuiltinFunctionSignature);
        m
    };
}

/// Check if provided function name is built-in function
pub fn is_builtin_function(name: &str) -> bool {
    BUILTIN_FUNCTIONS.contains_key(name)
}

/// Execute builtin function by signature
pub fn execute_builtin_function(
    fc: &FunctionCall,
    env: EnvRef,
) -> Result<ControlFlow, SpannedError> {
    match BUILTIN_FUNCTIONS.get(fc.name.as_str()) {
        Some(f) => f(fc, env),
        None => unreachable!(),
    }
}

/// Built-in `print` function
fn print(fc: &FunctionCall, env: EnvRef) -> Result<ControlFlow, SpannedError> {
    for arg in &fc.args {
        let e_r = eval(&arg.value, env.clone())?;
        let value = e_r.flatten();
        print!("{}", value);
    }

    Ok(ControlFlow::Simple(UVValue::Void))
}

/// Built-in `println` function
fn println(fc: &FunctionCall, env: EnvRef) -> Result<ControlFlow, SpannedError> {
    for arg in &fc.args {
        let e_r = eval(&arg.value, env.clone())?;
        let value = e_r.flatten();
        println!("{}", value);
    }

    Ok(ControlFlow::Simple(UVValue::Void))
}
