
use _db::database::{update, select};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, web};
use dotenv::var;
use serde_json::*;
use sqlx::Row;


use super::tables::sys_wexin_user_code::*;
use super::tables::traits::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
        .route("/purchase", web::post().to(purchase))
    );
    //println!("connect config finish");
}

//#[post("/purchase")]
async fn purchase(_req_body: String) -> impl Responder {
    let body:Value = serde_json::from_str(_req_body.as_str()).unwrap();
    let obj = body.as_object().unwrap();
    let db_name = var("DB_NAME").unwrap();
    let table_name = "sys_wexin_user_code";

    let select_res = select(
        db_name.as_str(), 
        table_name,
        SysWexinUserCode::default().var_filter(obj.get("filter").unwrap().as_object().unwrap()),
        false
    ).await;
    let sql_res_len = select_res.len();
    if sql_res_len != 1 {
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("'use':'code have {sql_res_len}'}}"));
    }
    let row = select_res.get(0).unwrap();

    let phone: std::result::Result<String, sqlx::Error> = row.try_get("phone");
    match phone {
        Ok(_) => {
            let phone:String = row.get("phone");
            if !phone.eq("") {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("{{'use':'already used by {phone}'}}"));
            }
        },
        Err(_) => {},
    }
    let _ = update(
        db_name.as_str(), 
        table_name,
        row.get("id"), 
        SysWexinUserCode::default().var_paser(obj.get("set").unwrap().as_object().unwrap()).as_str(), 
        false
    ).await;
    
    HttpResponse::Ok().body(format!("{{'use':'{sql_res_len}'}}"));
    return HttpResponse::Ok().body("");
}