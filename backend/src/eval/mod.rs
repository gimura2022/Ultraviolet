use ultraviolet_core::{
    errors::SpannedError,
    types::{
        backend::{ControlFlow, EnvRef, Environment},
        frontend::ast::{ASTBlockType, UVValue},
    },
};

use crate::eval::{
    program::eval_program,
    variables::{access_variable, assign_variable, define_variable},
};
mod program;
mod variables;

pub fn eval(node: &ASTBlockType, env: EnvRef) -> Result<ControlFlow, SpannedError> {
    Ok(match node {
        ASTBlockType::Program(program_block) => eval_program(program_block, env)?,
        ASTBlockType::HeadBlock(blocks) | ASTBlockType::MainBlock(blocks) => {
            eval_every(&blocks, env)?
        }
        ASTBlockType::VariableDefinition(def) => define_variable(def, env)?,
        ASTBlockType::FunctionDefinition(function_definition) => todo!(),
        ASTBlockType::FunctionCall(function_call) => todo!(),
        ASTBlockType::VariableAssignment(var_assign) => assign_variable(var_assign, env)?,
        ASTBlockType::VariableAccess(var_acc) => access_variable(var_acc, env)?,
        ASTBlockType::ConditionalOp(conditional_operator) => todo!(),
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
