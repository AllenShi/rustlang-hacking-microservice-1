#[macro_use] 
extern crate log;
extern crate env_logger;
extern crate actix_web;
extern crate hacking_news_domain;

use std::io::{Result};
use std::env::*;
use actix_web::{App, HttpServer};

pub mod controllers;
use controllers::news_controller::*;

#[actix_rt::main]
async fn main() -> Result<()> {
    set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    info!("Rust Actix Server running... http://localhost:8080/");
    HttpServer::new(||{
        App::new().service(index)
        .service(list_news)
        .service(insert_news)
        .service(get_news_by_id)
        .service(delete_news_by_id)
    }).bind("localhost:8080")?.run().await
}
