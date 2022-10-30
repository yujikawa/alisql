use minijinja::value::Rest;
use minijinja::{context, Environment};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::vec;

type TableName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub table: TableName,
    pub sql: SQL,
    pub depends_on: Vec<TableName>,
}

impl Table {
    pub fn new(table: TableName, sql: SQL, depends_on: Vec<TableName>) -> Self {
        Table {
            table: table,
            sql: sql,
            depends_on: depends_on,
        }
    }
}
pub trait Analyzer {
    fn new(path: String) -> Self;
    fn get_ref_tables(&self) -> Vec<TableName>;
    fn get_query(&self) -> &String;
    fn get_rendered_query(&self) -> String {
        let mut env = Environment::new();
        let ref_q = |i: Rest<String>| -> String { i.join(".") };
        env.add_function("ref", ref_q);
        let query = self.get_query();
        env.add_template("sql", &query).unwrap();
        let tmpl = env.get_template("sql").unwrap();
        tmpl.render(context!()).unwrap()
    }
    fn is_sql_file(file_name: &str) -> bool {
        let re = Regex::new(r"(\w*)\.sql").unwrap();
        re.is_match(file_name)
    }
    fn get_analized_table(&self) -> Table;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SQL {
    pub path: String,
    pub query: String,
}

impl SQL {
    pub fn new(path: String, query: String) -> Self {
        SQL {
            path: path,
            query: query,
        }
    }
}

#[derive(Debug)]
pub struct RegexSQLAnalyser {
    pub sql: SQL,
}

impl Analyzer for RegexSQLAnalyser {
    fn new(path: String) -> Self {
        let mut f = File::open(&path).expect("file not found");
        let mut query = String::new();
        f.read_to_string(&mut query)
            .expect("something went wrong reading the file");
        RegexSQLAnalyser {
            sql: SQL::new(path, query),
        }
    }

    fn get_ref_tables(&self) -> Vec<TableName> {
        let mut v: Vec<TableName> = Vec::new();
        let re = Regex::new(r"\{\{\W*ref\(\W*(\w*)\W*(\w*)\W*\)\W*\}\}").unwrap();
        let caps = re.captures_iter(&self.sql.query);
        for cap in caps {
            let t = if cap[2].is_empty() {
                vec![&cap[1]]
            } else {
                vec![&cap[1], &cap[2]]
            };
            v.push(t.join("."));
        }
        v
    }

    fn get_query(&self) -> &String {
        &self.sql.query
    }

    fn get_analized_table(&self) -> Table {
        let re = Regex::new(r"(\w*)\.sql").unwrap();
        let caps = re.captures(&self.sql.path).unwrap();
        let depends_on = self.get_ref_tables();
        let table_name = &caps[1];
        Table::new(table_name.to_string(), self.sql.clone(), depends_on)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_table() {
        let t = "users".to_string();
        assert!(t == "users".to_string());
    }
    #[test]
    fn test_create_sql() {
        let s = RegexSQLAnalyser::new("src/sample_sqls/sample.sql".to_string());
        assert!(s.sql.path == "src/sample_sqls/sample.sql".to_string());
        println!("{:?}", s.sql.query);
    }

    #[test]
    fn test_get_ref() {
        let s = RegexSQLAnalyser::new("src/sample_sqls/sample.sql".to_string());
        let tables = s.get_ref_tables();
        assert!(tables[0] == "db.users".to_string());
        assert!(tables[1] == "role".to_string());
    }

    #[test]
    fn test_get_rendered_query() {
        let s = RegexSQLAnalyser::new("src/sample_sqls/sample.sql".to_string());
        let query: String = s.get_rendered_query();
        println!("{}", &query);
        assert!(
            query
                == String::from(
                    "
select 
u.*
, r.* 
from db.users as u
left join role as r on
u.id = r.user_id
"
                    .trim()
                )
        );
    }
}
