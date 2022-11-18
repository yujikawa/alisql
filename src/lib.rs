//!# QuickStart
//!## Get dependencies from SQL written ref macro with Jinja.
//!First, create sql like Jinja.
//!```sql
//!-- src/sample_sqls/level1/sample.sql
//!select
//!u.*
//!, r.*
//!from {{ ref("db", "users") }} as u
//!left join {{ ref("role") }} as r on
//!u.id = r.user_id
//!```
//!```sql
//!-- src/sample_sqls/sample2.sql
//!select
//!u.*
//!, r.*
//!from {{ ref("db", "sales") }} as u
//!left join {{ ref("db", "sale_detail") }} as r on
//!u.id = r.sale_id
//!```
//!Use alias lib from main function.
//!```rust no_run
//!use alisql;
//!
//!let d = alisql::get_dependencies("src/sample_sqls", 5);
//! dbg!(d);
//!```
//!### Result
//!```text
//! d = [
//!    Table {
//!        table: "sample",
//!        sql: SQL {
//!            path: "src/sample_sqls/level1/sample.sql",
//!            query: "select \nu.*\n, r.* \nfrom {{ ref(\"db\", \"users\") }} as u\nleft join {{ ref(\"role\") }} as r on\nu.id = r.user_id",
//!        },
//!        depends_on: [
//!            "db.users",
//!            "role",
//!        ],
//!    },
//!    Table {
//!        table: "sample2",
//!        sql: SQL {
//!            path: "src/sample_sqls/sample2.sql",
//!            query: "select \nu.*\n, r.* \nfrom {{ ref(\"db\", \"sales\") }} as u\nleft join {{ ref(\"db\", \"sale_detail\") }} as r on\nu.id = r.sale_id",
//!        },
//!        depends_on: [
//!            "db.sales",
//!            "db.sale_detail",
//!        ],
//!    },
//!]
//!```
//! ## Get dependencies graph from SQL with Mermaid.
//!
//! orientation are
//! - TB : top to bottom
//! - TD : top-down/ same as top to bottom
//! - BT : bottom to top
//! - RL : right to left
//! - LR : left to right
//!
//! ```rust no_run
//! use alisql;
//!
//! let m = alisql::get_mermaid("src/sample_sqls", "TD", 5);
//! println!("{}", m);
//!```
//!
//! ```text
//!graph TD;
//!db.users --> sample;
//!role --> sample;
//!db.sales --> sample2;
//!db.sale_detail --> sample2;
//!```
//!
//! ### Reference
//! - [Mermaid](https://mermaid-js.github.io/mermaid/#/./flowchart?id=flowchart-orientation)
//!
pub mod sql_analyzer;
use sql_analyzer::{
    analyzer::{Analyzer, RegexSQLAnalyser, Table},
    graph::Mermaid,
};
use walkdir::WalkDir;

pub fn get_dependencies(root_dir: &str, max_depth: usize) -> Vec<Table> {
    let mut v: Vec<Table> = Vec::new();
    for entry in WalkDir::new(root_dir).max_depth(max_depth) {
        let entry = entry.unwrap();
        let file_path = entry.path();

        match file_path.file_name() {
            Some(path) => {
                if let Some(p) = path.to_str() {
                    if RegexSQLAnalyser::is_sql_file(p) {
                        let analyzer = RegexSQLAnalyser::new(file_path.as_os_str().to_os_string());
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

pub fn get_mermaid(root_dir: &str, orientation: &str, max_depth: usize) -> String {
    let tables = get_dependencies(root_dir, max_depth);
    let m = Mermaid::new(tables);
    m.get_graph(orientation)
}
