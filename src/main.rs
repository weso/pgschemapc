mod boolean_expr;
mod card;
mod edge;
mod formal_base_type;
mod formal_graph_type;
mod label_property_spec;
mod node;
mod node_id;
mod property_value_spec;
mod record;
mod record_type;
mod semantics_error;
mod value_type;

use crate::{
    formal_graph_type::FormalGraphType,
    label_property_spec::LabelPropertySpec,
    node::Node,
    node_id::NodeId,
    property_value_spec::{PropertyValue, PropertyValueSpec, TypeSpec},
    record::{Key, Value},
    value_type::ValueType,
};
use std::collections::HashSet;

pub type Name = String;
pub type LabelName = String;
pub type TypeName = String;

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
    let name = PropertyValue::property(Key::new("name"), TypeSpec::string());
    let age = PropertyValue::property(Key::new("age"), TypeSpec::integer());
    let aliases = PropertyValue::property(
        Key::new("aliases"),
        TypeSpec::zero_or_more(TypeSpec::string()),
    );
    let person_content = PropertyValue::each_of(name, PropertyValue::each_of(age, aliases));
    graph.add(
        "PersonType".to_string(),
        label_property_spec::LabelPropertySpec::content(
            person_label,
            PropertyValueSpec::closed(person_content),
        ),
    );

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
