mod cli;
mod template;

use clap::Parser;
use cli::Cli;
use template::TemplateManager;

fn main() {
    let mut template_manager = TemplateManager::default();

    // Parse CLI arguments
    let cli = Cli::parse();

    match &cli.command {
        cli::Commands::Register { name, file } => {
            if let Err(e) = template_manager.register_template(name.as_str(), &file) {
                eprintln!("Error registering template: {}", e);
            }
        }
        cli::Commands::Create { name, target } => {
            if let Err(e) = template_manager.create_file_from_template(name.as_str(), &target) {
                eprintln!("Error creating file: {}", e);
            }
        }
        cli::Commands::List => {
            template_manager.list_templates();
        }
    }
}