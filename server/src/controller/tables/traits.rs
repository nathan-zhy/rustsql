use std::collections::HashMap;

use serde_json::*;

pub trait Reflection {
    fn var_filter(&mut self, map:&Map<String, Value>) ->HashMap<String,String>;
    fn var_paser(&mut self, map:&Map<String, Value>) ->String;
}

impl <T> Reflection for T where T:Default{
    fn var_filter(&mut self, map:&Map<String, Value>)->HashMap<String,String>{
        let mut res: HashMap<String, String> = HashMap::new();
        for (key, value) in map {
            res.insert(key.to_string(), value.to_string());
        }

        return res;
    }
    fn var_paser(&mut self, map:&Map<String, Value>)->String{
        let mut res = String::new();
        for (key, value) in map {
            let valstr = value.as_str().unwrap();
            res.push_str(format!("{key}='{valstr}',").as_str());
        }
        res.pop();
        return res;
    }
}