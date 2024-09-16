use crate::{error, getparameterlist, request, user};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatasetDetails {
    dataset_name: String,
    dataset_description: String,
}

impl DatasetDetails {
    pub async fn parameters(
        &self,
        user: &user::User,
    ) -> Result<getparameterlist::BeaParameters, error::BeaError> {
        let mut body = user.body();
        body.push_str("&method=GETPARAMETERLIST");
        body.push_str(&format!("&datasetname={}", self.dataset_name));
        let client = reqwest::Client::new();
        let res = client.get(body).send().await?;
        info!("Response code: {}.", res.status());
        let data = res.json::<getparameterlist::BeaParameters>().await?;
        Ok(data)
    }

    pub fn name(&self) -> String {
        self.dataset_name.to_owned()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Datasets {
    pub dataset: Vec<DatasetDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatasetResults {
    pub request: request::RequestParameters,
    pub results: Datasets,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct BeaDatasets {
    pub beaapi: DatasetResults,
}

impl BeaDatasets {
    pub async fn get(user: &user::User) -> Result<Self, error::BeaError> {
        let mut body = user.body();
        body.push_str("&method=GETDATASETLIST");
        let client = reqwest::Client::new();
        let res = client.get(body).send().await?;
        info!("Response code: {}.", res.status());
        let data = res.json::<BeaDatasets>().await?;
        Ok(data)
    }

    pub fn results(&self) -> Vec<DatasetDetails> {
        self.beaapi.results.dataset.clone()
    }
}
