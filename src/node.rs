use std::{collections::HashSet, fmt::Display};

use crate::{LabelName, node_id::NodeId, record::Record};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: NodeId,
    pub label: HashSet<LabelName>,
    pub properties: Record,
}

impl Node {
    pub fn new(id: u32) -> Self {
        Node {
            id: NodeId::new(id),
            label: HashSet::new(),
            properties: Record::new(),
        }
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label.insert(label.to_string());
        self
    }

    pub fn with_content(mut self, content: &Record) -> Self {
        self.properties = content.clone();
        self
    }

    pub fn labels(&self) -> &HashSet<LabelName> {
        &self.label
    }

    pub fn content(&self) -> &Record {
        &self.properties
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node({} {:?} [{:?}]",
            self.id, self.label, self.properties
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        card::Card,
        formal_graph_type::FormalGraphType,
        key::Key,
        label_property_spec::LabelPropertySpec,
        property_value_spec::{PropertyValue, PropertyValueSpec, TypeSpec},
        value::Value,
    };

    use super::*;

    #[test]
    fn test_simple_record_alice() {
        let mut alice_content = Record::new();
        alice_content.insert(Key::new("name"), Value::str("Alice"));
        alice_content.insert(Key::new("age"), Value::int(42));
        let alice = Node::new(1)
            .with_label("Person")
            .with_content(&alice_content);

        let mut graph = FormalGraphType::new();
        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let age = PropertyValue::property(Key::new("age"), TypeSpec::integer(Card::One));
        let aliases =
            PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
        let person_content = PropertyValue::each_of(name, PropertyValue::each_of(age, aliases));
        graph.add(
            "PersonType".to_string(),
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );
        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &alice),
            Ok(true)
        )
    }

    #[test]
    fn test_simple_record_alice_wrong() {
        let alice = Node::new(1).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::str("other")),
        );

        let mut graph = FormalGraphType::new();
        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let age = PropertyValue::property(Key::new("age"), TypeSpec::integer(Card::One));
        let aliases =
            PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
        let person_content = PropertyValue::each_of(name, PropertyValue::each_of(age, aliases));
        graph.add(
            "PersonType".to_string(),
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );
        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &alice),
            Ok(false)
        )
    }

    #[test]
    fn test_simple_record_alice_optional_age_ok() {
        let alice = Node::new(1).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::int(42)),
        );

        let bob = Node::new(1)
            .with_label("Person")
            .with_content(&Record::new().with_key_value("name", Value::str("Bob")));

        let mut graph = FormalGraphType::new();
        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let age = PropertyValue::optional_property(Key::new("age"), TypeSpec::integer(Card::One));
        let aliases =
            PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
        let person_content = PropertyValue::each_of(name, PropertyValue::each_of(age, aliases));
        graph.add(
            "PersonType".to_string(),
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );
        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &alice),
            Ok(true)
        );
        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &bob),
            Ok(true)
        )
    }

    #[test]
    fn test_simple_record_bob() {
        let mut bob_content = Record::new();
        bob_content.insert(Key::new("first_name"), Value::str("Robert"));
        bob_content.insert(Key::new("last_name"), Value::str("Smith"));
        let aliases = [Value::str("Bob"), Value::str("Bobby")]
            .into_iter()
            .collect::<HashSet<_>>();
        bob_content.insert_values(Key::new("aliases"), aliases);
        let bob = Node::new(2).with_label("Person").with_content(&bob_content);
        let mut graph = FormalGraphType::new();
        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let age = PropertyValue::property(Key::new("age"), TypeSpec::integer(Card::One));
        let aliases =
            PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
        let person_content = PropertyValue::each_of(name, PropertyValue::each_of(age, aliases));
        graph.add(
            "PersonType".to_string(),
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );

        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &bob),
            Ok(true)
        )
    }
}
