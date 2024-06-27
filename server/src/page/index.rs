use actix_web::{HttpServer, App, web, Responder, HttpResponse};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn creat_app(){
    
    println!("add route");
    let _ = HttpServer::new(|| {
        App::new()
        .route("/hello/", web::get().to(manual_hello))
        .configure(crate::controller::account::config)
    })
    .bind("127.0.0.1:17733")
    .expect(&format!("can't bind port"))
    .run()
    .await;
}