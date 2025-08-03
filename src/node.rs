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
    use tracing::debug;
    use tracing_test::traced_test;

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
    fn test_simple_record_alice_non_optional() {
        let alice = Node::new(1).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::int(42))
                .with_key_value("aliases", Value::str("Ally")),
        );

        // Wrong label
        let alice_wrong1 = Node::new(1).with_label("Other").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::int(42))
                .with_key_value("aliases", Value::str("Ally")),
        );

        // Wrong age type
        let alice_wrong2 = Node::new(1).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("age", Value::str("Other"))
                .with_key_value("aliases", Value::str("Ally")),
        );

        // No age
        let alice_wrong3 = Node::new(1).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("aliases", Value::str("Ally")),
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

        let property_value_spec = graph.get("PersonType").unwrap();
        let semantics = property_value_spec.semantics(&graph).unwrap();
        debug!("Semantics of person type: {:?}", semantics);

        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &alice),
            Ok(true)
        );
        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &alice_wrong1),
            Ok(false)
        );
        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &alice_wrong2),
            Ok(false)
        );
        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &alice_wrong3),
            Ok(false)
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
                .with_key_value("age", Value::int(42))
                .with_key_value("aliases", Value::str("Ally")),
        );

        let bob = Node::new(1).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Bob"))
                .with_key_value("aliases", Value::str("Bobby")),
        );

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

    #[traced_test]
    #[test]
    fn test_each_of_one_of() {
        let alice = Node::new(2).with_label("Person").with_content(
            &Record::new()
                .with_key_value("name", Value::str("Alice"))
                .with_key_value("aliases", Value::str("Ally")),
        );

        let bob = Node::new(2).with_label("Person").with_content(
            &Record::new()
                .with_key_value("first_name", Value::str("Robert"))
                .with_key_value("last_name", Value::str("Smith"))
                .with_key_value("aliases", Value::str("Bob"))
                .with_key_value("aliases", Value::str("Bobby")),
        );

        let wrong1 = Node::new(2).with_label("Person").with_content(
            &Record::new()
                .with_key_value("first_name", Value::str("Robert"))
                .with_key_value("name", Value::str("extra_name"))
                .with_key_value("aliases", Value::str("Bob"))
                .with_key_value("aliases", Value::str("Bobby")),
        );

        let person_label = LabelPropertySpec::Label("Person".to_string());
        let name = PropertyValue::property(Key::new("name"), TypeSpec::string(Card::One));
        let first_name =
            PropertyValue::property(Key::new("first_name"), TypeSpec::string(Card::One));
        let last_name = PropertyValue::property(Key::new("last_name"), TypeSpec::string(Card::One));
        let age = PropertyValue::optional_property(Key::new("age"), TypeSpec::integer(Card::One));
        let aliases =
            PropertyValue::property(Key::new("aliases"), TypeSpec::string(Card::ZeroOrMore));
        let person_content = PropertyValue::each_of(
            PropertyValue::one_of(name, PropertyValue::each_of(first_name, last_name)),
            PropertyValue::each_of(age, aliases),
        );
        let graph = FormalGraphType::new().with_type_name(
            "PersonType",
            LabelPropertySpec::content(person_label, PropertyValueSpec::closed(person_content)),
        );

        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &alice),
            Ok(true)
        );

        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &bob),
            Ok(true)
        );

        assert_eq!(
            graph.conforms_node(&"PersonType".to_string(), &wrong1),
            Ok(false)
        );
    }
}
