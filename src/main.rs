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
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .route("/add", web::get().to(add))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}