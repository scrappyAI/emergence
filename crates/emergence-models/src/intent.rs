//! Intent Recognition Model
//! 
//! Provides intent classification capabilities for EMERGENCE agents.
//! Currently implements mock functionality that can be replaced with
//! real model inference when dependencies are available.

use super::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Intent classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// The classified intent
    pub intent_type: IntentType,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Types of intents that can be recognized
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntentType {
    /// User wants to analyze something
    Analyze,
    /// User wants to investigate something
    Investigate,
    /// User wants to create something
    Create,
    /// User wants to modify something
    Modify,
    /// User wants to understand something
    Understand,
    /// User wants to collaborate
    Collaborate,
    /// User wants to learn
    Learn,
    /// User wants to teach
    Teach,
    /// User wants to explore
    Explore,
    /// User wants to document
    Document,
    /// Unknown or unclear intent
    Unknown,
}

/// Intent recognition model implementing the composable interface
pub struct IntentModel {
    /// Model name
    name: String,
    /// Intent labels
    labels: Vec<String>,
    /// Mock confidence scores for different intents
    mock_confidence: HashMap<IntentType, f64>,
    /// Whether the model is ready
    ready: bool,
}

impl IntentModel {
    /// Create a new intent recognition model
    pub fn new() -> Self {
        let mut mock_confidence = HashMap::new();
        mock_confidence.insert(IntentType::Analyze, 0.85);
        mock_confidence.insert(IntentType::Investigate, 0.90);
        mock_confidence.insert(IntentType::Create, 0.75);
        mock_confidence.insert(IntentType::Modify, 0.80);
        mock_confidence.insert(IntentType::Understand, 0.88);
        mock_confidence.insert(IntentType::Collaborate, 0.70);
        mock_confidence.insert(IntentType::Learn, 0.82);
        mock_confidence.insert(IntentType::Teach, 0.78);
        mock_confidence.insert(IntentType::Explore, 0.92);
        mock_confidence.insert(IntentType::Document, 0.76);
        mock_confidence.insert(IntentType::Unknown, 0.50);
        
        Self {
            name: "distilbert-tiny".to_string(),
            labels: vec![
                "analyze".to_string(),
                "investigate".to_string(),
                "create".to_string(),
                "modify".to_string(),
                "understand".to_string(),
                "collaborate".to_string(),
                "learn".to_string(),
                "teach".to_string(),
                "explore".to_string(),
                "document".to_string(),
            ],
            mock_confidence,
            ready: true,
        }
    }
    
    /// Classify intent from text
    pub async fn classify_intent(&self, text: &str) -> Result<Intent, ModelError> {
        if !self.ready {
            return Err(ModelError::NotReady("Intent model not loaded".to_string()));
        }
        
        // Simple keyword-based classification for now
        let text_lower = text.to_lowercase();
        
        let intent_type = if text_lower.contains("analyze") || text_lower.contains("analysis") {
            IntentType::Analyze
        } else if text_lower.contains("investigate") || text_lower.contains("investigation") {
            IntentType::Investigate
        } else if text_lower.contains("create") || text_lower.contains("build") {
            IntentType::Create
        } else if text_lower.contains("modify") || text_lower.contains("change") {
            IntentType::Modify
        } else if text_lower.contains("understand") || text_lower.contains("comprehend") {
            IntentType::Understand
        } else if text_lower.contains("collaborate") || text_lower.contains("work together") {
            IntentType::Collaborate
        } else if text_lower.contains("learn") || text_lower.contains("study") {
            IntentType::Learn
        } else if text_lower.contains("teach") || text_lower.contains("explain") {
            IntentType::Teach
        } else if text_lower.contains("explore") || text_lower.contains("discover") {
            IntentType::Explore
        } else if text_lower.contains("document") || text_lower.contains("write") {
            IntentType::Document
        } else {
            IntentType::Unknown
        };
        
        let confidence = *self.mock_confidence.get(&intent_type).unwrap_or(&0.5);
        
        let mut metadata = HashMap::new();
        metadata.insert("text_length".to_string(), serde_json::Value::Number(text.len().into()));
        metadata.insert("keywords_found".to_string(), serde_json::Value::Number(
            text_lower.split_whitespace().count().into()
        ));
        
        Ok(Intent {
            intent_type,
            confidence,
            metadata,
        })
    }
}

impl Clone for IntentModel {
    fn clone(&self) -> Self {
        IntentModel {
            labels: self.labels.clone(),
            mock_confidence: self.mock_confidence.clone(),
            name: self.name.clone(),
            ready: self.ready,
        }
    }
}

#[async_trait]
impl ComposableModel for IntentModel {
    async fn process(&self, input: &str, context: &ModelContext) -> Result<ModelOutput, ModelError> {
        let intent = self.classify_intent(input).await?;
        
        let content = format!("Intent: {:?} (confidence: {:.2})", intent.intent_type, intent.confidence);
        
        Ok(ModelOutput {
            content,
            confidence: intent.confidence,
            energy_cost: self.energy_cost(),
            capabilities_used: vec![Capability::IntentRecognition],
        })
    }
    
    fn energy_cost(&self) -> f64 {
        0.001 // Very low energy cost for intent recognition
    }
    
    fn memory_requirement(&self) -> usize {
        50 * 1024 * 1024 // 50MB
    }
    
    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::IntentRecognition]
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_ready(&self) -> bool {
        self.ready
    }

    fn clone_box(&self) -> Box<dyn ComposableModel> {
        Box::new(self.clone())
    }
}

impl Default for IntentModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_intent_classification() {
        let model = IntentModel::new();
        
        let intent = model.classify_intent("I want to analyze the performance patterns").await.unwrap();
        assert_eq!(intent.intent_type, IntentType::Analyze);
        assert!(intent.confidence > 0.8);
        
        let intent = model.classify_intent("Let's investigate this issue").await.unwrap();
        assert_eq!(intent.intent_type, IntentType::Investigate);
        assert!(intent.confidence > 0.8);
        
        let intent = model.classify_intent("Unknown text here").await.unwrap();
        assert_eq!(intent.intent_type, IntentType::Unknown);
        assert!(intent.confidence <= 0.6);
    }
    
    #[tokio::test]
    async fn test_composable_interface() {
        let model = IntentModel::new();
        let context = ModelContext::default();
        
        let output = model.process("analyze the system", &context).await.unwrap();
        assert!(output.content.contains("Intent:"));
        assert!(output.confidence > 0.0);
        assert_eq!(output.capabilities_used, vec![Capability::IntentRecognition]);
    }
    
    #[test]
    fn test_model_properties() {
        let model = IntentModel::new();
        
        assert_eq!(model.name(), "distilbert-tiny");
        assert!(model.is_ready());
        assert_eq!(model.capabilities(), vec![Capability::IntentRecognition]);
        assert_eq!(model.energy_cost(), 0.001);
        assert_eq!(model.memory_requirement(), 50 * 1024 * 1024);
    }
} 