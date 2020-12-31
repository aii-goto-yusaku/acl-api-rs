use crate::app::model::relation_tuple::{Object, RelationTuple, User, UserSet};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// <tuple> ::= <object>'#'<relation>'@'<user> のパースをする
pub fn parse(token: &str) -> Result<RelationTuple, RelationTupleParserError> {
    let sharp_nth = token.find('#');
    let object = match sharp_nth {
        Some(nth) => Ok(Self::parse_object(&token[..nth])?),
        None => Err(RelationTupleParserError::InvalidFormatRelationTupleError(
            token.to_string(),
        )),
    }?;
    let sharp_nth = sharp_nth.unwrap();

    let (relation, user) = match token.find('@') {
        Some(nth) => Ok((
            token[sharp_nth + 1..nth].to_string(),
            Self::parse_user(&token[nth + 1..])?,
        )),
        None => Err(RelationTupleParserError::InvalidFormatRelationTupleError(
            token.to_string(),
        )),
    }?;

    Ok(RelationTuple {
        object,
        relation,
        user,
    })
}

/// <object> ::= <namespace>':'<object_id> のパースをする
fn parse_object(token: &str) -> Result<Object, RelationTupleParserError> {
    match token.find(':') {
        Some(nth) => Ok(Object {
            namespace: token[..nth].to_string(),
            object_id: token[nth + 1..].to_string(),
        }),
        None => Err(RelationTupleParserError::InvalidFormatObjectError(
            String::from(token),
        )),
    }
}

/// <user> ::= <user_id> | <userset>
/// <userset> ::= <object>'#'<relation>
/// 上記のパースをする
fn parse_user(token: &str) -> Result<User, RelationTupleParserError> {
    Ok(match token.find('#') {
        Some(nth) => User::UserSet(UserSet {
            object: Self::parse_object(&token[..nth])?,
            relation: token[nth + 1..].to_string(),
        }),
        None => User::UserID(token.to_string()),
    })
}

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum RelationTupleParserError {
    #[error(r#"invalid format error: format `<object>'#'<relation>'@'<user>` can be parsed: source: `{0}`"#)]
    InvalidFormatRelationTupleError(String),
    #[error(r#"invalid format error: format `<namespace>':'<object_id>` can be parsed: source: `{0}`"#)]
    InvalidFormatObjectError(String),
}
