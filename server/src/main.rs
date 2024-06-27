mod page;
mod controller;

use dotenv::{dotenv,var};

extern crate lazy_static;

use _db::database::establish_connection;

#[actix_web::main]
async fn main() {
    let ip_addr:String = local_ipaddress:: get().unwrap ();
    println!("start at ip:{}", ip_addr);
    if ip_addr.eq(&"172.24.191.33"){
        dotenv::from_filename(".env").unwrap();
    }
    else{
        dotenv::from_filename("test.env").unwrap();
    }
    dotenv().ok();
    
    //"mysql://username:password@hostname:port/database"
     let _ = establish_connection(
         var("DB_NAME").unwrap().as_str(),
         var("DATABASE_URL").unwrap().as_str()
     ).await
     .unwrap();

    page::index::creat_app().await;
}
