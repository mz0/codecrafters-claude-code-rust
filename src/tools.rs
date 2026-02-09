use serde_json::{json, Value};
use std::fs;
use std::process::Command;
use strum::{Display, EnumString, IntoEnumIterator, EnumIter};


#[derive(Debug, Clone, Copy, EnumString, Display, EnumIter)]
pub enum ToolName {
    Bash,
    ReadFile,
    WriteFile,
}

pub struct Tool {
    pub definition: Value,
}

impl Tool {
    pub fn run_bash() -> Self {
        Self {
            definition: json!({"type": "function", "function": {
                    "name": ToolName::Bash.to_string(),
                    "description": "Execute a shell command",
                    "parameters": {
                        "type": "object",
                        "required": ["command"],
                        "properties": {
                            "command": {
                                "type": "string",
                                "description": "The command to execute"
                            }
                        }
                    }
                }}),
        }
    }

    pub fn call_run_bash(args: Value) -> Result<String, String> {
        let command = args["command"].as_str()
            .ok_or("Missing command argument")?;

        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(stdout.to_string())
        } else {
            Err(format!("Command failed: {}", stderr))
        }
    }

    /// Creates the 'Read' tool definition
    pub fn read_file() -> Self {
        Self {
            definition: json!({"type": "function", "function": {
                    "name": ToolName::ReadFile.to_string(),
                    "description": "Read and return the contents of a file",
                    "parameters": {
                        "type": "object",
                        "required": ["file_path"],
                        "properties": {
                            "file_path": {
                                "type": "string",
                                "description": "The path to the file to read"
                            }
                        }
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

    pub fn write_file() -> Self {
        Self {
            definition: json!({"type": "function", "function": {
                    "name": ToolName::WriteFile.to_string(),
                    "description": "Write content to a file",
                    "parameters": {
                        "type": "object",
                        "required": ["file_path", "content"],
                        "properties": {
                            "file_path": {
                                "type": "string",
                                "description": "The path to the file to read"
                            },
                            "content": {
                                "type": "string",
                                "description": "The content to write to the file"
                            }
                        }
                    }
                }}),
        }
    }

    pub fn call_write_file(args: Value) -> Result<String, String> {
        let path_str = args["file_path"].as_str()
            .ok_or("Missing file_path argument")?;
        let content = args["content"].as_str()
            .ok_or("Missing content argument")?;

        let path = std::path::Path::new(path_str);

        if let Some(file_name) = path.file_name() {
            if file_name == ".env" {
                return Err("Access to .env file is forbidden".to_string());
            }
        }

        fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(format!("Wrote OK {}", path_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_write_readme_with_code_block() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("README.md");
        let text = r#"# Test
Write file `app/main.py` with the following content:
```python
print("Hello world")
```
"#;
        let args = json!({
            "file_path": file_path.to_str().unwrap(),
            "content": text
        });

        let result = Tool::call_write_file(args);
        assert!(result.is_ok());

        let written = fs::read_to_string(&file_path).unwrap();
        assert_eq!(written, text);
    }

    #[test]
    fn test_write_python_file() {
        let dir = tempdir().unwrap();
        let app_dir = dir.path().join("app");
        fs::create_dir(&app_dir).unwrap();
        let file_path = app_dir.join("main.py");
        let main_py_text = "print(\"Hello world\")";

        let args = json!({
            "file_path": file_path.to_str().unwrap(),
            "content": main_py_text
        });

        let result = Tool::call_write_file(args);
        assert!(result.is_ok());

        let written = fs::read_to_string(&file_path).unwrap();
        assert_eq!(written, main_py_text);
    }

    #[test]
    fn test_bash_rm_file() {
        let dir = tempdir().unwrap();
        let readme_path = dir.path().join("README.md");
        let junk_path = dir.path().join("Junk.txt");

        fs::write(&readme_path, "# README").unwrap();
        fs::write(&junk_path, "junk").unwrap();

        let args = json!({
            "command": format!("rm {}", junk_path.to_str().unwrap())
        });

        let result = Tool::call_run_bash(args);
        assert!(result.is_ok());

        assert!(readme_path.exists(), "README.md should still exist");
        assert!(!junk_path.exists(), "Junk.txt should be deleted");
    }
}
