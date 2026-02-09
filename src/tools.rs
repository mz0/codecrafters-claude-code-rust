use serde_json::{json, Value};
use std::fs;

#[derive(Debug, Clone, Copy)]
pub enum ToolName {
    Read,
}

impl ToolName {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToolName::Read => "Read",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Read" => Some(ToolName::Read),
            _ => None,
        }
    }
}

pub struct Tool {
    pub definition: Value,
}

impl Tool {
    /// Creates the 'Read' tool definition
    pub fn read_file() -> Self {
        Self {
            definition: json!({"type": "function", "function": {
                    "name": ToolName::Read.as_str(),
                    "description": "Read and return the contents of a file",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "file_path": {
                                "type": "string",
                                "description": "The path to the file to read"
                            }
                        },
                        "required": ["file_path"]
                    }
                }}),
        }
    }

    pub fn call_read_file(args: Value) -> Result<String, String> {
        let path_str = args["file_path"].as_str()
            .ok_or("Missing file_path argument")?;
        let path = std::path::Path::new(path_str);
        // 1. Check if it exists and is a file
        if !path.exists() {
            return Err(format!("File not found: {}", path.display()));
        }
        if !path.is_file() {
            return Err(format!("Path is not a file: {}", path.display()));
        }

        // 2. Check if it is .env
        if let Some(file_name) = path.file_name() {
            if file_name == ".env" {
                return Err("Access to .env file is forbidden".to_string());
            }
        }
        fs::read_to_string(path).map_err(|e| e.to_string())
    }
}
