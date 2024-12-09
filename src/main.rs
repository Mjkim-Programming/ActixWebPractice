#![warn(unused_imports)]

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// use shuttle_actix_web::ShuttleActixWeb;

struct AppState {
    dev_name: String,
}

struct UserData {
    name: String,
    email: String
}

#[get("/api/")]
async fn hello(data: web::Data<AppState>) -> String {
    let dev_name = &data.dev_name;
    format!("Hello, Actix-Web! This is {dev_name}!")
}

#[get("/api/user")]
async fn user() -> String {
    let john_doe = UserData {
        name: String::from("John Doe"),
        email: String::from("JohnDoe@gmail.com")
    };

    let name = john_doe.name;
    let email = john_doe.email;

    format!("Userdata [name: {name}, email: {email}]")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                dev_name: String::from("GaegulDev")
            }))
            .service(hello)
            .service(user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/* #[shuttle_runtime::main]
async fn actix_web() -> SuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello);
        cfg.service(user);
    };

    Ok(config.into())
} */