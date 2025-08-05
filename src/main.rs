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
mod validation_result;
mod value;
mod value_type;

use anyhow::*;
use clap::Parser;
use cli::{Cli, Command};
use pgschemapc::{
    formal_graph_type::FormalGraphType,
    parser::{map_builder::MapBuilder, pg_builder::PgBuilder, pgs_builder::PgsBuilder},
};
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
        Some(Command::Validate { graph, schema, map }) => run_validate(graph, schema, map),
        None => {
            bail!("Command not specified, type `--help` to see list of commands")
        }
    }
}

fn run_pgs(schema: &str) -> Result<()> {
    let schema = get_schema(schema)?;
    println!("Property graph schema: {}", schema);
    Ok(())
}

fn run_pg(graph: &str) -> Result<()> {
    let pg = get_graph(graph)?;
    println!("Property graph: {}", pg);
    Ok(())
}

fn run_map(map: &str) -> Result<()> {
    let map = get_map(map)?;
    println!("Type map associations:\n{}", map);
    Ok(())
}

fn run_validate(graph_path: &str, schema_path: &str, map_path: &str) -> Result<()> {
    let schema = get_schema(schema_path)?;
    let graph = get_graph(graph_path)?;
    let map = get_map(map_path)?;
    let result = map.validate(&schema, &graph)?;
    println!("Validation result: {}", result);
    Ok(())
}

fn get_schema(path: &str) -> Result<FormalGraphType> {
    let schema_content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read schema file: {}", path))?;
    let schema: FormalGraphType = match PgsBuilder::new().parse_pgs(schema_content.as_str()) {
        Ok(schema) => schema,
        Err(e) => {
            bail!("Failed to parse schema: {}", e);
        }
    };
    Ok(schema)
}

fn get_graph(path: &str) -> Result<pgschemapc::pg::PropertyGraph> {
    let graph_content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read graph file: {}", path))?;
    let graph = match PgBuilder::new().parse_pg(graph_content.as_str()) {
        Ok(graph) => graph,
        Err(e) => {
            bail!("Failed to parse graph: {}", e);
        }
    };
    Ok(graph)
}

fn get_map(path: &str) -> Result<pgschemapc::type_map::TypeMap> {
    let map_content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read type map file: {}", path))?;
    let map: pgschemapc::type_map::TypeMap = match MapBuilder::new().parse_map(map_content.as_str())
    {
        Ok(map) => map,
        Err(e) => {
            bail!("Failed to parse type map: {}", e);
        }
    };
    Ok(map)
}
