pub mod parser;

use crate::app::model::relation_tuple::parser::{RelationTupleParser, RelationTupleParserError};
use actix_web::web::Query;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt;
use thiserror::Error;

/// ```
/// <tuple> ::= <object>'#'<relation>'@'<user>
///
/// <object> ::= <namespace>':'<object_id>
///
/// <user> ::= <user_id> | <userset>
///
/// <userset> ::= <object>'#'<relation>
/// ```
///
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelationTuple {
    pub object: Object,
    pub relation: String,
    pub user: User,
}

/// ```
/// <object> ::= <namespace>':'<object_id>
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Object {
    pub namespace: String,
    pub object_id: String,
}

/// ```
/// <user> ::= <user_id> | <userset>
/// <userset> ::= <object>'#'<relation>
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum User {
    UserID(String),
    UserSet(UserSet),
}

/// ```
/// <userset> ::= <object>'#'<relation>
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserSet {
    pub object: Object,
    pub relation: String,
}

/// This is Data Transfer Object for Query Parameter.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckQueryParam {
    pub tuple: String, // RelationTuple
}

impl TryInto<RelationTuple> for CheckQueryParam {
    type Error = RelationTupleParserError;

    fn try_into(self) -> Result<RelationTuple, Self::Error> {
        RelationTupleParser::parse(&self.tuple)
    }
}
