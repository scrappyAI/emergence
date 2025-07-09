//! EMERGENCE Models - Composable AI for Living Agents
//! 
//! This crate provides a composable model system for EMERGENCE agents, supporting
//! both local tiny models and cloud fallbacks. Models are designed to work together
//! as modular components that can be combined and swapped based on task requirements.
//! 
//! Key features:
//! - Composable model architecture with trait-based interfaces
//! - Energy-aware model selection and management
//! - Hybrid local-cloud inference with intelligent fallbacks
//! - Support for researcher essence capabilities (pattern recognition, code analysis, etc.)
//! - Memory-efficient model loading and caching

pub mod intent;
pub mod response;
pub mod memory;
pub mod reasoning;
pub mod manager;
pub mod config;
pub mod composer;
pub mod cloud;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Core capabilities that models can provide
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    /// Intent recognition and classification
    IntentRecognition,
    /// Natural language response generation
    ResponseGeneration,
    /// Semantic memory and embeddings
    MemoryEmbedding,
    /// Complex reasoning and problem solving
    Reasoning,
    /// Code analysis and generation
    CodeAnalysis,
    /// Data synthesis and pattern recognition
    DataSynthesis,
    /// Text summarization and transformation
    TextTransformation,
}

/// Model context for inference
#[derive(Debug, Clone)]
pub struct ModelContext {
    /// Available energy budget
    pub energy_budget: f64,
    /// Agent personality traits
    pub personality: Personality,
    /// Current task complexity
    pub complexity: TaskComplexity,
    /// Memory constraints
    pub memory_limit: usize,
}

/// Agent personality traits (from researcher essence)
#[derive(Debug, Clone)]
pub struct Personality {
    pub curiosity: f64,      // 0.9 in researcher
    pub creativity: f64,     // 0.8 in researcher
    pub skepticism: f64,     // 0.6 in researcher
    pub patience: f64,       // 0.7 in researcher
    pub collaboration: f64,  // 0.7 in researcher
}

/// Task complexity levels
#[derive(Debug, Clone, PartialEq)]
pub enum TaskComplexity {
    Simple,    // Can use local tiny models
    Moderate,  // May need larger local models
    Complex,   // May need cloud fallback
}

/// Model output with metadata
#[derive(Debug, Clone)]
pub struct ModelOutput {
    /// The generated text or result
    pub content: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Energy cost of this inference
    pub energy_cost: f64,
    /// Capabilities used
    pub capabilities_used: Vec<Capability>,
}

/// Composable model trait - all models implement this
#[async_trait]
pub trait ComposableModel: Send + Sync {
    /// Process input and return output
    async fn process(&self, input: &str, context: &ModelContext) -> Result<ModelOutput, ModelError>;
    
    /// Get energy cost for this model
    fn energy_cost(&self) -> f64;
    
    /// Get memory requirement in bytes
    fn memory_requirement(&self) -> usize;
    
    /// Get capabilities this model provides
    fn capabilities(&self) -> Vec<Capability>;
    
    /// Get model name/identifier
    fn name(&self) -> &str;
    
    /// Check if model is loaded and ready
    fn is_ready(&self) -> bool;
    
    /// Clone the model as a boxed trait object
    fn clone_box(&self) -> Box<dyn ComposableModel>;
}

impl Clone for Box<dyn ComposableModel> {
    fn clone(&self) -> Box<dyn ComposableModel> {
        self.clone_box()
    }
}

/// Model types for identification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum ModelType {
    // Local tiny models
    DistilBertTiny,
    Gpt2Small,
    TinyLlama,
    Phi3Mini,
    Gemma2B,
    Qwen25_0_5B,
    SentenceTransformer,
    T5Small,
    
    // Cloud models
    CloudflareLlama2,
    CloudflareMistral,
    CloudflareCodeLlama,
    OpenRouterGpt4,
    OpenRouterClaude,
    OpenRouterCodeLlama,
}

/// Energy profile for a model
#[derive(Debug, Clone)]
pub struct EnergyProfile {
    pub cost_per_token: f64,
    pub quality_score: f64,
    pub reasoning_score: f64,
    pub code_score: f64,
    pub synthesis_score: f64,
}

/// Task definition for model composition
#[derive(Debug, Clone)]
pub struct Task {
    pub input: String,
    pub required_capabilities: Vec<Capability>,
    pub complexity: TaskComplexity,
    pub energy_budget: f64,
    pub memory_limit: usize,
}

/// Model pipeline for complex tasks
pub struct ModelPipeline {
    models: Vec<Box<dyn ComposableModel>>,
    total_energy_cost: f64,
    total_memory_requirement: usize,
}

impl ModelPipeline {
    pub fn new(models: Vec<Box<dyn ComposableModel>>) -> Self {
        let total_energy_cost = models.iter().map(|m| m.energy_cost()).sum();
        let total_memory_requirement = models.iter().map(|m| m.memory_requirement()).sum();
        
        Self {
            models,
            total_energy_cost,
            total_memory_requirement,
        }
    }
    
    /// Execute the pipeline on input
    pub async fn execute(&self, input: &str, context: &ModelContext) -> Result<String, ModelError> {
        let mut current_input = input.to_string();
        
        for model in &self.models {
            let output = model.process(&current_input, context).await?;
            current_input = output.content;
        }
        
        Ok(current_input)
    }
    
    pub fn total_energy_cost(&self) -> f64 {
        self.total_energy_cost
    }
    
    pub fn total_memory_requirement(&self) -> usize {
        self.total_memory_requirement
    }
}

/// Model errors
#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Model not ready: {0}")]
    NotReady(String),
    
    #[error("Insufficient energy: required {required}, available {available}")]
    InsufficientEnergy { required: f64, available: f64 },
    
    #[error("Insufficient memory: required {required}, available {available}")]
    InsufficientMemory { required: usize, available: usize },
    
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Inference failed: {0}")]
    InferenceFailed(String),
    
    #[error("Cloud API error: {0}")]
    CloudApiError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Default personality for researcher essence
impl Default for Personality {
    fn default() -> Self {
        Self {
            curiosity: 0.9,
            creativity: 0.8,
            skepticism: 0.6,
            patience: 0.7,
            collaboration: 0.7,
        }
    }
}

/// Default model context
impl Default for ModelContext {
    fn default() -> Self {
        Self {
            energy_budget: 1.0,
            personality: Personality::default(),
            complexity: TaskComplexity::Simple,
            memory_limit: 1024 * 1024 * 1024, // 1GB
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_personality_default() {
        let personality = Personality::default();
        assert_eq!(personality.curiosity, 0.9);
        assert_eq!(personality.creativity, 0.8);
        assert_eq!(personality.skepticism, 0.6);
        assert_eq!(personality.patience, 0.7);
        assert_eq!(personality.collaboration, 0.7);
    }
    
    #[test]
    fn test_model_context_default() {
        let context = ModelContext::default();
        assert_eq!(context.energy_budget, 1.0);
        assert_eq!(context.memory_limit, 1024 * 1024 * 1024);
        assert_eq!(context.complexity, TaskComplexity::Simple);
    }
} 