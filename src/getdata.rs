use crate::{config, error, getparameterlist, request};
use serde::{Serialize, Deserialize};
use tracing::info;

pub async fn get_data(config: &config::Config) -> Result<BeaDataResponse, error::BeaError> {
    let mut body = config.body();
    body.push_str(&format!("&method=GetData"));
    let client = reqwest::Client::new();
    let res = client
        .get(body)
        .send()
        .await?;
    let res = res.json::<BeaDataResponse>().await?;
    Ok(res)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dimension {
    name: String,
    data_type: String,
    #[serde(deserialize_with="getparameterlist::deserialize_bool")]
    is_value: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dimensions {
    dimensions: Vec<Dimension>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Note {
    note_ref: String,
    note_text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Notes {
    notes: Vec<Note>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Datum {
    code: String,
    geo_fips: String,
    geo_name: String,
    time_period: String,
    description: String,
    #[serde(rename = "CL_UNIT")]
    cl_unit: String,
    #[serde(rename = "UNIT_MULT")]
    unit_mult: String,
    data_value: String,
}

impl Datum {
    pub fn report(&self) {
        info!("Desc: {}, Value: {}", self.description, self.data_value);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    data: Vec<Datum>,
}

impl Data {
    pub fn new(data: &[Datum]) -> Self {
        Data {
            data: data.to_vec(),
        }
    }

    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(title)?;
        for i in self.data.clone() {
            wtr.serialize(i)?;
        }
        wtr.flush()?;
        Ok(())
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataResult {
    statistic: String,
    unit_of_measure: String,
    public_table: String,
    #[serde(rename(deserialize = "UTCProductionTime"))]
    utc_production_time: String,
    note_ref: String,
    dimensions: Vec<Dimension>,
    data: Vec<Datum>,
    notes: Vec<Note>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataResponse {
    request: request::RequestParameters,
    results: DataResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct BeaDataResponse {
    beaapi: DataResponse,
}

impl BeaDataResponse {
    pub fn results(&self) -> Vec<Datum> {
        self.beaapi.results.data.clone()
    }
}
