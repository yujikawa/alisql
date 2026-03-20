use alisql::sql_analyzer::analyzer::Table;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "alisql", about = "Analyze SQL files with Jinja2 ref() macros")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show table dependencies extracted from SQL files
    Deps {
        /// Root directory containing SQL files
        dir: String,

        /// Maximum directory depth to search
        #[arg(short, long, default_value_t = 5)]
        max_depth: usize,

        /// Output format
        #[arg(short, long, default_value = "text")]
        format: Format,
    },
    /// Generate a Mermaid dependency graph
    Graph {
        /// Root directory containing SQL files
        dir: String,

        /// Maximum directory depth to search
        #[arg(short, long, default_value_t = 5)]
        max_depth: usize,

        /// Graph orientation
        #[arg(short, long, default_value = "td")]
        orientation: Orientation,
    },
}

#[derive(ValueEnum, Clone)]
enum Format {
    Text,
    Json,
}

#[derive(ValueEnum, Clone)]
enum Orientation {
    Tb,
    Td,
    Bt,
    Rl,
    Lr,
}

impl Orientation {
    fn as_str(&self) -> &'static str {
        match self {
            Orientation::Tb => "TB",
            Orientation::Td => "TD",
            Orientation::Bt => "BT",
            Orientation::Rl => "RL",
            Orientation::Lr => "LR",
        }
    }
}

fn format_deps_text(tables: &[Table]) -> String {
    let mut lines = Vec::new();
    for table in tables {
        lines.push(format!("[{}]", table.table));
        if table.depends_on.is_empty() {
            lines.push("  (no dependencies)".to_string());
        } else {
            for dep in &table.depends_on {
                lines.push(format!("  <- {}", dep));
            }
        }
    }
    lines.join("\n")
}

fn format_deps_json(tables: &[Table]) -> String {
    serde_json::to_string_pretty(tables).unwrap()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Deps {
            dir,
            max_depth,
            format,
        } => {
            let tables = alisql::get_dependencies(&dir, max_depth);
            let output = match format {
                Format::Json => format_deps_json(&tables),
                Format::Text => format_deps_text(&tables),
            };
            println!("{}", output);
        }
        Commands::Graph {
            dir,
            max_depth,
            orientation,
        } => {
            let graph = alisql::get_mermaid(&dir, orientation.as_str(), max_depth);
            println!("{}", graph);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alisql::sql_analyzer::analyzer::{Analyzer, RegexSQLAnalyser};
    use std::ffi::OsString;
    use std::str::FromStr;

    fn sample_table() -> Table {
        let path = OsString::from_str("src/sample_sqls/level1/sample.sql").unwrap();
        RegexSQLAnalyser::new(path).get_analized_table()
    }

    #[test]
    fn test_format_deps_text_with_dependencies() {
        let table = sample_table();
        let output = format_deps_text(&[table]);
        assert!(output.contains("[sample]"));
        assert!(output.contains("  <- db.users"));
        assert!(output.contains("  <- role"));
    }

    #[test]
    fn test_format_deps_text_no_dependencies() {
        use alisql::sql_analyzer::analyzer::SQL;
        let sql = SQL::new(
            OsString::from("dummy.sql"),
            "select 1".to_string(),
            "select 1".to_string(),
        );
        let table = Table::new("dummy".to_string(), sql, vec![]);
        let output = format_deps_text(&[table]);
        assert!(output.contains("[dummy]"));
        assert!(output.contains("(no dependencies)"));
    }

    #[test]
    fn test_format_deps_json() {
        let table = sample_table();
        let output = format_deps_json(&[table]);
        let parsed: serde_json::Value = serde_json::from_str(&output).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed[0]["table"], "sample");
        assert_eq!(parsed[0]["depends_on"][0], "db.users");
        assert_eq!(parsed[0]["depends_on"][1], "role");
    }

    #[test]
    fn test_orientation_as_str() {
        assert_eq!(Orientation::Td.as_str(), "TD");
        assert_eq!(Orientation::Lr.as_str(), "LR");
        assert_eq!(Orientation::Bt.as_str(), "BT");
        assert_eq!(Orientation::Rl.as_str(), "RL");
        assert_eq!(Orientation::Tb.as_str(), "TB");
    }
}
