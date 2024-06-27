
pub mod database{
    extern crate sqlx;

    use std::collections::HashMap;

    use sqlx::{mysql::{MySqlPoolOptions, self}, Pool, MySql, Error, query};
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static!{
        static  ref DB_MAP: Mutex<HashMap<String, Pool<MySql>>> = {
            let map:HashMap<String, Pool<MySql>> = HashMap::new();
            Mutex::new(map)
        };
    }
    pub async fn establish_connection(db_name:&str, connection_str:&str) -> Result<*const Pool<MySql>, Error> {
        
        if  DB_MAP.lock().unwrap().contains_key(db_name) {
            return Err(Error::Protocol("db_name already exist".to_string()))
        }

        let db_pool: Pool<MySql> = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&connection_str)
            .await
            .unwrap();

        DB_MAP.lock().unwrap().insert(db_name.to_string(), db_pool);
        println!("sql succuss sql_name:{db_name},connection_str:{connection_str}");
        Ok(DB_MAP.lock().unwrap().get(db_name).unwrap())
    }

    async fn query_db(db_name:&str, query_str:&str, print_query:bool) -> Vec<mysql::MySqlRow>{
        if print_query {
            println!("table {db_name},query:{query_str}");
        }

        let res_rows: Vec<mysql::MySqlRow> = query(
            query_str
        )
        .fetch_all(DB_MAP.lock().unwrap().get(db_name).unwrap())
        .await
        .unwrap();
        return res_rows;
    }
}