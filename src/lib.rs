mod sql_analyzer;
use sql_analyzer::{
    analyzer::{Analyzer, RegexSQLAnalyser, Table},
    graph::Mermaid,
};
use walkdir::WalkDir;

/// Get dependencies from SQL written ref macro with Jinja.
/// # Example
/// ```rust
/// let d: Vec<Table> = get_dependencies("sqls");
/// ```
/// First, create sql like Jinja.
/// ```sql
/// -- src/sample_sqls/sample.sql
/// select
/// u.*
/// , r.*
/// from {{ ref("db", "users") }} as u
/// left join {{ ref("role") }} as r on
/// u.id = r.user_id
/// ```
///
/// ```sql
/// -- src/sample_sqls/sample2.sql
/// select
/// u.*
/// , r.*
/// from {{ ref("db", "sales") }} as u
/// left join {{ ref("db", "sale_detail") }} as r on
/// u.id = r.sale_id
/// ```
///
/// Use alias lib from main function.
/// ```rust
/// use alias;
/// fn main() {
///     let d = alias::get_dependencies("src/sample_sqls")
/// }
/// ```
/// ## Result
///
/// ```rust
/// d = [
///     Table {
///         table: "sample",
///         sql: SQL {
///             path: "src/sample_sqls/sample.sql",
///             query: "select \nu.*\n, r.* \nfrom {{ ref(\"db\", \"users\") }} as u\nleft join {{ ref(\"role\") }} as r on\nu.id = r.user_id",
///         },
///         depends_on: [
///             "db.users",
///             "role",
///         ],
///     },
///     Table {
///         table: "sample2",
///         sql: SQL {
///             path: "src/sample_sqls/sample2.sql",
///             query: "select \nu.*\n, r.* \nfrom {{ ref(\"db\", \"sales\") }} as u\nleft join {{ ref(\"db\", \"sale_detail\") }} as r on\nu.id = r.sale_id",
///         },
///         depends_on: [
///             "db.sales",
///             "db.sale_detail",
///         ],
///     },
/// ]
/// ```
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
/// ```rust
/// let mermaid = get_mermaid("sqls", "TD");
/// ```
/// orientation are
/// - TB : top to bottom
/// - TD : top-down/ same as top to bottom
/// - BT : bottom to top
/// - RL : right to left
/// - LR : left to right
///
/// Reference: [Mermaid](https://mermaid-js.github.io/mermaid/#/./flowchart?id=flowchart-orientation)
///
/// ## Result
/// ```mermaid
///graph TD;
///db.users --> sample;
///role --> sample;
///db.sales --> sample2;
///db.sale_detail --> sample2;
///```
pub fn get_mermaid(root_dir: &str, orientation: &str) -> String {
    let tables = get_dependencies(root_dir);
    let m = Mermaid::new(tables);
    m.get_graph(orientation)
}
