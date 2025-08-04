/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::pg::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type UNQUOTED_STRING = String;
pub fn unquoted_string(_ctx: &Ctx, token: Token) -> UNQUOTED_STRING {
    token.value.into()
}
pub type IDENTIFIER = String;
pub fn identifier(_ctx: &Ctx, token: Token) -> IDENTIFIER {
    token.value.into()
}
pub type NUMBER = String;
pub fn number(_ctx: &Ctx, token: Token) -> NUMBER {
    token.value.into()
}
pub type Pg = Nodes;
pub fn pg_nodes(_ctx: &Ctx, nodes: Nodes) -> Pg {
    nodes
}
#[derive(Debug, Clone)]
pub struct EachOf {
    pub left: Box<Nodes>,
    pub right: Box<Nodes>,
}
#[derive(Debug, Clone)]
pub enum Nodes {
    EachOf(EachOf),
    SingleNode(Node),
}
pub fn nodes_each_of(_ctx: &Ctx, left: Nodes, right: Nodes) -> Nodes {
    Nodes::EachOf(EachOf {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn nodes_single_node(_ctx: &Ctx, node: Node) -> Nodes {
    Nodes::SingleNode(node)
}
#[derive(Debug, Clone)]
pub struct Node {
    pub id: Id,
    pub label_property_spec: LabelPropertySpec,
}
pub fn node_c1(_ctx: &Ctx, id: Id, label_property_spec: LabelPropertySpec) -> Node {
    Node { id, label_property_spec }
}
pub type Id = IDENTIFIER;
pub fn id_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> Id {
    identifier
}
#[derive(Debug, Clone)]
pub struct LabelPropertySpec {
    pub label_spec_opt: LabelSpecOpt,
    pub property_spec_opt: PropertySpecOpt,
}
pub fn label_property_spec_c1(
    _ctx: &Ctx,
    label_spec_opt: LabelSpecOpt,
    property_spec_opt: PropertySpecOpt,
) -> LabelPropertySpec {
    LabelPropertySpec {
        label_spec_opt,
        property_spec_opt,
    }
}
pub type LabelSpecOpt = Option<LabelSpec>;
pub fn label_spec_opt_label_spec(_ctx: &Ctx, label_spec: LabelSpec) -> LabelSpecOpt {
    Some(label_spec)
}
pub fn label_spec_opt_empty(_ctx: &Ctx) -> LabelSpecOpt {
    None
}
pub type PropertySpecOpt = Option<PropertySpec>;
pub fn property_spec_opt_property_spec(
    _ctx: &Ctx,
    property_spec: PropertySpec,
) -> PropertySpecOpt {
    Some(property_spec)
}
pub fn property_spec_opt_empty(_ctx: &Ctx) -> PropertySpecOpt {
    None
}
pub type LabelSpec = IDENTIFIER;
pub fn label_spec_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> LabelSpec {
    identifier
}
pub type PropertySpec = Properties;
pub fn property_spec_properties(_ctx: &Ctx, properties: Properties) -> PropertySpec {
    properties
}
#[derive(Debug, Clone)]
pub struct EachOfProperties {
    pub left: Box<Properties>,
    pub right: Box<Properties>,
}
#[derive(Debug, Clone)]
pub enum Properties {
    EachOfProperties(EachOfProperties),
    BaseProperty(Property),
}
pub fn properties_each_of_properties(
    _ctx: &Ctx,
    left: Properties,
    right: Properties,
) -> Properties {
    Properties::EachOfProperties(EachOfProperties {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn properties_base_property(_ctx: &Ctx, property: Property) -> Properties {
    Properties::BaseProperty(property)
}
#[derive(Debug, Clone)]
pub struct Property {
    pub key: key,
    pub values: Values,
}
pub fn property_c1(_ctx: &Ctx, key: key, values: Values) -> Property {
    Property { key, values }
}
pub type key = IDENTIFIER;
pub fn key_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> key {
    identifier
}
#[derive(Debug, Clone)]
pub struct EachOfValues {
    pub left: Box<Values>,
    pub right: Box<Values>,
}
#[derive(Debug, Clone)]
pub enum Values {
    EachOfValues(EachOfValues),
    SingleValue(SingleValue),
}
pub fn values_each_of_values(_ctx: &Ctx, left: Values, right: Values) -> Values {
    Values::EachOfValues(EachOfValues {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn values_single_value(_ctx: &Ctx, single_value: SingleValue) -> Values {
    Values::SingleValue(single_value)
}
#[derive(Debug, Clone)]
pub enum SingleValue {
    StringValue(UNQUOTED_STRING),
    NumberValue(NUMBER),
}
pub fn single_value_string_value(
    _ctx: &Ctx,
    unquoted_string: UNQUOTED_STRING,
) -> SingleValue {
    SingleValue::StringValue(unquoted_string)
}
pub fn single_value_number_value(_ctx: &Ctx, number: NUMBER) -> SingleValue {
    SingleValue::NumberValue(number)
}
