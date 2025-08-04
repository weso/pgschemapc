/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::pgs::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type IDENTIFIER = String;
pub fn identifier(_ctx: &Ctx, token: Token) -> IDENTIFIER {
    token.value.into()
}
pub type Pgs = CreateType;
pub fn pgs_create_type(_ctx: &Ctx, create_type: CreateType) -> Pgs {
    create_type
}
#[derive(Debug, Clone)]
pub enum CreateType {
    CreateNodeType(CreateNodeType),
    CreateEdgeType(CreateEdgeType),
    CreateGraphType(CreateGraphType),
}
pub fn create_type_create_node_type(
    _ctx: &Ctx,
    create_node_type: CreateNodeType,
) -> CreateType {
    CreateType::CreateNodeType(create_node_type)
}
pub fn create_type_create_edge_type(
    _ctx: &Ctx,
    create_edge_type: CreateEdgeType,
) -> CreateType {
    CreateType::CreateEdgeType(create_edge_type)
}
pub fn create_type_create_graph_type(
    _ctx: &Ctx,
    create_graph_type: CreateGraphType,
) -> CreateType {
    CreateType::CreateGraphType(create_graph_type)
}
pub type CreateNodeType = NodeType;
pub fn create_node_type_node_type(_ctx: &Ctx, node_type: NodeType) -> CreateNodeType {
    node_type
}
pub type CreateEdgeType = EdgeType;
pub fn create_edge_type_edge_type(_ctx: &Ctx, edge_type: EdgeType) -> CreateEdgeType {
    edge_type
}
pub type CreateGraphType = GraphType;
pub fn create_graph_type_graph_type(
    _ctx: &Ctx,
    graph_type: GraphType,
) -> CreateGraphType {
    graph_type
}
#[derive(Debug, Clone)]
pub struct NodeType {
    pub type_name: TypeName,
    pub label_property_spec: LabelPropertySpec,
}
pub fn node_type_c1(
    _ctx: &Ctx,
    type_name: TypeName,
    label_property_spec: LabelPropertySpec,
) -> NodeType {
    NodeType {
        type_name,
        label_property_spec,
    }
}
#[derive(Debug, Clone)]
pub struct EdgeType {
    pub endpoint_type_1: EndpointType,
    pub type_name: TypeName,
    pub label_property_spec: LabelPropertySpec,
    pub endpoint_type_6: EndpointType,
}
pub fn edge_type_c1(
    _ctx: &Ctx,
    endpoint_type_1: EndpointType,
    type_name: TypeName,
    label_property_spec: LabelPropertySpec,
    endpoint_type_6: EndpointType,
) -> EdgeType {
    EdgeType {
        endpoint_type_1,
        type_name,
        label_property_spec,
        endpoint_type_6,
    }
}
pub type GraphType = TypeName;
pub fn graph_type_type_name(_ctx: &Ctx, type_name: TypeName) -> GraphType {
    type_name
}
pub type TypeName = IDENTIFIER;
pub fn type_name_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> TypeName {
    identifier
}
pub type EndpointType = LabelPropertySpec;
pub fn endpoint_type_label_property_spec(
    _ctx: &Ctx,
    label_property_spec: LabelPropertySpec,
) -> EndpointType {
    label_property_spec
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
pub struct EachOf {
    pub left: Box<Properties>,
    pub right: Box<Properties>,
}
#[derive(Debug, Clone)]
pub struct OneOf {
    pub left: Box<Properties>,
    pub right: Box<Properties>,
}
#[derive(Debug, Clone)]
pub enum Properties {
    EachOf(EachOf),
    OneOf(OneOf),
    Paren(Box<Properties>),
    Property(Property),
}
pub fn properties_each_of(
    _ctx: &Ctx,
    left: Properties,
    right: Properties,
) -> Properties {
    Properties::EachOf(EachOf {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn properties_one_of(_ctx: &Ctx, left: Properties, right: Properties) -> Properties {
    Properties::OneOf(OneOf {
        left: Box::new(left),
        right: Box::new(right),
    })
}
pub fn properties_paren(_ctx: &Ctx, properties: Properties) -> Properties {
    Properties::Paren(Box::new(properties))
}
pub fn properties_property(_ctx: &Ctx, property: Property) -> Properties {
    Properties::Property(property)
}
#[derive(Debug, Clone)]
pub struct Property {
    pub key: key,
    pub type_spec: TypeSpec,
}
pub fn property_c1(_ctx: &Ctx, key: key, type_spec: TypeSpec) -> Property {
    Property { key, type_spec }
}
pub type key = IDENTIFIER;
pub fn key_identifier(_ctx: &Ctx, identifier: IDENTIFIER) -> key {
    identifier
}
pub type TypeSpec = SimpleType;
pub fn type_spec_simple_type(_ctx: &Ctx, simple_type: SimpleType) -> TypeSpec {
    simple_type
}
#[derive(Debug, Clone)]
pub enum SimpleType {
    STRING_NAME,
    INTEGER_NAME,
    DATE_NAME,
}
pub fn simple_type_string_name(_ctx: &Ctx) -> SimpleType {
    SimpleType::STRING_NAME
}
pub fn simple_type_integer_name(_ctx: &Ctx) -> SimpleType {
    SimpleType::INTEGER_NAME
}
pub fn simple_type_date_name(_ctx: &Ctx) -> SimpleType {
    SimpleType::DATE_NAME
}
