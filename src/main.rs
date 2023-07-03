#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]

use std::io::Read;

use clap::Parser;

/// CLI tool to format your SQL code.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The SQL code to format. Could be passed as an argument or piped through stdin.
    sql: Option<String>,

    /// Whether to use uppercase keywords.
    #[clap(short, long)]
    uppercase: bool,

    /// Whether to use tabs instead of spaces.
    #[clap(short, long)]
    tabs: bool,

    /// The number of spaces to use for indentation.
    #[clap(short, long, default_value = "2")]
    indent: u8,

    /// The number of lines between queries.
    #[clap(short, long, default_value = "1")]
    lines_between_queries: u8,
}

impl From<&Args> for sqlformat::FormatOptions {
    fn from(args: &Args) -> Self {
        Self {
            indent: if args.tabs {
                sqlformat::Indent::Tabs
            } else {
                sqlformat::Indent::Spaces(args.indent)
            },
            uppercase: args.uppercase,
            lines_between_queries: args.lines_between_queries,
        }
    }
}

impl Args {
    /// Format the SQL code.
    fn format(&self) -> String {
        let sql = self.get_sql();
        let indentation = " ".repeat(sql.len() - sql.trim_start().len());
        let formatted = sqlformat::format(&sql, &sqlformat::QueryParams::None, self.into());
        formatted
            .lines()
            .map(|line| format!("{indentation}{line}",))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get the SQL code from stdin or the command line.
    fn get_sql(&self) -> String {
        if let Some(sql) = &self.sql {
            sql.clone()
        } else {
            let mut sql = String::new();
            std::io::stdin()
                .read_to_string(&mut sql)
                .expect("Failed to read from stdin");
            sql
        }
    }
}

fn main() {
    let args = Args::parse();

    println!("{}", args.format());
}
