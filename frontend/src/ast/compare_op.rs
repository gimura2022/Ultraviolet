use crate::{
    ast::{
        GeneratorOutputType, generate_ast,
        traits::{IsVariadic, StringToUVCompareOp},
        types::{ASTBlockType, CompareOp},
    },
    errors::SpannedError,
    tokens_parser::types::UVParseNode,
};

/// Parse Ultraviolet compare operators
pub fn parse_compare_op(node: &UVParseNode) -> GeneratorOutputType {
    let op_type = node
        .name
        .to_uvcompare()
        .ok_or(SpannedError::new("Unknown comparison operation", node.span))?;

    let children = parse_arguments(node, !op_type.is_variadic())?;

    Ok(ASTBlockType::CompareOp(CompareOp {
        op_type,
        operands: children,
        span: node.span,
    }))
}

/// Parse arguments for compare
fn parse_arguments(node: &UVParseNode, only_two: bool) -> Result<Vec<ASTBlockType>, SpannedError> {
    if !node.all_tags() {
        return Err(SpannedError::new(
            "Unexpected literals inside parse operation",
            node.span,
        ));
    }

    if node.children_len() < 2 {
        return Err(SpannedError::new(
            "Comparison operator cannot have less than 2 operands",
            node.span,
        ));
    }

    if only_two && node.children_len() != 2 {
        return Err(SpannedError::new(
            format!("`{}` math operation can handle only 2 arguments", node.name),
            node.span,
        ));
    }

    node.get_all_tags()
        .into_iter()
        .map(|ch| generate_ast(ch))
        .collect::<Result<Vec<ASTBlockType>, SpannedError>>()
}
