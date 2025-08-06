/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::pg::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type QUOTED_STRING = String;
pub fn quoted_string(_ctx: &Ctx, token: Token) -> QUOTED_STRING {
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
pub type Nodes = Node1;
pub fn nodes_node1(_ctx: &Ctx, node1: Node1) -> Nodes {
    node1
}
pub type Node1 = Vec<Node>;
pub fn node1_c1(_ctx: &Ctx, mut node1: Node1, node: Node) -> Node1 {
    node1.push(node);
    node1
}
pub fn node1_node(_ctx: &Ctx, node: Node) -> Node1 {
    vec![node]
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
pub type LabelSpec = IDENTIFIER1;
pub fn label_spec_identifier1(_ctx: &Ctx, identifier1: IDENTIFIER1) -> LabelSpec {
    identifier1
}
pub type IDENTIFIER1 = Vec<IDENTIFIER>;
pub fn identifier1_c1(
    _ctx: &Ctx,
    mut identifier1: IDENTIFIER1,
    identifier: IDENTIFIER,
) -> IDENTIFIER1 {
    identifier1.push(identifier);
    identifier1
}
pub fn identifier1_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> IDENTIFIER1 {
    vec![identifier]
}
pub type PropertySpec = Properties;
pub fn property_spec_properties(_ctx: &Ctx, properties: Properties) -> PropertySpec {
    properties
}
pub type Properties = Property1;
pub fn properties_property1(_ctx: &Ctx, property1: Property1) -> Properties {
    property1
}
pub type Property1 = Vec<Property>;
pub fn property1_c1(
    _ctx: &Ctx,
    mut property1: Property1,
    property: Property,
) -> Property1 {
    property1.push(property);
    property1
}
pub fn property1_property(_ctx: &Ctx, property: Property) -> Property1 {
    vec![property]
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
pub enum Values {
    SingleValue(SingleValue),
    ListValue(ListValues),
}
pub fn values_single_value(_ctx: &Ctx, single_value: SingleValue) -> Values {
    Values::SingleValue(single_value)
}
pub fn values_list_value(_ctx: &Ctx, list_values: ListValues) -> Values {
    Values::ListValue(list_values)
}
pub type ListValues = SingleValue0;
pub fn list_values_single_value0(_ctx: &Ctx, single_value0: SingleValue0) -> ListValues {
    single_value0
}
pub type SingleValue1 = Vec<SingleValue>;
pub fn single_value1_c1(
    _ctx: &Ctx,
    mut single_value1: SingleValue1,
    single_value: SingleValue,
) -> SingleValue1 {
    single_value1.push(single_value);
    single_value1
}
pub fn single_value1_single_value(
    _ctx: &Ctx,
    single_value: SingleValue,
) -> SingleValue1 {
    vec![single_value]
}
pub type SingleValue0 = Option<SingleValue1>;
pub fn single_value0_single_value1(
    _ctx: &Ctx,
    single_value1: SingleValue1,
) -> SingleValue0 {
    Some(single_value1)
}
pub fn single_value0_empty(_ctx: &Ctx) -> SingleValue0 {
    None
}
#[derive(Debug, Clone)]
pub enum SingleValue {
    StringValue(QUOTED_STRING),
    NumberValue(NUMBER),
}
pub fn single_value_string_value(
    _ctx: &Ctx,
    quoted_string: QUOTED_STRING,
) -> SingleValue {
    SingleValue::StringValue(quoted_string)
}
pub fn single_value_number_value(_ctx: &Ctx, number: NUMBER) -> SingleValue {
    SingleValue::NumberValue(number)
}
