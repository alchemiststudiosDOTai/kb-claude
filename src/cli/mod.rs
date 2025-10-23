use std::path::PathBuf;

use anyhow::Result;
use clap::{ArgAction, Args, Parser, Subcommand};

mod init;
mod link;
mod manifest;
mod new;
mod search;
mod validate;

#[derive(Parser, Debug)]
#[command(
    name = "kb-claude",
    version,
    about = "CLI utilities for managing kb-claude knowledge bases",
    arg_required_else_help = true,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Init(InitArgs),
    New(NewArgs),
    Search(SearchArgs),
    Link(LinkArgs),
    Validate(ValidateArgs),
    Manifest(ManifestArgs),
}

#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    #[arg(
        short,
        long,
        value_name = "PATH",
        default_value = ".",
        help = "Directory to initialize with a .claude structure"
    )]
    pub directory: PathBuf,
    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Print the layout actions without writing to disk"
    )]
    pub dry_run: bool,
}

#[derive(Args, Debug, Clone)]
pub struct NewArgs {
    #[arg(value_name = "TITLE", help = "Title for the new knowledge entry")]
    pub title: String,
    #[arg(
        short = 't',
        long = "type",
        value_name = "TYPE",
        help = "Explicit document type; defaults to interactive prompt"
    )]
    pub doc_type: Option<String>,
    #[arg(
        short = 'g',
        long = "tag",
        value_name = "TAG",
        action = ArgAction::Append,
        help = "Tag to attach; use multiple times for more than one tag"
    )]
    pub tags: Vec<String>,
    #[arg(
        long = "relates-to",
        value_name = "LINK",
        action = ArgAction::Append,
        help = "Link identifiers to relate to; repeat for multiples"
    )]
    pub relates_to: Vec<String>,
    #[arg(
        short = 'f',
        long = "file",
        value_name = "PATH",
        help = "Optional path override for the output markdown file"
    )]
    pub file_override: Option<PathBuf>,
}

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    #[arg(
        value_name = "TERM",
        required = true,
        help = "Keyword terms to search; provide one or more"
    )]
    pub terms: Vec<String>,
    #[arg(
        short = 't',
        long = "tag",
        action = ArgAction::Append,
        value_name = "TAG",
        help = "Filter results by tag; repeat for multiple tags"
    )]
    pub tags: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct LinkArgs {
    #[arg(value_name = "SOURCE", help = "Link slug for the source document")]
    pub source: String,
    #[arg(value_name = "TARGET", help = "Link slug for the target document")]
    pub target: String,
    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Skip duplicate checking when inserting relations"
    )]
    pub force: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ValidateArgs {
    #[arg(
        short,
        long,
        value_name = "PATH",
        help = "Workspace directory to validate; defaults to current"
    )]
    pub directory: Option<PathBuf>,
    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Treat warnings as errors when reporting validation findings"
    )]
    pub strict: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ManifestArgs {
    #[arg(
        short,
        long,
        value_name = "PATH",
        help = "Write manifest to a custom location instead of default"
    )]
    pub output: Option<PathBuf>,
    #[arg(
        short = 'd',
        long,
        value_name = "PATH",
        help = "Workspace directory containing the .claude hierarchy"
    )]
    pub directory: Option<PathBuf>,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    execute(cli)
}

pub fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Init(args) => init::run(args),
        Command::New(args) => new::run(args),
        Command::Search(args) => search::run(args),
        Command::Link(args) => link::run(args),
        Command::Validate(args) => validate::run(args),
        Command::Manifest(args) => manifest::run(args),
    }
}
