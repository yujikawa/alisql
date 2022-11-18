use super::analyzer::Table;

pub struct Mermaid {
    tables: Vec<Table>,
}

impl Mermaid {
    pub fn new(tables: Vec<Table>) -> Self {
        Mermaid { tables: tables }
    }

    pub fn get_graph(&self, orientation: &str) -> String {
        let mut temp: Vec<String> = Vec::new();
        temp.push(format!("graph {}", orientation).to_string());
        for table in &self.tables {
            for depends_table in &table.depends_on {
                temp.push(format!("{} --> {}", depends_table, table.table));
            }
        }
        temp.join(";\n") + ";\n"
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::str::FromStr;

    use crate::sql_analyzer::analyzer::Analyzer;

    use super::super::analyzer::RegexSQLAnalyser;
    use super::*;

    #[test]
    fn test_get_graph() {
        let path = OsString::from_str("src/sample_sqls/level1/sample.sql").unwrap();
        let s = RegexSQLAnalyser::new(path);
        let tables = s.get_analized_table();
        let m = Mermaid::new(vec![tables]);
        let mermaid = m.get_graph("TD");
        assert!(mermaid == "graph TD;\ndb.users --> sample;\nrole --> sample;\n".to_string());
    }
}
