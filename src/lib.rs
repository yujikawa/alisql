mod sql_analyzer;
use sql_analyzer::analyzer::{RegexSQLAnalyser, Analyzer, Table};
use walkdir::WalkDir;


/// Get dependencies from SQL written ref macro with Jinja.
/// # Example
/// let d: Vec<Table> = get_dependencies("sqls") // You chose directory name
pub fn get_dependencies(root_dir: &str) -> Vec<Table> {
    let mut v: Vec<Table> = Vec::new();
    for entry in WalkDir::new(root_dir) {
        let entry = entry.unwrap();
        match entry.path().file_name() {
            Some(path) => {
                if let Some(p) = path.to_str() {
                    if RegexSQLAnalyser::is_sql_file(p) {
                        let analyzer = RegexSQLAnalyser::new(format!("{}/{}", root_dir, p).to_string());
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


