use dirs::home_dir;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Structure to manage templates
#[derive(Debug, Default)]
pub struct TemplateManager {
    templates: HashMap<String, String>, // Template name -> Template content
}

impl TemplateManager {
    /// Get the path to the template storage directory
    fn get_template_dir() -> PathBuf {
        let mut path = home_dir().expect("Failed to find home directory");
        path.push(".t-box-template");
        path
    }

    /// Ensure the template directory exists
    fn ensure_template_dir() -> io::Result<()> {
        let dir = Self::get_template_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(())
    }

    /// Save a single template to the template directory
    fn save_template(name: &str, content: &str) -> io::Result<()> {
        let mut path = Self::get_template_dir();
        path.push(name);
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Load all templates from the template directory
    pub fn load_templates(&mut self) -> io::Result<()> {
        let dir = Self::get_template_dir();
        if dir.exists() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        self.templates.insert(name.to_string(), content);
                    }
                }
            }
        }
        Ok(())
    }

    /// Register a new template
    pub fn register_template(
        &mut self,
        template_name: &str,
        template_file: &str,
    ) -> io::Result<()> {
        let content = fs::read_to_string(template_file)?;
        Self::ensure_template_dir()?;
        Self::save_template(template_name, &content)?;
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

    pub fn delete_template(&mut self, template_name: &str) -> io::Result<()> {
        let mut path = Self::get_template_dir();
        path.push(template_name);

        if path.exists() {
            fs::remove_file(&path)?;
            self.templates.remove(template_name); // Remove from in-memory storage
            println!("Template '{}' deleted successfully.", template_name);
            Ok(())
        } else {
            println!("Template '{}' not found.", template_name);
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Template does not exist",
            ))
        }
    }
}
