#[derive(Debug, PartialEq)]
pub struct SourceUnit(pub Vec<SourceUnitPart>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourceUnitPart {
    Architecture(Architecture),
    ConceptUnit(ConceptUnit),
    ContractUnit(ContractUnit),
    ApiUnit(ApiUnit),
    ConceptBy(ConceptBy),
    StructFor(StructFor),
    BehaviorFor(BehaviorFor),
    ConceptSpace(ConceptSpace),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Architecture {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptBy {
    pub cataloging: Cataloging,
    pub path: String,
}

impl ConceptBy {
    pub fn new() -> ConceptBy {
        ConceptBy {
            cataloging: Cataloging::File,
            path: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Cataloging {
    File,
    Dir,
}

impl Cataloging {
    pub fn from(text: String) -> Cataloging {
        match text.as_str() {
            "file" => {
                Cataloging::File
            }
            "dir" => {
                Cataloging::Dir
            }
            _ => {
                Cataloging::File
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptSpace {
    pub identifier: String,
    pub package: String,
    pub concepts: Vec<String>,
}

impl ConceptSpace {
    pub fn new() -> ConceptSpace {
        ConceptSpace {
            identifier: "".to_string(),
            package: "".to_string(),
            concepts: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiUnit {
    pub name: String,
    pub import: Option<String>,
    pub apis: Vec<ApiNode>,
}

impl ApiUnit {
    pub fn new() -> ApiUnit {
        ApiUnit {
            name: "".to_string(),
            import: None,
            apis: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiNode {
    pub inbound: Vec<StructField>,
    pub outbound: Vec<StructField>,
    pub pre_cond: Vec<Condition>,
    pub post_cond: Vec<Condition>,
}

impl ApiNode {
    pub fn new() -> ApiNode {
        ApiNode {
            inbound: vec![],
            outbound: vec![],
            pre_cond: vec![],
            post_cond: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    pub text: String,
    pub expr: Expression,
}

impl Condition {
    pub fn new() -> Condition {
        Condition {
            text: "".to_string(),
            expr: Expression::String(String::from("")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    String(String)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConceptUnit {
    pub description: String,
    pub identifier: String,
    pub extends: Vec<String>,
    pub structs: Vec<StructField>,
    pub behaviors: Vec<Interface>,
    pub struct_source: StructSource
}

impl ConceptUnit {
    pub fn new() -> ConceptUnit {
        ConceptUnit { description: "".to_string(), identifier: "".to_string(), extends: vec![], structs: vec![], behaviors: vec![], struct_source: Default::default() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructSource {
    type_type: String,
    path: String,
    source: String,
}

impl Default for StructSource {
    fn default() -> Self {
        StructSource {
            type_type: "".to_string(),
            path: "".to_string(),
            source: "".to_string()
        }
    }
}

// naming refs: https://github.com/vickenty/lang-c/blob/master/grammar.rustpeg
// naming refs: https://github.com/vickenty/lang-c
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructFor {
    pub identifier: String,
    pub declarations: Vec<StructField>,
}

impl StructFor {
    pub fn new() -> StructFor {
        StructFor {
            identifier: "".to_string(),
            declarations: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructField {
    pub identifier: String,
    pub declarator: TypeSpecifier,
}

impl StructField {
    pub fn new() -> StructField {
        StructField {
            identifier: "".to_string(),
            declarator: TypeSpecifier::TypeType(String::from("")),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BehaviorFor {
    identifier: String,
    functions: Vec<Interface>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interface {
    pub identifier: String,
    pub params: Vec<Parameter>,
    pub return_type: TypeSpecifier,
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            identifier: "".to_string(),
            params: vec![],
            return_type: TypeSpecifier::None,
        }
    }
}

/// auto insert/update comment to code
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HighlightCore {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeSpecifier {
    /// no return or empty
    None,
    Int,
    Float,
    Double,
    String,
    Array,
    TypeType(String),
}

impl TypeSpecifier {
    pub fn from(text: String) -> TypeSpecifier {
        match text.to_lowercase().as_str() {
            "int" => {
                TypeSpecifier::Int
            }
            "float" => {
                TypeSpecifier::Float
            }
            "double" => {
                TypeSpecifier::Double
            }
            "string" => {
                TypeSpecifier::String
            }
            "array" => {
                TypeSpecifier::Array
            }
            _ => {
                TypeSpecifier::TypeType(text)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiResource {
    identifier: String,
    base_url: String,
    source: String,
    api: Vec<ApiDecl>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiDecl {
    inbound: Vec<Parameter>,
    outbound: Vec<Parameter>,
    pre_condition: String,
    post_condition: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Parameter {
    pub identifier: String,
    pub specifier: TypeSpecifier,
}

impl Parameter {
    pub fn new() -> Parameter {
        Parameter {
            identifier: "".to_string(),
            specifier: TypeSpecifier::None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractUnit {
    pub identifier: String,
    pub during: String,
    pub pre_condition: Vec<Condition>,
    pub post_condition: Vec<Condition>,
}

impl ContractUnit {
    pub fn new() -> ContractUnit {
        ContractUnit {
            identifier: "".to_string(),
            during: "".to_string(),
            pre_condition: vec![],
            post_condition: vec![],
        }
    }
}

