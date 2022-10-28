use minijinja::value::Rest;
use minijinja::{context, Environment};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::vec;
use walkdir::WalkDir;

type TableName = String;

fn ref_q(values: Rest<String>) -> String {
    values.join(".")
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Table {
    table: TableName,
    sql: SQL,
    depends_on: Vec<TableName>,
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
/// Get dependencies from SQL written ref macro with Jinja

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SQL {
    path: String,
    query: String,
}

impl SQL {
    pub fn new(path: String) -> Self {
        let mut f = File::open(&path).expect("file not found");
        let mut query = String::new();
        f.read_to_string(&mut query)
            .expect("something went wrong reading the file");
        SQL {
            path: path,
            query: query,
        }
    }

    pub fn get_ref_tables(&self) -> Vec<TableName> {
        let mut v: Vec<TableName> = Vec::new();
        let re = Regex::new(r"\{\{\W*ref\(\W*(\w*)\W*(\w*)\W*\)\W*\}\}").unwrap();
        let caps = re.captures_iter(&self.query);
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

    pub fn get_rendered_query(&self) -> String {
        let mut env = Environment::new();
        env.add_function("ref", ref_q);
        env.add_template("sql", &self.query).unwrap();
        let tmpl = env.get_template("sql").unwrap();
        tmpl.render(context!()).unwrap()
    }
}

/// Get dependencies from SQL written ref macro with Jinja.
/// # Example
/// let d = get_dependencies("sqls") // You chose directory name
pub fn get_dependencies(root_dir: &str) -> Vec<Table> {
    let mut v: Vec<Table> = Vec::new();
    for entry in WalkDir::new(root_dir) {
        let entry = entry.unwrap();
        let re = Regex::new(r"(\w*)\.sql").unwrap();

        match entry.path().file_name() {
            Some(path) => {
                if let Some(p) = path.to_str() {
                    if re.is_match(p) {
                        let sql = SQL::new(format!("{}/{}", root_dir, p).to_string());
                        let depends_on = sql.get_ref_tables();
                        v.push(Table::new(p.to_string(), sql, depends_on))
                    }
                }
            }
            None => {
                println!("Skip")
            }
        }
    }
    v
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
        let s = SQL::new("src/sample_sqls/sample.sql".to_string());
        assert!(s.path == "src/sample_sqls/sample.sql".to_string());
        println!("{:?}", s.query);
    }

    #[test]
    fn test_get_ref() {
        let s = SQL::new("src/sample_sqls/sample.sql".to_string());
        let tables = s.get_ref_tables();
        assert!(tables[0] == "db.users".to_string());
        assert!(tables[1] == "role".to_string());
    }

    #[test]
    fn test_get_rendered_query() {
        let s = SQL::new("src/sample_sqls/sample.sql".to_string());
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
