use serde_json::{json, Value};
use crate::tools::{Tool, ToolName};

pub struct Manager {
    pub messages: Vec<Value>,
    pub tools: Vec<Value>,
}

impl Manager {
    pub fn new(prompt: &str) -> Self {
        Self {
            messages: vec![json!({ "role": "user", "content": prompt })],
            tools: vec![
                Tool::run_bash().definition,
                Tool::read_file().definition,
                Tool::write_file().definition,
            ]
        }
    }

    /// Appends a message to the dialog history
    pub fn add_message(&mut self, message: Value) {
        self.messages.push(message);
    }

    /// Handles the logic of executing the tool and formatting the response
    pub fn handle_tool_call(&mut self, tool_call: &Value) -> Result<(), String> {
        let name = tool_call["function"]["name"].as_str().unwrap_or("");
        let args_str = tool_call["function"]["arguments"].as_str().unwrap_or("{}");
        let args: Value = serde_json::from_str(args_str).map_err(|e| e.to_string())?;
        let call_id = tool_call["id"].as_str().unwrap_or("");

        let result = match name.parse::<ToolName>() {
            Ok(ToolName::Bash) => Tool::call_run_bash(args),
            Ok(ToolName::ReadFile) => Tool::call_read_file(args),
            Ok(ToolName::WriteFile) => Tool::call_write_file(args),
            Err(_) => Err(format!("Tool {} not found", name)),
        };

        // Format the result for the AI
        self.add_message(json!({
            "role": "tool",
            "tool_call_id": call_id,
            "content": result.unwrap_or_else(|e| e)
        }));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_all_tools_offered() {
        let manager = Manager::new("test");

        // Collect all tool names offered to the model
        let offered_names: Vec<String> = manager.tools
            .iter()
            .filter_map(|t| t["function"]["name"].as_str())
            .map(|s| s.to_string())
            .collect();

        // Check that every ToolName variant is offered
        for tool_name in ToolName::iter() {
            assert!(
                offered_names.contains(&tool_name.to_string()),
                "Tool '{}' is not offered to the model",
                tool_name
            );
        }
    }
}
