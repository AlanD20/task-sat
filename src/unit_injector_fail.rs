use rocket::form::FromForm;
use serde::Serialize;

#[derive(FromForm, Serialize)]
pub struct UnitInjectorFail {
    pub vin: String,
}

#[derive(Serialize)]
pub struct UnitInjectorFailResponse {
    pub fail_probability: f32,
}

impl UnitInjectorFailResponse {
    pub fn new(fail_probability: f32) -> UnitInjectorFailResponse {
        UnitInjectorFailResponse {
            fail_probability: format!("{:.2}", fail_probability).parse().unwrap_or(0.0),
        }
    }
}
