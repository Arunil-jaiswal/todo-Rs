use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatetodoSchema {
    pub title: String,
    pub content: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatetodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,

}