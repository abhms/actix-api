// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


// struct AllData{
//     id:String
// }

// #[get("/")]
// async fn hello(req_query: String) ->  String {
//     let id: &String = &req_query; 
//     println!("query{}",id);
//     format!("Hello {}!", id)
    
//     // HttpResponse::Ok().body("Hello world! {req_query}")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
 

// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

mod api;
mod allmodel;
mod repository;

//modify imports below
use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, get_user,update_user,delete_user};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    println!("hello bro");
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

