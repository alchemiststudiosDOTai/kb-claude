use anyhow::Result;
use clap::{Parser, Subcommand};

mod agent;
mod commands;
mod io;
mod manifest;
mod models;
mod schema;

use models::EntryType;

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
        entry_type: EntryType,
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
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        heading: Option<String>,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        file_path: Option<String>,
        #[arg(long)]
        note: Option<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Modify existing entries by ID or file")]
    Update {
        #[arg(value_name = "TYPE")]
        entry_type: EntryType,
        #[arg(long)]
        file: Option<String>,
        #[arg(long)]
        component: Option<String>,
        #[arg(long)]
        error: Option<String>,
        #[arg(long)]
        solution: Option<String>,
        #[arg(long)]
        question: Option<String>,
        #[arg(long)]
        answer: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        heading: Option<String>,
        #[arg(long)]
        content: Option<String>,
        #[arg(long)]
        file_path: Option<String>,
        #[arg(long)]
        note: Option<String>,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Delete entries by component or file")]
    Delete {
        #[arg(value_name = "TYPE")]
        entry_type: EntryType,
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
        r#type: Option<EntryType>,
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
            name,
            description,
            heading,
            content,
            file_path,
            note,
            json,
        } => commands::add::handle(
            entry_type,
            component,
            summary,
            error,
            solution,
            question,
            answer,
            name,
            description,
            heading,
            content,
            file_path,
            note,
            json,
        ),
        Commands::Update {
            entry_type,
            file,
            component,
            error,
            solution,
            question,
            answer,
            name,
            description,
            heading,
            content,
            file_path,
            note,
            json,
        } => commands::update::handle(
            entry_type,
            file,
            component,
            error,
            solution,
            question,
            answer,
            name,
            description,
            heading,
            content,
            file_path,
            note,
            json,
        ),
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
