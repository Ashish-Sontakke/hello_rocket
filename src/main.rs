#![feature(proc_macro_hygiene, decl_macro)]

use rocket::tokio::time::{sleep, Duration};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[get("/healthcheck")]
fn healthcheck() -> &'static str {
    "Healthy"
}

// sample async route
#[get("/async/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, healthcheck, delay])
        .launch()
        .await;

    Ok(())
}
