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
mod pgs_error;
mod property_value_spec;
mod record;
mod record_type;
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
use pgschemapc::parser::{pg_builder::PgBuilder, pgs::PgsParser};
use rustemo::Parser as RustEmoParser;
use std::collections::HashSet;
use std::result::Result::Ok;

// src/main.rs
fn main() -> Result<()> {
    /*let mut alice_content = record::Record::new();
    alice_content.insert(Key::new("name"), Value::str("Alice"));
    alice_content.insert(Key::new("age"), Value::int(42));
    let alice = Node::new(1)
        .with_label("Person")
        .with_content(&alice_content);

    let mut bob_content = record::Record::new();
    bob_content.insert(Key::new("first_name"), Value::str("Robert"));
    bob_content.insert(Key::new("last_name"), Value::str("Smith"));
    let aliases = [Value::str("Bob"), Value::str("Bobby")]
        .into_iter()
        .collect::<HashSet<_>>();
    bob_content.insert_values(Key::new("aliases"), aliases);
    let bob = Node::new(2).with_label("Person").with_content(&bob_content);

    let mut graph = FormalGraphType::new();
    let person_label = LabelPropertySpec::Label("Person".to_string());
    let name = PropertyValue::property(Key::new("name"), TypeSpec::string(card::Card::One));
    let age = PropertyValue::property(Key::new("age"), TypeSpec::integer(card::Card::One));
    let aliases = PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
    let person_content = PropertyValue::each_of(name, PropertyValue::each_of(age, aliases));
    graph.add(
        "PersonType".to_string(),
        label_property_spec::LabelPropertySpec::content(
            person_label,
            PropertyValueSpec::closed(person_content),
        ),
    );

    let label_property_spec = graph.get("PersonType").unwrap();
    let semantics = label_property_spec.semantics(&graph).unwrap();
    println!("Formal Base Type: {:?}", semantics);

    println!("Alice: {:?}", alice);
    println!("Bob: {:?}", bob);
    // println!("Graph Type: {:?}", graph);
    println!(
        "Conforms Alice to PersonType: {:?}",
        graph.conforms_node(&"PersonType".to_string(), &alice)
    );
    println!(
        "Conforms Bob to PersonType: {:?}",
        graph.conforms_node(&"PersonType".to_string(), &bob)
    );*/

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let cli = Cli::parse_from(args);

    match &cli.command {
        Some(Command::Pgs { schema }) => run_pgs(schema),
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
    match PgBuilder::new().parse_pgs(schema_content.as_str()) {
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
