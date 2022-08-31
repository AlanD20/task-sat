mod dissel_usage;
mod response_json;
mod unit_injector_fail;

#[macro_use]
extern crate rocket;

use dissel_usage::*;
use rand::Rng;
use response_json::*;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use unit_injector_fail::*;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(
        r"<h1>Hello! Please visit one of the following api: </h1>
    <div><a href='/api/calculateDisselUsageForDistance'>Calculate Dissel Usage For Distance</a></div>
    <div><a href='/api/probabilityOfUnitInjectorFail'>Probability of Unit Injector Failure</a></div>",
    )
}

#[get("/api/calculateDisselUsageForDistance?<dissel_usage..>")]
fn calculate_dissel_usage_for_distance(
    dissel_usage: Option<DisselUsage>,
) -> Result<Json<Response<DisselUsage, DisselUsageResponse>>, Json<ErrorResponse>> {
    match dissel_usage {
        Some(dissel_usage) => {
          let result = dissel_usage.calculate();

          Ok(Response::json(
              "success",
              DisselUsage::from(dissel_usage),
              DisselUsageResponse::new(result)
            )
          )
        }
        None => Err(ErrorResponse::new(
          "The following parameters are required: distance: positive integers, fuel_per_100km: positive integers, year_of_production, positive Integers. Example: ?distance=1450&fuel_per_100km=120&year_of_production=2021",
          "failed"
        )
        ),
    }
}

#[get("/api/probabilityOfUnitInjectorFail?<injector_fail..>")]
fn probability_of_unit_injector_fail(
    injector_fail: Option<UnitInjectorFail>,
) -> Result<Json<Response<UnitInjectorFail, UnitInjectorFailResponse>>, Json<ErrorResponse>> {
    match injector_fail {
        Some(unit_injector) => {
            let rng = rand::thread_rng().gen_range(0.0..1.0);

            Ok(Response::json(
                "success",
                UnitInjectorFail::from(unit_injector),
                UnitInjectorFailResponse::new(rng),
            ))
        }
        None => Err(ErrorResponse::new(
            "The following parameters are required: vin: positive integers. Example: ?vin=ABCDEF01234",
            "failed",
        )),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            calculate_dissel_usage_for_distance,
            probability_of_unit_injector_fail
        ],
    )
}
