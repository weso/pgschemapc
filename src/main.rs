mod boolean_expr;
mod card;
mod edge;
mod evidence;
mod formal_base_type;
mod formal_graph_type;
mod key;
mod label_property_spec;
mod node;
mod node_id;
mod property_value_spec;
mod record;
mod record_type;
mod semantics_error;
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
use std::collections::HashSet;

// src/main.rs
fn main() {
    let mut alice_content = record::Record::new();
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
    );
}
