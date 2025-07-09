//! Model Composer - Composable Model Architecture
//! 
//! This module implements the composable model system that can combine multiple
//! models into pipelines for complex tasks, with energy-aware selection and
//! intelligent fallback strategies.

use super::*;
use std::collections::HashMap;

/// Composition rule for model pipelines
#[derive(Debug, Clone)]
pub struct CompositionRule {
    /// Required capabilities for this rule
    pub required_capabilities: Vec<Capability>,
    /// Preferred model types in order
    pub preferred_models: Vec<ModelType>,
    /// Maximum energy cost for this composition
    pub max_energy_cost: f64,
    /// Maximum memory usage for this composition
    pub max_memory_usage: usize,
}

/// Model composer that creates optimized pipelines
pub struct ModelComposer {
    /// Available models indexed by type
    models: HashMap<ModelType, Box<dyn ComposableModel>>,
    /// Composition rules for different task types
    composition_rules: Vec<CompositionRule>,
    /// Energy profiles for each model type
    energy_profiles: HashMap<ModelType, EnergyProfile>,
    /// Current energy budget
    energy_budget: f64,
    /// Current memory limit
    memory_limit: usize,
}

impl ModelComposer {
    /// Create a new model composer
    pub fn new(energy_budget: f64, memory_limit: usize) -> Self {
        let mut composer = Self {
            models: HashMap::new(),
            composition_rules: Self::default_composition_rules(),
            energy_profiles: Self::default_energy_profiles(),
            energy_budget,
            memory_limit,
        };
        
        composer
    }
    
    /// Add a model to the composer
    pub fn add_model(&mut self, model_type: ModelType, model: Box<dyn ComposableModel>) {
        self.models.insert(model_type, model);
    }
    
    /// Compose a pipeline for a given task
    pub async fn compose_pipeline(&self, task: &Task) -> Result<ModelPipeline, ModelError> {
        // Find models that can handle each required capability
        let mut selected_models = Vec::new();
        let mut used_energy = 0.0;
        let mut used_memory = 0;
        
        for capability in &task.required_capabilities {
            let models_for_capability = self.find_models_for_capability(capability);
            
            // Find the best model that fits within constraints
            let best_model = self.select_best_model(
                &models_for_capability,
                task.energy_budget - used_energy,
                task.memory_limit - used_memory,
                capability,
            )?;
            
            // Check if we can add this model
            let model_energy = best_model.energy_cost();
            let model_memory = best_model.memory_requirement();
            
            if used_energy + model_energy > task.energy_budget {
                return Err(ModelError::InsufficientEnergy {
                    required: used_energy + model_energy,
                    available: task.energy_budget,
                });
            }
            
            if used_memory + model_memory > task.memory_limit {
                return Err(ModelError::InsufficientMemory {
                    required: used_memory + model_memory,
                    available: task.memory_limit,
                });
            }
            
            selected_models.push(best_model);
            used_energy += model_energy;
            used_memory += model_memory;
        }
        
        // Optimize pipeline order
        self.optimize_pipeline_order(&mut selected_models);
        
        Ok(ModelPipeline::new(selected_models))
    }
    
    /// Find models that provide a specific capability
    fn find_models_for_capability(&self, capability: &Capability) -> Vec<&dyn ComposableModel> {
        self.models
            .iter()
            .filter(|(_, model)| model.capabilities().contains(capability))
            .map(|(_, model)| model.as_ref())
            .collect()
    }
    
    /// Select the best model for a capability within constraints
    fn select_best_model(
        &self,
        models: &[&dyn ComposableModel],
        available_energy: f64,
        available_memory: usize,
        capability: &Capability,
    ) -> Result<Box<dyn ComposableModel>, ModelError> {
        let mut best_model = None;
        let mut best_score = 0.0;
        
        for model in models {
            // Check constraints
            if model.energy_cost() > available_energy {
                continue;
            }
            
            if model.memory_requirement() > available_memory {
                continue;
            }
            
            // Calculate quality score based on capability
            let score = self.calculate_model_score(&**model, capability);
            
            if score > best_score {
                best_score = score;
                best_model = Some(model);
            }
        }
        
        best_model
            .map(|model| {
                // Clone the model (this would need proper cloning in real implementation)
                // For now, we'll return an error if we can't clone
                Err(ModelError::ModelNotFound("Model cloning not implemented".to_string()))
            })
            .unwrap_or_else(|| {
                Err(ModelError::ModelNotFound(format!(
                    "No suitable model found for capability {:?}",
                    capability
                )))
            })
    }
    
    /// Calculate a quality score for a model based on capability
    fn calculate_model_score(&self, model: &dyn ComposableModel, capability: &Capability) -> f64 {
        let model_type = self.get_model_type_from_name(model.name());
        
        if let Some(profile) = self.energy_profiles.get(&model_type) {
            match capability {
                Capability::Reasoning => profile.reasoning_score,
                Capability::CodeAnalysis => profile.code_score,
                Capability::DataSynthesis => profile.synthesis_score,
                _ => profile.quality_score,
            }
        } else {
            0.5 // Default score
        }
    }
    
    /// Optimize the order of models in a pipeline
    fn optimize_pipeline_order(&self, models: &mut Vec<Box<dyn ComposableModel>>) {
        // Simple optimization: put faster models first
        models.sort_by(|a, b| {
            let a_speed = 1.0 / a.energy_cost(); // Higher energy cost = slower
            let b_speed = 1.0 / b.energy_cost();
            b_speed.partial_cmp(&a_speed).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    
    /// Get model type from model name
    fn get_model_type_from_name(&self, name: &str) -> ModelType {
        match name {
            "distilbert-tiny" => ModelType::DistilBertTiny,
            "gpt2-small" => ModelType::Gpt2Small,
            "tinyllama" => ModelType::TinyLlama,
            "phi3-mini" => ModelType::Phi3Mini,
            "gemma2b" => ModelType::Gemma2B,
            "qwen2.5-0.5b" => ModelType::Qwen25_0_5B,
            "sentence-transformer" => ModelType::SentenceTransformer,
            "t5-small" => ModelType::T5Small,
            _ => ModelType::Gpt2Small, // Default
        }
    }
    
    /// Default composition rules
    fn default_composition_rules() -> Vec<CompositionRule> {
        vec![
            // Simple intent + response
            CompositionRule {
                required_capabilities: vec![Capability::IntentRecognition, Capability::ResponseGeneration],
                preferred_models: vec![ModelType::DistilBertTiny, ModelType::Gpt2Small],
                max_energy_cost: 0.1,
                max_memory_usage: 600 * 1024 * 1024, // 600MB
            },
            // Complex reasoning
            CompositionRule {
                required_capabilities: vec![Capability::Reasoning, Capability::CodeAnalysis],
                preferred_models: vec![ModelType::Phi3Mini, ModelType::TinyLlama],
                max_energy_cost: 0.3,
                max_memory_usage: 8 * 1024 * 1024 * 1024, // 8GB
            },
            // Memory + synthesis
            CompositionRule {
                required_capabilities: vec![Capability::MemoryEmbedding, Capability::DataSynthesis],
                preferred_models: vec![ModelType::SentenceTransformer, ModelType::Gemma2B],
                max_energy_cost: 0.2,
                max_memory_usage: 4 * 1024 * 1024 * 1024, // 4GB
            },
        ]
    }
    
    /// Default energy profiles for each model type
    fn default_energy_profiles() -> HashMap<ModelType, EnergyProfile> {
        let mut profiles = HashMap::new();
        
        // Tiny models - low energy, lower quality
        profiles.insert(ModelType::DistilBertTiny, EnergyProfile {
            cost_per_token: 0.001,
            quality_score: 0.6,
            reasoning_score: 0.3,
            code_score: 0.2,
            synthesis_score: 0.4,
        });
        
        profiles.insert(ModelType::Gpt2Small, EnergyProfile {
            cost_per_token: 0.005,
            quality_score: 0.7,
            reasoning_score: 0.5,
            code_score: 0.4,
            synthesis_score: 0.6,
        });
        
        // Medium models - balanced
        profiles.insert(ModelType::TinyLlama, EnergyProfile {
            cost_per_token: 0.02,
            quality_score: 0.8,
            reasoning_score: 0.7,
            code_score: 0.8,
            synthesis_score: 0.6,
        });
        
        profiles.insert(ModelType::Phi3Mini, EnergyProfile {
            cost_per_token: 0.03,
            quality_score: 0.85,
            reasoning_score: 0.8,
            code_score: 0.7,
            synthesis_score: 0.7,
        });
        
        profiles.insert(ModelType::Gemma2B, EnergyProfile {
            cost_per_token: 0.04,
            quality_score: 0.8,
            reasoning_score: 0.6,
            code_score: 0.6,
            synthesis_score: 0.8,
        });
        
        // Specialized models
        profiles.insert(ModelType::SentenceTransformer, EnergyProfile {
            cost_per_token: 0.002,
            quality_score: 0.9,
            reasoning_score: 0.3,
            code_score: 0.2,
            synthesis_score: 0.9,
        });
        
        profiles.insert(ModelType::T5Small, EnergyProfile {
            cost_per_token: 0.01,
            quality_score: 0.75,
            reasoning_score: 0.4,
            code_score: 0.3,
            synthesis_score: 0.7,
        });
        
        profiles
    }
    
    /// Get current energy budget
    pub fn energy_budget(&self) -> f64 {
        self.energy_budget
    }
    
    /// Get current memory limit
    pub fn memory_limit(&self) -> usize {
        self.memory_limit
    }
    
    /// Update energy budget
    pub fn update_energy_budget(&mut self, new_budget: f64) {
        self.energy_budget = new_budget;
    }
    
    /// Update memory limit
    pub fn update_memory_limit(&mut self, new_limit: usize) {
        self.memory_limit = new_limit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_composer_creation() {
        let composer = ModelComposer::new(1.0, 1024 * 1024 * 1024);
        assert_eq!(composer.energy_budget(), 1.0);
        assert_eq!(composer.memory_limit(), 1024 * 1024 * 1024);
    }
    
    #[test]
    fn test_energy_profiles() {
        let profiles = ModelComposer::default_energy_profiles();
        assert!(profiles.contains_key(&ModelType::DistilBertTiny));
        assert!(profiles.contains_key(&ModelType::Phi3Mini));
        
        let distilbert = &profiles[&ModelType::DistilBertTiny];
        assert_eq!(distilbert.cost_per_token, 0.001);
        assert_eq!(distilbert.quality_score, 0.6);
    }
    
    #[test]
    fn test_composition_rules() {
        let rules = ModelComposer::default_composition_rules();
        assert_eq!(rules.len(), 3);
        
        let simple_rule = &rules[0];
        assert!(simple_rule.required_capabilities.contains(&Capability::IntentRecognition));
        assert!(simple_rule.required_capabilities.contains(&Capability::ResponseGeneration));
        assert_eq!(simple_rule.max_energy_cost, 0.1);
    }
} 