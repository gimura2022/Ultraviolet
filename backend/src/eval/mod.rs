use ultraviolet_core::{
    errors::SpannedError,
    types::{
        backend::{ControlFlow, EnvRef, Environment},
        frontend::ast::{ASTBlockType, UVValue},
    },
};

use crate::{
    builtins::functions::{execute_builtin_function, is_builtin_function},
    eval::{
        conditional_op::eval_conditional_op,
        program::eval_program,
        variables::{access_variable, assign_variable, define_variable},
    },
};
mod conditional_op;
mod program;
mod variables;

pub fn eval(node: &ASTBlockType, env: EnvRef) -> Result<ControlFlow, SpannedError> {
    Ok(match node {
        // Main program and others service blocks
        ASTBlockType::Program(program_block) => eval_program(program_block, env)?,
        ASTBlockType::HeadBlock(blocks) | ASTBlockType::MainBlock(blocks) => {
            eval_every(&blocks, env)?
        }

        // Variables things
        ASTBlockType::VariableDefinition(def) => define_variable(def, env)?,
        ASTBlockType::VariableAssignment(var_assign) => assign_variable(var_assign, env)?,
        ASTBlockType::VariableAccess(var_acc) => access_variable(var_acc, env)?,

        // Functions things
        ASTBlockType::FunctionDefinition(function_definition) => todo!(),
        ASTBlockType::FunctionCall(fc) if is_builtin_function(&fc.name) => {
            execute_builtin_function(fc, env)?
        }
        ASTBlockType::FunctionCall(function_call) => todo!(),

        ASTBlockType::ConditionalOp(co) => eval_conditional_op(co, env)?,
        ASTBlockType::MathOp(math_op) => todo!(),
        ASTBlockType::LogicalOp(logical_op) => todo!(),
        ASTBlockType::CompareOp(compare_op) => todo!(),
        ASTBlockType::ForLoop(for_loop) => todo!(),
        ASTBlockType::WhileLoop(while_loop) => todo!(),
        ASTBlockType::Value(val) => ControlFlow::Simple(val.value.clone()),
        ASTBlockType::GroupBlock(block) => eval_every(block, env)?,
        ASTBlockType::Return(block) => eval_return(block, env)?,
    })
}

/// Eval every block in node vector
fn eval_every(nodes: &Vec<ASTBlockType>, env: EnvRef) -> Result<ControlFlow, SpannedError> {
    let new_env = Environment::new_child(env);

    for node in nodes {
        match eval(&node, new_env.clone())? {
            ControlFlow::Return(val) => return Ok(ControlFlow::Return(val)),
            _ => {} // Continue eval on simple result
        }
    }

    Ok(ControlFlow::Simple(UVValue::Void))
}

/// Eval return expression
fn eval_return(node: &Box<ASTBlockType>, env: EnvRef) -> Result<ControlFlow, SpannedError> {
    let val = match eval(node, env)? {
        ControlFlow::Simple(val) | ControlFlow::Return(val) => val,
    };

    Ok(ControlFlow::Return(val))
}
