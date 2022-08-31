use rocket::form::FromForm;
use serde::Serialize;

#[derive(FromForm, Serialize, Clone, Copy)]
pub struct DisselUsage {
    pub distance: u32,
    pub fuel_per_100km: u32,
    pub year_of_production: u32,
}

impl DisselUsage {
    pub fn calculate(&self) -> f32 {
        let result = self.fuel_per_100km as f32 / self.distance as f32;
        result * 100.0
    }
}

#[derive(Serialize)]
pub struct DisselUsageResponse {
    pub fuel_consumption: f32,
}

impl DisselUsageResponse {
    pub fn new(fuel_consumption: f32) -> DisselUsageResponse {
        DisselUsageResponse { fuel_consumption }
    }
}
