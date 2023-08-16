use crate::{config, error, request};
use serde::{Serialize, Deserialize};
use tracing::info;

pub async fn get_geofips(config: &config::Config) -> Result<BeaGeoFips, error::BeaError> {
    let mut body = config.body();
    body.push_str(&format!("&method=GetParameterValuesFiltered"));
    body.push_str(&format!("&TargetParameter=GeoFips"));
    let client = reqwest::Client::new();
    let res = client
        .get(body)
        .send()
        .await?;
    Ok(res.json::<BeaGeoFips>().await?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeoFipsItem {
    key: String,
    desc: String,
}

impl GeoFipsItem {
    pub fn report(&self) {
        info!("Key: {}, Desc: {}", self.key, self.desc);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeoFips {
    param_value: Vec<GeoFipsItem>
}

impl GeoFips {
    pub fn report(&self) {
        for code in self.param_value.clone() {
            code.report();
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeoFipsResults {
    request: request::RequestParameters,
    results: GeoFips,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct BeaGeoFips {
    beaapi: GeoFipsResults,
}

impl BeaGeoFips {
    pub fn results(&self) -> GeoFips {
        self.beaapi.results.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeoFipsTask {
    key: String,
    processed: bool,
}

impl GeoFipsTask {
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn processed(&self) -> bool {
        self.processed
    }
    
    pub fn set_processed(&mut self, value: bool) {
        self.processed = value;
    }

    pub fn report(&self) {
        info!("Key: {}, Processed: {}", self.key, self.processed);
    }
}

impl From<&GeoFipsItem> for GeoFipsTask {
    fn from(geofips: &GeoFipsItem) -> Self {
        GeoFipsTask {
            key: geofips.key.clone(),
            processed: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeoFipsTasks {
    tasks: Vec<GeoFipsTask>,
}

impl GeoFipsTasks {
    pub fn tasks(&self) -> Vec<GeoFipsTask> {
        self.tasks.clone()
    }

    pub fn tasks_mut(&mut self) -> &mut Vec<GeoFipsTask> {
        &mut self.tasks
    }

    pub fn report(&self) {
        for task in self.tasks.clone() {
            task.report();
        }
    }
}

impl From<&GeoFips> for GeoFipsTasks {
    fn from(geofips: &GeoFips) -> Self {
        let mut tasks = Vec::new();
        for code in geofips.param_value.clone() {
            tasks.push(GeoFipsTask::from(&code));
        }
        GeoFipsTasks {
            tasks
        }
    }
}
