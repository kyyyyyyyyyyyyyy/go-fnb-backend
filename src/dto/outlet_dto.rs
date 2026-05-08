use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateOutletDTO {
    pub name: String,
    pub address_line: String,
    pub city: String,
    pub province: String,
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}



#[derive(Deserialize)]
pub struct UpdateOutletDTO {
    pub name: Option<String>,
    pub address_line: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub postal_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

