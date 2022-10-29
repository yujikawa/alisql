use super::analyzer::{Table};

pub struct Mermaid {
    tables: Vec<Table>,
}

impl Mermaid {
    fn new(tables: Vec<Table>) -> Self {
        Mermaid { tables: tables}
    }

    fn get_graph(&self) -> String {
        let mut temp: Vec<String>= Vec::new();
        temp.push("graph TD".to_string());
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
    use crate::sql_analyzer::analyzer::Analyzer;

    use super::super::analyzer::RegexSQLAnalyser;
    use super::*;

    #[test]
    fn test_get_graph() {
        let s = RegexSQLAnalyser::new("src/sample_sqls/sample.sql".to_string());
        let tables = s.get_analized_table();
        let m = Mermaid::new(vec![tables]);
        let mermaid = m.get_graph();
        assert!(mermaid == "graph TD;\ndb.users --> sample;\nrole --> sample;\n".to_string());
    }
   
}
