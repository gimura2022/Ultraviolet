use crate::{
    ast::{
        GeneratorOutputType, generate_ast,
        types::{ASTBlockType, ForLoop},
    },
    errors::SpannedError,
    tokens_parser::types::UVParseNode,
    types::{Positional, Spanned},
};

pub fn parse_for_loop(node: &UVParseNode) -> GeneratorOutputType {
    let extra = node.search_extra_children(vec!["iterator", "start", "end", "step", "body"]);

    if !extra.is_empty() {
        let first_extra = extra.first().ok_or(SpannedError::new(
            "[INTERNAL ERROR] Cannot get inner extra tag",
            node.span,
        ))?;

        return Err(SpannedError::new(
            "Found extra children inside `for` loop declaration",
            first_extra.get_span(),
        ));
    }

    // Iterator

    let iterator_node = match node.get_child_by_name("iterator") {
        Some(x) if x.children_len() != 1 || !x.all_literals() => {
            return Err(SpannedError::new(
                "`iterator` child must have only one inner literal",
                x.span,
            ));
        }
        Some(x) => x,
        None => {
            return Err(SpannedError::new(
                "`for` loop must have an `iterator` child",
                node.span,
            ));
        }
    };

    let iterator = iterator_node.get_inner_literal().ok_or(SpannedError::new(
        "[INTERNAL ERROR] Cannot get inner literal for iterator",
        iterator_node.span,
    ))?;

    let step = match node.get_child_by_name("step") {
        Some(_) => Some(generate_ast(get_and_validate_inner_tag(node, "step")?)?),
        None => None,
    };

    let body = match node.get_child_by_name("body") {
        Some(x) => x,
        None => return Err(SpannedError::new("`for` loop must have a body", node.span)),
    };

    if !body.all_tags() {
        let literal = body.get_inner_literal().ok_or(SpannedError::new(
            "[INTERNAL ERROR] Cannot get extra literal in `for` body",
            body.span,
        ))?;
        return Err(SpannedError::new("Unexpected literal", literal.span));
    }

    let body_parsed = body
        .get_all_tags()
        .iter()
        .map(|n| generate_ast(n))
        .collect::<Result<Vec<ASTBlockType>, SpannedError>>()?;

    Ok(ASTBlockType::ForLoop(Box::new(ForLoop {
        iterator: iterator.clone(),
        start: generate_ast(get_and_validate_inner_tag(node, "start")?)?,
        end: generate_ast(get_and_validate_inner_tag(node, "end")?)?,
        step,
        body: Spanned::new(body_parsed, body.span),
        span: node.span,
    })))
}

fn get_and_validate_inner_tag<'a>(
    node: &'a UVParseNode,
    name: &'a str,
) -> Result<&'a UVParseNode, SpannedError> {
    let end_node = match node.get_child_by_name(&name) {
        Some(x) if x.children_len() != 1 || !x.all_tags() => {
            return Err(SpannedError::new(
                format!("`{name}` child must have only one inner tag"),
                x.span,
            ));
        }
        Some(x) => x,
        None => {
            return Err(SpannedError::new(
                format!("`for` loop must have an `{name}` tag"),
                node.span,
            ));
        }
    };

    end_node.get_tag_at(0).ok_or(SpannedError::new(
        "[INTERNAL ERROR] Cannot get inner tag",
        end_node.span,
    ))
}
