
pub mod database{
    extern crate sqlx;

    use std::collections::{HashMap,HashSet};

    use sqlx::{mysql::{MySqlPoolOptions, self, MySqlRow}, Pool, MySql, Error, query};
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static!{
        static  ref DB_MAP: Mutex<HashMap<String, DbObj>> = {
            let map:HashMap<String, DbObj> = HashMap::new();
            Mutex::new(map)
        };
    }

    struct DbObj{
        db_pool:Pool<MySql>,
        tables:HashSet<String>,
    }

    pub async fn establish_connection(db_name:&str, connection_str:&str) -> Result<(), Error> {
        
        if  DB_MAP.lock().unwrap().contains_key(db_name) {
            return Err(Error::Protocol("db_name already exist".to_string()))
        }

        let db_pool: Pool<MySql> = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&connection_str)
            .await
            .unwrap();

        DB_MAP.lock().unwrap().insert(db_name.to_string(), get_tables(db_pool).await);
        println!("sql succuss sql_name:{db_name},connection_str:{connection_str}");
        
        Ok(())
    }

    pub async fn update(db_name:&str, table_name:&str, id:i32, value:&str, print_query:bool)-> Vec<MySqlRow>{

        let db_map = DB_MAP.lock().unwrap();
        let obj: &DbObj = db_map.get(db_name).unwrap();
        if !have_table(table_name, obj){
            
        }

        let query_str=format!("update {table_name} set {value} where id={id}");
        if print_query {
            println!("table {db_name},query:{query_str}");
        }
        return query_db(&obj.db_pool,query_str.as_str()).await;
    }
    
    pub async fn select(db_name:&str,table_name:&str, map:HashMap<String,String>, print_query:bool)-> Vec<MySqlRow>{

        let db_map = DB_MAP.lock().unwrap();
        let obj: &DbObj = db_map.get(db_name).unwrap();
        if !have_table(table_name, obj){
            
        }

        let mut keys = String::new();
        let mut values = String::new();
        keys.push('(');
        values.push('(');
        for (key, value) in map{
            keys.push_str(format!("{key},").as_str());
            values.push_str(format!("({value}),").as_str());
        }
        keys.pop();
        values.pop();
        keys.push(')');
        values.push(')');

        let query_str:String=format!("select * from {table_name} where {keys} in {values}");
        if print_query {
            println!("table {db_name},query:{query_str}");
        }
        return query_db(&obj.db_pool, query_str.as_str()).await;
    }

    async fn query_db(db_pool:&Pool<MySql>, query_str:&str) -> Vec<MySqlRow>{

        let res_rows: Vec<mysql::MySqlRow> = query(
            query_str
        )
        .fetch_all(db_pool)
        .await
        .unwrap();
        return res_rows;
    }

    async fn get_tables(db_pool:Pool<MySql>) -> DbObj{
        let rows:Vec<MySqlRow> = query(
            "show tables"
        )
        .fetch_all(&db_pool)
        .await
        .unwrap();

        for table in rows.iter(){
            //println!("{:?}",table);
        }

        let tables = HashSet::new();

        return DbObj{ db_pool, tables};
    }
    
    fn have_table(table_name:&str,obj: &DbObj)-> bool{

        for name in obj.tables.iter(){
            if table_name.eq(name){
                return true;
            }
        }
        return true;
    }
}
