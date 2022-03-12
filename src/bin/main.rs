use std::{
    env
};

use backend::*;
use backend::models::*;

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use rocket::{
    request::{FromRequest, Outcome},
    form::Form,
    http::Status,
};

use diesel::result::DatabaseErrorKind;

use serde::Serialize;

use stripe::{WebhookEvent, EventObject, EventType, EventData, InvoiceBillingReason};

use chrono::TimeZone;
//use chrono_tz::Tz;
use chrono::{Utc, NaiveDateTime};

#[rocket::launch]
fn rocket() -> _ {
    if let Err(var_error) = std::env::var("DB_URL") {
        println!("DB URL not set; got error {}", var_error);
        panic!();
    }

    if let Err(var_error) = std::env::var("STRIPE_SECRET") {
        println!("STRIPE_SECRET not set; got error {}", var_error);
        panic!();
    }

    if let Err(var_error) = std::env::var("STRIPE_HOOK_SECRET") {
        println!("STRIPE_HOOK_SECRET not set; got error {}", var_error);
        panic!();
    }

    if let Err(var_error) = std::env::var("MAIL_PASSWORD") {
        println!("MAIL_PASSWORD not set; got error {}", var_error);
        panic!();
    }

    let rocket_b = rocket::build();
    let figment = rocket_b.figment();
    let config = figment.extract().expect("Reading config from default figment");

    println!("config {:?}",config);

    let state = AppState {
        config
    };


    use backend::routes::*;
    rocket_b.mount("/", routes![
                          //misc::get_prices,
                          misc::list_nodes
    ]).manage(state)
}
