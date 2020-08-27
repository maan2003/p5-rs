use serde::Deserialize;
use std::collections::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Ast {
    pub literals: HashMap<String, String>,
    pub constants: BTreeMap<String, Vec<Vec<String>>>,
    pub classes: ClassesInfo,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClassesInfo {
    pub files: ItemsWraped<HashMap<String, FileInfo>>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct ItemsWraped<T> {
    pub items: T,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub augmentations: Option<ItemsWraped<HashMap<String, ClassInfo>>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClassInfo {
    pub processed: ProcessedInfo,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProcessedInfo {
    pub instanceMethods: Vec<Method>,
    pub staticMethods: Vec<Method>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Method {
    pub checked: Checked,
    pub name: String,
    pub params: Vec<Param>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Param {
    pub name: String,
    #[serde(rename="type")]
    pub ty: String,
    #[serde(default)]
    pub optional: bool,

}

#[derive(Deserialize, Debug, Clone)]
pub struct Checked {
    pub returnType: Vec<Type>,
    pub classitem: Description,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag="type", content="value")]
#[serde(rename_all="camelCase")]
pub enum Type {
    Basic(String),
    Array(Vec<Type>),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Description {
    pub description: Option<String>,
    pub example: Option<Vec<String>>,
    pub overloads: Option<Vec<OverloadDesc>>,
    pub params: Option<Vec<ParamDesc>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OverloadDesc {
    pub params: Vec<ParamDesc>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ParamDesc {
    pub description: String,
    pub name: String,
    #[serde(default)]
    pub optional: bool,
}