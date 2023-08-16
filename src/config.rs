use crate::user;

pub struct Config {
    user: user::User,
    dataset: String,
    table: Option<String>,
    geofips: Option<String>,
    linecode: Option<String>,
    year: Option<String>,
}

impl Config {
    pub fn new(user: &user::User, dataset: &str) -> Self {
        Config {
            user: user.clone(),
            dataset: dataset.to_owned(),
            table: None,
            geofips: None,
            linecode: None,
            year: None,
        }
    }

    pub fn set_table(&mut self, table: &str) -> &mut Self {
        self.table = Some(table.to_owned());
        self
    }

    pub fn set_geofips(&mut self, value: &str) -> &mut Self {
        self.geofips = Some(value.to_owned());
        self
    }

    pub fn set_linecode(&mut self, value: &str) -> &mut Self {
        self.linecode = Some(value.to_owned());
        self
    }

    pub fn set_year(&mut self, value: &str) -> &mut Self {
        self.year = Some(value.to_owned());
        self
    }

    pub fn body(&self) -> String {
        let mut body = self.user.body();
        body.push_str(&format!("&datasetname={}", self.dataset));
        if let Some(table) = self.table.clone() {
            body.push_str(&format!("&TableName={}", table));
        }
        if let Some(geofips) = self.geofips.clone() {
            body.push_str(&format!("&GeoFips={}", geofips));
        }
        if let Some(linecode) = self.linecode.clone() {
            body.push_str(&format!("&LineCode={}", linecode));
        }
        if let Some(year) = self.year.clone() {
            body.push_str(&format!("&Year={}", year));
        }
        body
    }
}

