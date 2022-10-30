pub mod sql_analyzer;
use sql_analyzer::{
    analyzer::{Analyzer, RegexSQLAnalyser, Table},
    graph::Mermaid,
};
use walkdir::WalkDir;

/// Get dependencies from SQL written ref macro with Jinja.
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
///
/// orientation are
/// - TB : top to bottom
/// - TD : top-down/ same as top to bottom
/// - BT : bottom to top
/// - RL : right to left
/// - LR : left to right
///
/// ## Reference
/// - [Mermaid](https://mermaid-js.github.io/mermaid/#/./flowchart?id=flowchart-orientation)

pub fn get_mermaid(root_dir: &str, orientation: &str) -> String {
    let tables = get_dependencies(root_dir);
    let m = Mermaid::new(tables);
    m.get_graph(orientation)
}
