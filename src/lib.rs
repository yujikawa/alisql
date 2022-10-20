use std::io::Read;
use std::fs::File;
use regex::Regex;
use minijinja::{Environment, context, Error};
use minijinja::value::Rest;

#[derive(Debug)]
pub struct Table {
    name: String,
}

#[derive(Debug)]
pub struct SQL {
    table: Table,
    path: String,
    query: String,
}


fn ref_q(values: Rest<String>) -> String {
    values.join(".")
}



impl SQL {
    pub fn new(path: String) -> Self {
        let mut f = File::open(&path).expect("file not found");
        let mut query = String::new();
        f.read_to_string(&mut query).expect("something went wrong reading the file");
        SQL {
            table: Table {
                name: "sample".to_string(),
            },
            path: path,
            query: query,
        }
    }

    pub fn get_ref_tables(&self) -> Vec<Table> {
        let mut v:Vec<Table> = Vec::new();        
        let re = Regex::new(r"\{\{\W*ref\(\W*(\w*)\W*(\w*)\W*\)\W*\}\}").unwrap();
        let caps = re.captures_iter(&self.query);
        for cap in caps{
            v.push(Table{ name: vec![&cap[1], &cap[2]].join(".") });
        }
        return v;
    }

    pub fn get_rendered_query(&self) -> String {
        let mut env = Environment::new();
        env.add_function("ref", ref_q);
        env.add_template("sql", &self.query).unwrap();
        let tmpl = env.get_template("sql").unwrap();
        tmpl.render(context!()).unwrap()
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_table() {
        let t = Table {
            name: "users".to_string(),
        };
        assert!(t.name == "users".to_string());
    }
    #[test]
    fn test_create_sql() {
        let s = SQL::new("src/sample_sqls/sample.sql".to_string());
        assert!(s.path == "src/sample_sqls/sample.sql".to_string());
        println!("{:?}", s.query);
    }

    #[test]
    fn test_get_ref() {
        let s = SQL::new("src/sample_sqls/sample.sql".to_string());
        let tables = s.get_ref_tables();
        assert!(tables[0].name == "db.users".to_string());
        assert!(tables[1].name == "db.role".to_string());
    }

    #[test]
    fn test_get_rendered_query() {
        let s = SQL::new("src/sample_sqls/sample.sql".to_string());
        let query:String = s.get_rendered_query();
        println!("{}", &query);
        assert!(query == String::from("
select 
u.*
, r.* 
from db.users as u
left join db.role as r on
u.id = r.user_id
".trim()));
    }
}
