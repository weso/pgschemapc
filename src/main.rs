mod boolean_expr;
mod card;
mod cli;
mod edge;
mod evidence;
mod formal_base_type;
mod formal_graph_type;
mod key;
mod label_property_spec;
mod node;
mod node_id;
mod pg;
mod pgs_error;
mod property_value_spec;
mod record;
mod record_type;
mod type_map;
mod type_name;
mod value;
mod value_type;

use crate::{
    card::Card,
    formal_graph_type::FormalGraphType,
    key::Key,
    label_property_spec::LabelPropertySpec,
    node::Node,
    node_id::NodeId,
    property_value_spec::{PropertyValue, PropertyValueSpec, TypeSpec},
    value::Value,
    value_type::ValueType,
};
use anyhow::*;
use clap::Parser;
use cli::{Cli, Command};
use pgschemapc::parser::{
    map_builder::MapBuilder, pg_builder::PgBuilder, pgs::PgsParser, pgs_builder::PgsBuilder,
};
use rustemo::Parser as RustEmoParser;
use std::collections::HashSet;
use std::result::Result::Ok;

// src/main.rs
fn main() -> Result<()> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let cli = Cli::parse_from(args);

    match &cli.command {
        Some(Command::Pgs { schema }) => run_pgs(schema),
        Some(Command::Pg { graph }) => run_pg(graph),
        Some(Command::TypeMap { map }) => run_map(map),
        Some(Command::Validate { graph, schema, map }) => run_validate(graph, schema),
        None => {
            bail!("Command not specified, type `--help` to see list of commands")
        }
    }
}

fn run_pgs(schema: &str) -> Result<()> {
    // Load the schema file:
    let schema_content = std::fs::read_to_string(schema)
        .with_context(|| format!("Failed to read schema file: {}", schema))?;

    // Parse the schema:
    match PgsBuilder::new().parse_pgs(schema_content.as_str()) {
        Ok(graph_type) => {
            // Successfully parsed the schema, now we can work with `graph_type`.
            println!("Parsed graph type: {}", graph_type);
            Ok(())
        }
        Err(e) => {
            bail!("Failed to parse schema: {}", e);
        }
    }
}

fn run_pg(graph: &str) -> Result<()> {
    let graph_content = std::fs::read_to_string(graph)
        .with_context(|| format!("Failed to read graph file: {}", graph))?;
    let pg = PgBuilder::new()
        .parse_pg(graph_content.as_str())
        .with_context(|| format!("Failed to parse graph: {}", graph))?;
    println!("Property graph: {}", pg);
    Ok(())
}

fn run_map(map: &str) -> Result<()> {
    let map_content = std::fs::read_to_string(map)
        .with_context(|| format!("Failed to read map file: {}", map))?;
    let map = MapBuilder::new()
        .parse_map(map_content.as_str())
        .with_context(|| format!("Failed to parse map: {}", map))?;
    println!("Type map associations: {}", map);
    Ok(())
}

fn run_validate(graph: &str, schema: &str) -> Result<()> {
    let schema_content = std::fs::read_to_string(schema)
        .with_context(|| format!("Failed to read schema file: {}", schema))?;
    let graph_content = std::fs::read_to_string(graph)
        .with_context(|| format!("Failed to read graph file: {}", graph))?;
    bail!(
        "Validation requires PG parser which is not implemented yet. The test cases check validation for parsed structures."
    );
}
