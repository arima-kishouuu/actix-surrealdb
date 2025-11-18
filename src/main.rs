use actix_web::{ get, App, post , patch, Responder , HttpResponse , HttpServer};

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas available")
}

// #[post("/pizzas")]
// async fn create_pizza() -> impl Responder {
//     HttpResponse::create().body("pizzacreate")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
