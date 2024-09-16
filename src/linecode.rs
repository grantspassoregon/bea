use crate::{config, error, request};
use serde::{Deserialize, Serialize};
use tracing::info;

pub async fn get_line_codes(config: &config::Config) -> Result<BeaLineCodes, error::BeaError> {
    let mut body = config.body();
    body.push_str("&method=GetParameterValuesFiltered");
    body.push_str("&TargetParameter=LineCode");
    let client = reqwest::Client::new();
    let res = client.get(body).send().await?;
    Ok(res.json::<BeaLineCodes>().await?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineCode {
    key: String,
    desc: String,
}

impl LineCode {
    pub fn report(&self) {
        info!("Key: {}, Desc: {}", self.key, self.desc);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineCodes {
    param_value: Vec<LineCode>,
}

impl LineCodes {
    pub fn report(&self) {
        for code in self.param_value.clone() {
            code.report();
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineCodeResults {
    request: request::RequestParameters,
    results: LineCodes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct BeaLineCodes {
    beaapi: LineCodeResults,
}

impl BeaLineCodes {
    pub fn results(&self) -> LineCodes {
        self.beaapi.results.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineCodeTask {
    key: String,
    processed: bool,
}

impl From<&LineCode> for LineCodeTask {
    fn from(linecode: &LineCode) -> Self {
        LineCodeTask {
            key: linecode.key.clone(),
            processed: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LineCodeTasks {
    tasks: Vec<LineCodeTask>,
}

impl From<&LineCodes> for LineCodeTasks {
    fn from(linecodes: &LineCodes) -> Self {
        let mut tasks = Vec::new();
        for code in linecodes.param_value.clone() {
            tasks.push(LineCodeTask::from(&code));
        }
        LineCodeTasks { tasks }
    }
}
