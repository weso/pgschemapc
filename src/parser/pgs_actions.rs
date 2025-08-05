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
pub type Number = String;
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.into()
}
pub type Pgs = CreateType1;
pub fn pgs_create_type1(_ctx: &Ctx, create_type1: CreateType1) -> Pgs {
    create_type1
}
pub type CreateType1 = Vec<CreateType>;
pub fn create_type1_c1(
    _ctx: &Ctx,
    mut create_type1: CreateType1,
    create_type: CreateType,
) -> CreateType1 {
    create_type1.push(create_type);
    create_type1
}
pub fn create_type1_create_type(_ctx: &Ctx, create_type: CreateType) -> CreateType1 {
    vec![create_type]
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
pub type LabelSpec = Labels;
pub fn label_spec_labels(_ctx: &Ctx, labels: Labels) -> LabelSpec {
    labels
}
#[derive(Debug, Clone)]
pub struct Labels {
    pub single_label: SingleLabel,
    pub more_labels_opt: MoreLabelsOpt,
}
pub fn labels_c1(
    _ctx: &Ctx,
    single_label: SingleLabel,
    more_labels_opt: MoreLabelsOpt,
) -> Labels {
    Labels {
        single_label,
        more_labels_opt,
    }
}
pub type MoreLabelsOpt = Option<MoreLabels>;
pub fn more_labels_opt_more_labels(
    _ctx: &Ctx,
    more_labels: MoreLabels,
) -> MoreLabelsOpt {
    Some(more_labels)
}
pub fn more_labels_opt_empty(_ctx: &Ctx) -> MoreLabelsOpt {
    None
}
#[derive(Debug, Clone)]
pub struct AndLabels {
    pub single_label: SingleLabel,
    pub more_labels_opt: Box<MoreLabelsOpt>,
}
#[derive(Debug, Clone)]
pub struct OrLabels {
    pub single_label: SingleLabel,
    pub more_labels_opt: Box<MoreLabelsOpt>,
}
#[derive(Debug, Clone)]
pub enum MoreLabels {
    AndLabels(AndLabels),
    OrLabels(OrLabels),
}
pub fn more_labels_and_labels(
    _ctx: &Ctx,
    single_label: SingleLabel,
    more_labels_opt: MoreLabelsOpt,
) -> MoreLabels {
    MoreLabels::AndLabels(AndLabels {
        single_label,
        more_labels_opt: Box::new(more_labels_opt),
    })
}
pub fn more_labels_or_labels(
    _ctx: &Ctx,
    single_label: SingleLabel,
    more_labels_opt: MoreLabelsOpt,
) -> MoreLabels {
    MoreLabels::OrLabels(OrLabels {
        single_label,
        more_labels_opt: Box::new(more_labels_opt),
    })
}
#[derive(Debug, Clone)]
pub enum SingleLabel {
    SingleLabel(IDENTIFIER),
    TypeName(IDENTIFIER),
}
pub fn single_label_single_label(_ctx: &Ctx, identifier: IDENTIFIER) -> SingleLabel {
    SingleLabel::SingleLabel(identifier)
}
pub fn single_label_type_name(_ctx: &Ctx, identifier: IDENTIFIER) -> SingleLabel {
    SingleLabel::TypeName(identifier)
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
pub struct BaseProperty {
    pub optionalopt: OPTIONALOpt,
    pub property: Property,
}
#[derive(Debug, Clone)]
pub enum Properties {
    EachOf(EachOf),
    OneOf(OneOf),
    Paren(Box<Properties>),
    BaseProperty(BaseProperty),
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
pub fn properties_base_property(
    _ctx: &Ctx,
    optionalopt: OPTIONALOpt,
    property: Property,
) -> Properties {
    Properties::BaseProperty(BaseProperty {
        optionalopt,
        property,
    })
}
pub type OPTIONALOpt = Option<OptionalOptNoO>;
#[derive(Debug, Clone)]
pub enum OptionalOptNoO {
    OPTIONAL,
}
pub fn optionalopt_optional(_ctx: &Ctx) -> OPTIONALOpt {
    Some(OptionalOptNoO::OPTIONAL)
}
pub fn optionalopt_empty(_ctx: &Ctx) -> OPTIONALOpt {
    None
}
#[derive(Debug, Clone)]
pub struct Property {
    pub key: key,
    pub type_spec: TypeSpec,
    pub card_opt: CardOpt,
}
pub fn property_c1(
    _ctx: &Ctx,
    key: key,
    type_spec: TypeSpec,
    card_opt: CardOpt,
) -> Property {
    Property {
        key,
        type_spec,
        card_opt,
    }
}
pub type CardOpt = Option<Card>;
pub fn card_opt_card(_ctx: &Ctx, card: Card) -> CardOpt {
    Some(card)
}
pub fn card_opt_empty(_ctx: &Ctx) -> CardOpt {
    None
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
#[derive(Debug, Clone)]
pub struct Range {
    pub number: Number,
    pub max: Max,
}
#[derive(Debug, Clone)]
pub enum Card {
    Optional,
    OneOrMore,
    ZeroOrMore,
    Range(Range),
}
pub fn card_optional(_ctx: &Ctx) -> Card {
    Card::Optional
}
pub fn card_one_or_more(_ctx: &Ctx) -> Card {
    Card::OneOrMore
}
pub fn card_zero_or_more(_ctx: &Ctx) -> Card {
    Card::ZeroOrMore
}
pub fn card_range(_ctx: &Ctx, number: Number, max: Max) -> Card {
    Card::Range(Range { number, max })
}
#[derive(Debug, Clone)]
pub enum Max {
    Number(Number),
    Star,
}
pub fn max_number(_ctx: &Ctx, number: Number) -> Max {
    Max::Number(number)
}
pub fn max_star(_ctx: &Ctx) -> Max {
    Max::Star
}
