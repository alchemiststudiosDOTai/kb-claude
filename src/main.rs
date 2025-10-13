use anyhow::Result;
use clap::{Parser, Subcommand};

mod agent;
mod commands;
mod io;
mod manifest;
mod models;
mod schema;

#[derive(Parser)]
#[command(name = "claude-kb")]
#[command(about = "Structured, typed management of .claude knowledge base", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create new typed KB entries")]
    Add {
        #[arg(value_name = "TYPE")]
        entry_type: String,
        #[arg(long)]
        component: String,
        #[arg(long)]
        summary: Option<String>,
        #[arg(long)]
        error: Option<String>,
        #[arg(long)]
        solution: Option<String>,
        #[arg(long)]
        question: Option<String>,
        #[arg(long)]
        answer: Option<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Modify existing entries by ID or file")]
    Update {
        #[arg(value_name = "TYPE")]
        entry_type: String,
        #[arg(long)]
        file: Option<String>,
        #[arg(long)]
        component: Option<String>,
        #[arg(long)]
        error: Option<String>,
        #[arg(long)]
        solution: Option<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Delete entries by component or file")]
    Delete {
        #[arg(value_name = "TYPE")]
        entry_type: String,
        #[arg(long)]
        component: Option<String>,
        #[arg(long)]
        file: Option<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Validate schema integrity")]
    Validate {
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Synchronize manifest & detect drift")]
    Sync {
        #[arg(long)]
        verbose: bool,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Enumerate entries by type or component")]
    List {
        #[arg(long)]
        r#type: Option<String>,
        #[arg(long)]
        component: Option<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Compare manifests between commits")]
    Diff {
        #[arg(long)]
        since: Option<String>,
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            entry_type,
            component,
            summary,
            error,
            solution,
            question,
            answer,
            json,
        } => commands::add::handle(
            entry_type, component, summary, error, solution, question, answer, json,
        ),
        Commands::Update {
            entry_type,
            file,
            component,
            error,
            solution,
            json,
        } => commands::update::handle(entry_type, file, component, error, solution, json),
        Commands::Delete {
            entry_type,
            component,
            file,
            json,
        } => commands::delete::handle(entry_type, component, file, json),
        Commands::Validate { path, json } => commands::validate::handle(path, json),
        Commands::Sync { verbose, json } => commands::sync::handle(verbose, json),
        Commands::List {
            r#type,
            component,
            json,
        } => commands::list::handle(r#type, component, json),
        Commands::Diff { since, json } => commands::diff::handle(since, json),
    }
}
