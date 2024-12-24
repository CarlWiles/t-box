use clap::{Parser, Subcommand};

/// Define CLI parameters and commands
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Register a template
    Register {
        /// Template name
        name: String,
        /// Template file path
        file: String,
    },
    /// Create a file using a template
    Create {
        /// Template name
        name: String,
        /// Target file path
        target: String,
    },
    /// List all templates
    List,
    /// Delete a templates
    Delete {
        /// Delete template name
        name: String,
    },
}
