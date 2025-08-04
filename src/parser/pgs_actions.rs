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
pub type LabelPropertySpec = IDENTIFIER;
pub fn label_property_spec_identifier(
    _ctx: &Ctx,
    identifier: IDENTIFIER,
) -> LabelPropertySpec {
    identifier
}
