mod sql_analyzer;
use sql_analyzer::{
    analyzer::{Analyzer, RegexSQLAnalyser, Table},
    graph::Mermaid,
};
use walkdir::WalkDir;

/// Get dependencies from SQL written ref macro with Jinja.
/// # Example
/// let d: Vec<Table> = get_dependencies("sqls");
pub fn get_dependencies(root_dir: &str) -> Vec<Table> {
    let mut v: Vec<Table> = Vec::new();
    for entry in WalkDir::new(root_dir) {
        let entry = entry.unwrap();
        match entry.path().file_name() {
            Some(path) => {
                if let Some(p) = path.to_str() {
                    if RegexSQLAnalyser::is_sql_file(p) {
                        let analyzer =
                            RegexSQLAnalyser::new(format!("{}/{}", root_dir, p).to_string());
                        let table = analyzer.get_analized_table();
                        v.push(table)
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

/// Get dependencies graph from SQL with Mermaid.
/// # Example
/// let mermaid = get_mermaid("sqls");
pub fn get_mermaid(root_dir: &str) -> String {
    let tables = get_dependencies(root_dir);
    let m = Mermaid::new(tables);
    m.get_graph()
}
