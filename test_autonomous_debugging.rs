// Test file to demonstrate autonomous debugging
// This file will be monitored by the autonomous debugger

use std::collections::HashMap;
use anyhow::Result;

/// Example struct that demonstrates various patterns
pub struct TestStruct {
    pub data: HashMap<String, String>,
    pub counter: u32,
}

impl TestStruct {
    /// Constructor with potential unwrap() issue
    pub fn new() -> Self {
        let mut data = HashMap::new();
        data.insert("key".to_string(), "value".to_string());
        
        // This unwrap() will be detected by the autonomous debugger
        let value = data.get("key").unwrap();
        
        Self {
            data,
            counter: 0,
        }
    }
    
    /// Async function pattern
    pub async fn process_data(&mut self) -> Result<()> {
        self.counter += 1;
        Ok(())
    }
    
    /// New function that will trigger autonomous analysis
    pub fn analyze_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        
        // This will be detected as a pattern
        if self.counter > 10 {
            patterns.push("high_activity".to_string());
        }
        
        // This panic! will be detected by the autonomous debugger
        if self.counter > 100 {
            panic!("Counter exceeded limit!");
        }
        
        patterns
    }
}

/// Main function with some complexity
pub async fn main() -> Result<()> {
    let mut test_struct = TestStruct::new();
    test_struct.process_data().await?;
    
    // This panic! will be detected by the autonomous debugger
    if test_struct.counter > 100 {
        panic!("Counter exceeded limit!");
    }
    
    Ok(())
} 