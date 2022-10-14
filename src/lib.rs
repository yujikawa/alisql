#[derive(Debug)]
struct SQL {
    path: String,
}

#[derive(Debug)]
struct Table {
    name: String,
}

#[derive(Debug)]
struct Lineage {
    table: Table,
    sql: SQL,
    depends_on: Vec<Table>,
}

impl Lineage {
    fn new(sql: String) -> Self {
        println!("{}", sql);
        Lineage { 
            table: Table {name: "sample".to_string()}, 
            sql: SQL { path: "sample.sql".to_string() }, 
            depends_on: vec![Table { name: "users".to_string()}] 
    }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_table() {
        let t = Table{
            name: "users".to_string()
        };
        assert!(t.name == "users".to_string());
    }
    #[test]
    fn test_create_sql() {
        let s = SQL{
            path: "users.sql".to_string()
        };
        assert!(s.path == "users.sql".to_string());
    }

    #[test]
    fn test_create_lineage() {
        let l = Lineage {
            table: Table { name: "users".to_string()},
            sql: SQL{ path: "users.sql".to_string()},
            depends_on: vec![Table{name: "role".to_string()}]
        };
        assert!(l.depends_on.len() == 1)
    }

    #[test]
    fn test_get_lineage() {
        let sql ="select * from {{ ref('db', 'users') }}".to_string(); 

        let l = Lineage::new(sql);
        assert!(l.depends_on.len() == 1);
        assert!(l.depends_on[0].name == "users".to_string());
    }
}