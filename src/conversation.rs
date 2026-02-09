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
            tools: vec![Tool::read_file().definition],
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

        let result = match ToolName::from_str(name) {
            Some(ToolName::Read) => Tool::call_read_file(args),
            _ => Err(format!("Tool {} not found", name)),
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
