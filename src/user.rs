#[derive(Clone, Debug)]
pub struct User {
    url: String,
    api: String,
}

impl User {
    pub fn new(url: &str, api: &str) -> Self {
        User {
            url: url.to_owned(),
            api: api.to_owned(),
        }
    }

    pub fn body(&self) -> String {
        let mut body = self.url.to_owned();
        body.push_str(&format!("?&UserID={}", self.api));
        body
    }
}
