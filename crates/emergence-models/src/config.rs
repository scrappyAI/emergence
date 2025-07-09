//! Configuration for EMERGENCE models

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::info;

/// Main model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// General inference settings
    pub inference: InferenceConfig,
    
    /// Energy management settings
    pub energy: EnergyConfig,
    
    /// Model-specific configurations
    pub models: HashMap<String, IndividualModelConfig>,
    
    /// Cache settings
    pub cache: CacheConfig,
    
    /// Performance settings
    pub performance: PerformanceConfig,
}

/// Inference configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Default device to use (cpu, cuda, mps)
    pub device: String,
    
    /// Maximum batch size for inference
    pub max_batch_size: usize,
    
    /// Maximum sequence length
    pub max_sequence_length: usize,
    
    /// Whether to use mixed precision
    pub use_mixed_precision: bool,
    
    /// Number of threads for CPU inference
    pub num_threads: usize,
    
    /// Whether to enable model caching
    pub enable_model_caching: bool,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            device: "cpu".to_string(),
            max_batch_size: 1,
            max_sequence_length: 2048,
            use_mixed_precision: false,
            num_threads: num_cpus::get(),
            enable_model_caching: true,
        }
    }
}

/// Energy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyConfig {
    /// Total energy budget for all models
    pub total_energy_budget: f64,
    
    /// Energy cost per token for different model types
    pub energy_per_token: HashMap<String, f64>,
    
    /// Energy decay rate per second
    pub energy_decay_rate: f64,
    
    /// Energy recovery rate per second
    pub energy_recovery_rate: f64,
    
    /// Whether to enable energy-aware inference
    pub enable_energy_aware_inference: bool,
    
    /// Energy threshold for model eviction
    pub energy_eviction_threshold: f64,
}

impl Default for EnergyConfig {
    fn default() -> Self {
        let mut energy_per_token = HashMap::new();
        energy_per_token.insert("intent_recognition".to_string(), 0.001);
        energy_per_token.insert("response_generation".to_string(), 0.005);
        energy_per_token.insert("memory_embedding".to_string(), 0.002);
        energy_per_token.insert("reasoning".to_string(), 0.01);
        energy_per_token.insert("code_generation".to_string(), 0.02);
        
        Self {
            total_energy_budget: 1.0,
            energy_per_token,
            energy_decay_rate: 0.01,
            energy_recovery_rate: 0.005,
            enable_energy_aware_inference: true,
            energy_eviction_threshold: 0.1,
        }
    }
}

/// Individual model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualModelConfig {
    /// Model name/identifier
    pub name: String,
    
    /// Model type
    pub model_type: String,
    
    /// Model file path
    pub model_path: Option<PathBuf>,
    
    /// Tokenizer file path
    pub tokenizer_path: Option<PathBuf>,
    
    /// Whether the model is enabled
    pub enabled: bool,
    
    /// Memory limit in bytes
    pub memory_limit_bytes: usize,
    
    /// Energy budget for this model
    pub energy_budget: f64,
    
    /// Priority for loading (higher = more important)
    pub priority: u32,
    
    /// Model-specific parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of models to cache
    pub max_cached_models: usize,
    
    /// Maximum memory for cached models
    pub max_cache_memory_bytes: usize,
    
    /// Time to live for cached models (seconds)
    pub cache_ttl_seconds: u64,
    
    /// Whether to enable model quantization
    pub enable_quantization: bool,
    
    /// Quantization precision (8, 16, 32)
    pub quantization_precision: u8,
    
    /// Whether to enable model compression
    pub enable_compression: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_cached_models: 5,
            max_cache_memory_bytes: 2 * 1024 * 1024 * 1024, // 2GB
            cache_ttl_seconds: 3600,
            enable_quantization: true,
            quantization_precision: 8,
            enable_compression: false,
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Target latency in milliseconds
    pub target_latency_ms: u64,
    
    /// Maximum memory usage percentage
    pub max_memory_percentage: f64,
    
    /// Whether to enable performance monitoring
    pub enable_monitoring: bool,
    
    /// Performance metrics collection interval (seconds)
    pub metrics_interval_seconds: u64,
    
    /// Whether to enable automatic optimization
    pub enable_auto_optimization: bool,
    
    /// Optimization strategies to use
    pub optimization_strategies: Vec<String>,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            target_latency_ms: 100,
            max_memory_percentage: 80.0,
            enable_monitoring: true,
            metrics_interval_seconds: 60,
            enable_auto_optimization: true,
            optimization_strategies: vec![
                "quantization".to_string(),
                "pruning".to_string(),
                "caching".to_string(),
            ],
        }
    }
}

impl ModelConfig {
    /// Load configuration from file
    pub fn from_file(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: ModelConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Get configuration for a specific model
    pub fn get_model_config(&self, model_name: &str) -> Option<&IndividualModelConfig> {
        self.models.get(model_name)
    }
    
    /// Check if a model is enabled
    pub fn is_model_enabled(&self, model_name: &str) -> bool {
        self.models.get(model_name)
            .map(|config| config.enabled)
            .unwrap_or(false)
    }
    
    /// Get energy cost for a model type
    pub fn get_energy_cost(&self, model_type: &str) -> f64 {
        self.energy.energy_per_token.get(model_type)
            .copied()
            .unwrap_or(0.01) // Default energy cost
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Validate energy budget
        if self.energy.total_energy_budget <= 0.0 {
            errors.push("Total energy budget must be positive".to_string());
        }
        
        // Validate memory limits
        if self.cache.max_cache_memory_bytes == 0 {
            errors.push("Cache memory limit must be positive".to_string());
        }
        
        // Validate model configurations
        for (name, config) in &self.models {
            if config.memory_limit_bytes == 0 {
                errors.push(format!("Model '{}' memory limit must be positive", name));
            }
            
            if config.energy_budget < 0.0 {
                errors.push(format!("Model '{}' energy budget must be non-negative", name));
            }
        }
        
        // Validate performance settings
        if self.performance.target_latency_ms == 0 {
            errors.push("Target latency must be positive".to_string());
        }
        
        if self.performance.max_memory_percentage <= 0.0 || self.performance.max_memory_percentage > 100.0 {
            errors.push("Memory percentage must be between 0 and 100".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    /// Create default configuration
    pub fn default() -> Self {
        let mut models = HashMap::new();
        
        // Intent recognition model
        models.insert("intent_recognition".to_string(), IndividualModelConfig {
            name: "intent_recognition".to_string(),
            model_type: "distilbert".to_string(),
            model_path: None,
            tokenizer_path: None,
            enabled: true,
            memory_limit_bytes: 100 * 1024 * 1024, // 100MB
            energy_budget: 0.1,
            priority: 1,
            parameters: HashMap::new(),
        });
        
        // Response generation model
        models.insert("response_generation".to_string(), IndividualModelConfig {
            name: "response_generation".to_string(),
            model_type: "gpt2".to_string(),
            model_path: None,
            tokenizer_path: None,
            enabled: true,
            memory_limit_bytes: 500 * 1024 * 1024, // 500MB
            energy_budget: 0.2,
            priority: 2,
            parameters: HashMap::new(),
        });
        
        // Memory embedding model
        models.insert("memory_embedding".to_string(), IndividualModelConfig {
            name: "memory_embedding".to_string(),
            model_type: "sentence_transformer".to_string(),
            model_path: None,
            tokenizer_path: None,
            enabled: true,
            memory_limit_bytes: 200 * 1024 * 1024, // 200MB
            energy_budget: 0.15,
            priority: 3,
            parameters: HashMap::new(),
        });
        
        // Reasoning model
        models.insert("reasoning".to_string(), IndividualModelConfig {
            name: "reasoning".to_string(),
            model_type: "tinyllama".to_string(),
            model_path: None,
            tokenizer_path: None,
            enabled: true,
            memory_limit_bytes: 4 * 1024 * 1024 * 1024, // 4GB
            energy_budget: 0.3,
            priority: 4,
            parameters: HashMap::new(),
        });
        
        Self {
            inference: InferenceConfig::default(),
            energy: EnergyConfig::default(),
            models,
            cache: CacheConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

/// Configuration builder for easy configuration creation
pub struct ModelConfigBuilder {
    config: ModelConfig,
}

impl ModelConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            config: ModelConfig::default(),
        }
    }
    
    /// Set inference configuration
    pub fn inference(mut self, inference: InferenceConfig) -> Self {
        self.config.inference = inference;
        self
    }
    
    /// Set energy configuration
    pub fn energy(mut self, energy: EnergyConfig) -> Self {
        self.config.energy = energy;
        self
    }
    
    /// Set cache configuration
    pub fn cache(mut self, cache: CacheConfig) -> Self {
        self.config.cache = cache;
        self
    }
    
    /// Set performance configuration
    pub fn performance(mut self, performance: PerformanceConfig) -> Self {
        self.config.performance = performance;
        self
    }
    
    /// Add or update a model configuration
    pub fn model(mut self, name: String, config: IndividualModelConfig) -> Self {
        self.config.models.insert(name, config);
        self
    }
    
    /// Build the configuration
    pub fn build(self) -> Result<ModelConfig, Vec<String>> {
        self.config.validate()?;
        Ok(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_default_config() {
        let config = ModelConfig::default();
        assert!(config.validate().is_ok());
        assert!(config.is_model_enabled("intent_recognition"));
        assert!(!config.is_model_enabled("nonexistent_model"));
    }
    
    #[test]
    fn test_config_builder() {
        let config = ModelConfigBuilder::new()
            .inference(InferenceConfig {
                device: "cuda".to_string(),
                ..Default::default()
            })
            .energy(EnergyConfig {
                total_energy_budget: 2.0,
                ..Default::default()
            })
            .build()
            .unwrap();
        
        assert_eq!(config.inference.device, "cuda");
        assert_eq!(config.energy.total_energy_budget, 2.0);
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = ModelConfig::default();
        config.energy.total_energy_budget = -1.0;
        
        let errors = config.validate().unwrap_err();
        assert!(errors.iter().any(|e| e.contains("energy budget")));
    }
    
    #[test]
    fn test_energy_cost() {
        let config = ModelConfig::default();
        assert_eq!(config.get_energy_cost("intent_recognition"), 0.001);
        assert_eq!(config.get_energy_cost("nonexistent"), 0.01); // Default
    }
} 