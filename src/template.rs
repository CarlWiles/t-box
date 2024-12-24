use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Structure to manage templates
#[derive(Debug, Default)]
pub struct TemplateManager {
    templates: HashMap<String, String>, // Template name -> Template content
}

impl TemplateManager {
    /// Register a template
    pub fn register_template(
        &mut self,
        template_name: &str,
        template_file: &str,
    ) -> io::Result<()> {
        let content = fs::read_to_string(template_file)?;
        self.templates.insert(template_name.to_string(), content);
        println!("Template registered: {}", template_name);
        Ok(())
    }

    /// Create a file using a template
    pub fn create_file_from_template(
        &self,
        template_name: &str,
        target_file: &str,
    ) -> io::Result<()> {
        if let Some(content) = self.templates.get(template_name) {
            // Check if the target file already exists
            if Path::new(target_file).exists() {
                println!("File already exists: {}. Skipping creation.", target_file);
                return Ok(());
            }

            // Create the target file and write the template content
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(target_file)?;
            writeln!(file, "{}", content)?;

            println!(
                "File created: {} using template: {}",
                target_file, template_name
            );
            Ok(())
        } else {
            println!("Template not found: {}", template_name);
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Template does not exist",
            ))
        }
    }

    /// List all registered templates
    pub fn list_templates(&self) {
        if self.templates.is_empty() {
            println!("No templates registered.");
        } else {
            println!("Registered templates:");
            for name in self.templates.keys() {
                println!("- {}", name);
            }
        }
    }
}
