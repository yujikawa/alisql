use std::fmt::format;
use std::io::Read;
use std::vec;
use std::fs::File;
use regex::Regex;
use std::fs;

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
        let s = SQL::new("src/sample.sql".to_string());
        assert!(s.path == "src/sample.sql".to_string());
        println!("{:?}", s.query);
    }

    #[test]
    fn test_get_ref() {
        let s = SQL::new("src/sample.sql".to_string());
        let tables = s.get_ref_tables();
        assert!(tables[0].name == "db.users".to_string());
        assert!(tables[1].name == "db.role".to_string());
    }
}
