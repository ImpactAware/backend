#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate chrono;
extern crate chrono_tz;
extern crate derive_more;

pub mod models;
pub mod schema;
pub mod routes;

use rocket::{response, Response, http::{ContentType, Status}, Request, response::Responder};
use serde::{Deserialize, Serialize};
use std::{
    io::Cursor,
    env,
    error::Error
};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use data_encoding::BASE64;
use data_encoding::HEXLOWER;
use rand::rngs::OsRng;
use rand::RngCore;

pub use diesel::{prelude::*, PgConnection, result::DatabaseErrorKind, result::Error as DieselError};
pub use schema::{
    nodes::dsl as nodesdsl,
};

pub use models::{HasId, RowRef};
pub use rocket::{
    serde::json::Json,
    State
};

pub use derive_more::Display;

pub use chrono::Utc;

pub fn establish_connection() -> PgConnection {
        PgConnection::establish(&env::var("DB_URL").expect("No DB URL Set")).expect(&format!("Error connecting to database"))
}


#[derive(Serialize, Debug)]
pub struct ApiError<E: Sized + Error + Serialize + Into<Status>> {
    status: &'static str,
    error: E
}

impl<R: Serialize + Into<Status> + Sized + Error> From<R> for ApiError<R> {
    fn from(internal: R) -> ApiError<R> {
        ApiError {
            status: "failure",
            error: internal
        }
    }
}

impl<'r, E> Responder<'r, 'static> for ApiError<E> where E: Serialize + Into<Status> + Sized + Error,
{
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let result = Json::respond_to(Json(&self), request);
        match result {
            Ok(mut response) => {
                response.set_status(self.error.into());
                Ok(response)
            }, 
            Err(e) => {
                // serialization error
                Err(e)
            }
        }
    }
}

// this is necessary to maintain backwards compatibility with ApiResponse style endpoints, as they
// may be converted to Result endpoints without requiring frontend changes
#[derive(Serialize, Debug)]
pub struct ApiData<D: Sized + Serialize> {
    status: &'static str,
    data: D
}

impl<'r, D> Responder<'r, 'static> for ApiData<D> where D: Serialize + Sized
{
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        Json::respond_to(Json(&self), request)
    }
}

impl<R: Serialize + Sized> From<R> for ApiData<R> {
    fn from(internal: R) -> ApiData<R> {
        ApiData {
            status: "success",
            data: internal
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    /// Price IDs for stripe subscriptions where the index is the class size
    pub subscription_price_ids: Vec<String>,

    /// Prices in cents for one-time meetings where the index is the class size
    pub onetime_prices_cents: Vec<i64>,

    /// how much in advance meetings must be booked/canceled. Use the presence of a payment where
    /// possible
    pub offset_period_secs: i64,

    /// How long sessions last on our website
    pub cookie_token_life_secs: u64,
    
    /// How long session-generating email tokens last
    pub email_token_life_secs: u64,

    /// Key used to sign email tokens
    pub token_signing_key: String
}

#[derive(Deserialize, Debug)]
pub struct AppState {
    pub config: AppConfig
}
