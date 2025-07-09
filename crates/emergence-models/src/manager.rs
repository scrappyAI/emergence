//! Model Manager - Energy-Aware Model Management
//! 
//! Provides intelligent model loading, caching, and energy-aware inference
//! for the composable model architecture.

use super::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Model manager for energy-aware model management
pub struct ModelManager {
    /// Available models indexed by type
    models: Arc<RwLock<HashMap<ModelType, Box<dyn ComposableModel>>>>,
    /// Model loading cache
    cache: Arc<RwLock<ModelCache>>,
    /// Energy budget tracker
    energy_tracker: Arc<RwLock<EnergyTracker>>,
    /// Model metrics
    metrics: Arc<RwLock<HashMap<ModelType, ModelMetrics>>>,
}

/// Model cache for efficient loading
#[derive(Debug, Clone)]
pub struct ModelCache {
    /// Maximum number of models to keep in memory
    pub max_models: usize,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: usize,
    /// Time to live for unused models (seconds)
    pub ttl_seconds: u64,
    /// Whether to enable caching
    pub enable_caching: bool,
}

/// Energy tracker for model inference
#[derive(Debug, Clone)]
pub struct EnergyTracker {
    /// Current energy budget
    pub energy_budget: f64,
    /// Energy consumption history
    pub consumption_history: Vec<EnergyConsumption>,
    /// Energy cost per model type
    pub model_energy_costs: HashMap<ModelType, f64>,
}

/// Energy consumption record
#[derive(Debug, Clone)]
pub struct EnergyConsumption {
    /// Model type that consumed energy
    pub model_type: ModelType,
    /// Energy consumed
    pub energy_consumed: f64,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Model metrics for performance tracking
#[derive(Debug, Clone)]
pub struct ModelMetrics {
    /// Total number of inferences
    pub total_inferences: u64,
    /// Average inference time (milliseconds)
    pub avg_inference_time_ms: u64,
    /// Total energy consumed
    pub total_energy_consumed: f64,
    /// Average confidence score
    pub avg_confidence: f64,
    /// Success rate (0.0-1.0)
    pub success_rate: f64,
    /// Last used timestamp
    pub last_used: chrono::DateTime<chrono::Utc>,
}

impl ModelManager {
    /// Create a new model manager
    pub fn new(energy_budget: f64) -> Self {
        let cache = ModelCache {
            max_models: 5,
            max_memory_bytes: 8 * 1024 * 1024 * 1024, // 8GB
            ttl_seconds: 3600, // 1 hour
            enable_caching: true,
        };
        
        let energy_tracker = EnergyTracker {
            energy_budget,
            consumption_history: Vec::new(),
            model_energy_costs: Self::initialize_energy_costs(),
        };
        
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(RwLock::new(cache)),
            energy_tracker: Arc::new(RwLock::new(energy_tracker)),
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add a model to the manager
    pub async fn add_model(&self, model_type: ModelType, model: Box<dyn ComposableModel>) {
        let mut models = self.models.write().await;
        models.insert(model_type.clone(), model);
        info!("Added model: {:?}", model_type);
    }
    
    /// Get a model by type
    pub async fn get_model(&self, model_type: ModelType) -> Option<Box<dyn ComposableModel>> {
        let models = self.models.read().await;
        models.get(&model_type).map(|m| m.clone())
    }
    
    /// Run inference with energy constraints
    pub async fn run_inference(
        &self,
        model_type: ModelType,
        input: &str,
        context: &ModelContext,
    ) -> Result<ModelOutput, ModelError> {
        let start_time = std::time::Instant::now();
        
        // Check energy budget
        let energy_tracker = self.energy_tracker.read().await;
        let estimated_cost = energy_tracker.model_energy_costs.get(&model_type).copied().unwrap_or(0.1);
        
        if energy_tracker.energy_budget < estimated_cost {
            return Err(ModelError::InsufficientEnergy {
                required: estimated_cost,
                available: energy_tracker.energy_budget,
            });
        }
        
        // Get model
        let model = self.get_model(model_type).await
            .ok_or_else(|| ModelError::ModelNotFound(format!("Model {:?} not found", model_type)))?;
        
        // Run inference
        let result = model.process(input, context).await?;
        let inference_time = start_time.elapsed();
        // Update metrics
        self.update_metrics(model_type.clone(), &result, inference_time).await;
        // Update energy consumption
        self.update_energy_consumption(model_type.clone(), result.energy_cost).await;
        info!("Inference completed for {:?} in {:?}", model_type, inference_time);
        
        Ok(result)
    }
    
    /// Update model metrics
    async fn update_metrics(
        &self,
        model_type: ModelType,
        result: &ModelOutput,
        inference_time: std::time::Duration,
    ) {
        let mut metrics = self.metrics.write().await;
        let model_metrics = metrics.entry(model_type).or_insert_with(|| ModelMetrics {
            total_inferences: 0,
            avg_inference_time_ms: 0,
            total_energy_consumed: 0.0,
            avg_confidence: 0.0,
            success_rate: 1.0,
            last_used: chrono::Utc::now(),
        });
        
        // Update metrics
        model_metrics.total_inferences += 1;
        model_metrics.total_energy_consumed += result.energy_cost;
        model_metrics.last_used = chrono::Utc::now();
        
        // Update average inference time
        let total_time = model_metrics.avg_inference_time_ms as f64 * (model_metrics.total_inferences - 1) as f64
            + inference_time.as_millis() as f64;
        model_metrics.avg_inference_time_ms = (total_time / model_metrics.total_inferences as f64) as u64;
        
        // Update average confidence
        let total_confidence = model_metrics.avg_confidence * (model_metrics.total_inferences - 1) as f64
            + result.confidence;
        model_metrics.avg_confidence = total_confidence / model_metrics.total_inferences as f64;
    }
    
    /// Update energy consumption
    async fn update_energy_consumption(&self, model_type: ModelType, energy_consumed: f64) {
        let mut energy_tracker = self.energy_tracker.write().await;
        energy_tracker.energy_budget -= energy_consumed;
        
        energy_tracker.consumption_history.push(EnergyConsumption {
            model_type,
            energy_consumed,
            timestamp: chrono::Utc::now(),
        });
        
        // Keep only last 1000 consumption records
        if energy_tracker.consumption_history.len() > 1000 {
            energy_tracker.consumption_history.drain(0..100);
        }
    }
    
    /// Get current energy budget
    pub async fn energy_budget(&self) -> f64 {
        let energy_tracker = self.energy_tracker.read().await;
        energy_tracker.energy_budget
    }
    
    /// Update energy budget
    pub async fn update_energy_budget(&self, new_budget: f64) {
        let mut energy_tracker = self.energy_tracker.write().await;
        energy_tracker.energy_budget = new_budget;
        info!("Updated energy budget to: {}", new_budget);
    }
    
    /// Get model metrics
    pub async fn get_metrics(&self, model_type: ModelType) -> Option<ModelMetrics> {
        let metrics = self.metrics.read().await;
        metrics.get(&model_type).cloned()
    }
    
    /// Get all model metrics
    pub async fn get_all_metrics(&self) -> HashMap<ModelType, ModelMetrics> {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
    
    /// Initialize energy costs for different model types
    fn initialize_energy_costs() -> HashMap<ModelType, f64> {
        let mut costs = HashMap::new();
        
        // Local models
        costs.insert(ModelType::DistilBertTiny, 0.001);
        costs.insert(ModelType::Gpt2Small, 0.005);
        costs.insert(ModelType::TinyLlama, 0.02);
        costs.insert(ModelType::Phi3Mini, 0.03);
        costs.insert(ModelType::Gemma2B, 0.04);
        costs.insert(ModelType::Qwen25_0_5B, 0.01);
        costs.insert(ModelType::SentenceTransformer, 0.002);
        costs.insert(ModelType::T5Small, 0.01);
        
        // Cloud models (higher costs)
        costs.insert(ModelType::CloudflareLlama2, 0.05);
        costs.insert(ModelType::CloudflareMistral, 0.05);
        costs.insert(ModelType::CloudflareCodeLlama, 0.06);
        costs.insert(ModelType::OpenRouterGpt4, 0.1);
        costs.insert(ModelType::OpenRouterClaude, 0.1);
        costs.insert(ModelType::OpenRouterCodeLlama, 0.08);
        
        costs
    }
    
    /// Get cache configuration
    pub async fn get_cache_config(&self) -> ModelCache {
        let cache = self.cache.read().await;
        cache.clone()
    }
    
    /// Update cache configuration
    pub async fn update_cache_config(&self, config: ModelCache) {
        let mut cache = self.cache.write().await;
        *cache = config.clone();
        info!("Updated cache configuration: {:?}", config);
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_model_manager_creation() {
        let manager = ModelManager::new(1.0);
        assert_eq!(manager.energy_budget().await, 1.0);
    }
    
    #[tokio::test]
    async fn test_add_and_get_model() {
        let manager = ModelManager::new(1.0);
        let intent_model = Box::new(super::intent::IntentModel::new());
        
        manager.add_model(ModelType::DistilBertTiny, intent_model).await;
        
        let retrieved_model = manager.get_model(ModelType::DistilBertTiny).await;
        assert!(retrieved_model.is_some());
    }
    
    #[tokio::test]
    async fn test_energy_budget_management() {
        let manager = ModelManager::new(1.0);
        
        assert_eq!(manager.energy_budget().await, 1.0);
        
        manager.update_energy_budget(0.5).await;
        assert_eq!(manager.energy_budget().await, 0.5);
    }
    
    #[tokio::test]
    async fn test_cache_configuration() {
        let manager = ModelManager::new(1.0);
        
        let config = manager.get_cache_config().await;
        assert_eq!(config.max_models, 5);
        assert_eq!(config.max_memory_bytes, 8 * 1024 * 1024 * 1024);
        assert_eq!(config.ttl_seconds, 3600);
        assert!(config.enable_caching);
    }
} 