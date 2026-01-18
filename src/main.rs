/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct AddQuery {
    a: f64,
    b: f64,
}

async fn add(query: web::Query<AddQuery>) -> Result<String> {
    let sum = query.a + query.b;
    Ok(format!("The sum of {} and {} is {}", query.a, query.b, sum))
}

async fn index() -> Result<String> {
    Ok("Rust API is running. Use /add?a=1&b=2 to add numbers.".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/add", web::get().to(add))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}