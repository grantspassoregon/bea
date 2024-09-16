use crate::{error, getparametervalues, request, user};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    parameter_name: String,
    #[serde(deserialize_with = "deserialize_bool")]
    multiple_accepted_flag: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    parameter_is_required_flag: bool,
    parameter_data_type: String,
    parameter_description: String,
    parameter_default_value: Option<String>,
}

impl Parameter {
    pub async fn values(
        &self,
        user: &user::User,
        dataset: &str,
    ) -> Result<getparametervalues::BeaParameterValues, error::BeaError> {
        let mut body = user.body();
        body.push_str("&method=GETPARAMETERVALUES");
        body.push_str(&format!("&datasetname={}", dataset));
        body.push_str(&format!("&ParameterName={}", self.parameter_name));
        let client = reqwest::Client::new();
        let res = client.get(body.clone()).send().await?;
        info!("Response: {}", res.text().await?);
        let res = client.get(body).send().await?;
        info!("Response code: {}.", res.status());
        let data = res.json::<getparametervalues::BeaParameterValues>().await?;
        Ok(data)
    }

    pub fn name(&self) -> String {
        self.parameter_name.to_owned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameters {
    parameter: Vec<Parameter>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterList {
    request: request::RequestParameters,
    results: Parameters,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct BeaParameters {
    beaapi: ParameterList,
}

impl BeaParameters {
    pub fn results(&self) -> Vec<Parameter> {
        self.beaapi.results.parameter.clone()
    }
}

pub fn deserialize_bool<'de, D: Deserializer<'de>>(de: D) -> Result<bool, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    match intermediate {
        "1" => Ok(true),
        _ => Ok(false),
    }
}
