use crate::request;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterFields {
    desc: String,
    key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterValues {
    param_value: Vec<ParameterFields>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BeaParameterValues {
    request: request::RequestParameters,
    results: ParameterValues,
}

