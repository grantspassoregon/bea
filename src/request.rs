use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestParameter {
    parameter_name: String,
    parameter_value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestParameters {
    request_param: Vec<RequestParameter>,
}
