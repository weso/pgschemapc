use std::collections::HashMap;

use super::pg_actions::Pg;
use rustemo::Parser;

use crate::{
    parser::{
        pg::PgParser,
        pg_actions::{
            LabelPropertySpec, LabelSpec, Nodes, Properties, Property, PropertySpec, SingleValue,
        },
    },
    pg::PropertyGraph,
    pgs_error::PgsError,
    record::Record,
};

pub struct PgBuilder {}

impl PgBuilder {
    pub fn new() -> Self {
        PgBuilder {}
    }
    pub fn parse_pg(&self, input: &str) -> Result<PropertyGraph, PgsError> {
        let pg_content = PgParser::new()
            .parse(input)
            .map_err(|e| PgsError::PGParserError {
                error: e.to_string(),
            })?;
        let nodes = get_nodes(pg_content)?;
        Ok(PropertyGraph::new().with_nodes(nodes))
    }
}

fn get_nodes(nodes: Nodes) -> Result<HashMap<String, Record>, PgsError> {
    match nodes {
        Nodes::EachOf(each_of) => todo!(),
        Nodes::SingleNode(node) => todo!(),
    }
}
