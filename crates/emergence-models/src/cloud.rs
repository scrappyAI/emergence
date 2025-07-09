//! Cloud Model Integration - Hybrid Local-Cloud Inference
//! 
//! This module provides integration with cloud-based models for fallback scenarios
//! when local models are insufficient or unavailable. Supports Cloudflare Workers AI
//! and OpenRouter for cost-effective cloud inference.

use super::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cloud model configuration
#[derive(Debug, Clone)]
pub struct CloudConfig {
    /// Cloudflare Workers AI configuration
    pub cloudflare: CloudflareConfig,
    /// OpenRouter configuration
    pub openrouter: OpenRouterConfig,
    /// Fallback strategy
    pub fallback_strategy: FallbackStrategy,
}

/// Cloudflare Workers AI configuration
#[derive(Debug, Clone)]
pub struct CloudflareConfig {
    /// API token
    pub api_token: String,
    /// Account ID
    pub account_id: String,
    /// Base URL
    pub base_url: String,
    /// Default model
    pub default_model: String,
}

/// OpenRouter configuration
#[derive(Debug, Clone)]
pub struct OpenRouterConfig {
    /// API key
    pub api_key: String,
    /// Base URL
    pub base_url: String,
    /// Default model
    pub default_model: String,
}

/// Fallback strategy for cloud models
#[derive(Debug, Clone)]
pub enum FallbackStrategy {
    /// Always try local first, then cloud
    LocalFirst,
    /// Use cloud for complex tasks, local for simple
    ComplexityBased,
    /// Use cloud when local models fail
    FailureBased,
    /// Use cloud for specific capabilities
    CapabilityBased(Vec<Capability>),
}

/// Cloud model router
pub struct CloudModelRouter {
    config: CloudConfig,
    client: Client,
    model_capabilities: HashMap<ModelType, Vec<Capability>>,
}

impl CloudModelRouter {
    /// Create a new cloud model router
    pub fn new(config: CloudConfig) -> Self {
        let client = Client::new();
        let model_capabilities = Self::initialize_model_capabilities();
        
        Self {
            config,
            client,
            model_capabilities,
        }
    }
    
    /// Route a task to the appropriate cloud model
    pub async fn route_task(&self, task: &Task) -> Result<String, ModelError> {
        match &self.config.fallback_strategy {
            FallbackStrategy::LocalFirst => {
                // This would be called after local models fail
                self.route_to_best_cloud_model(task)
            }
            FallbackStrategy::ComplexityBased => {
                match task.complexity {
                    TaskComplexity::Simple => {
                        Err(ModelError::ModelNotFound("Use local models for simple tasks".to_string()))
                    }
                    TaskComplexity::Moderate | TaskComplexity::Complex => {
                        self.route_to_best_cloud_model(task)
                    }
                }
            }
            FallbackStrategy::FailureBased => {
                // This would be called after local models fail
                self.route_to_best_cloud_model(task)
            }
            FallbackStrategy::CapabilityBased(capabilities) => {
                if task.required_capabilities.iter().any(|c| capabilities.contains(c)) {
                    self.route_to_best_cloud_model(task)
                } else {
                    Err(ModelError::ModelNotFound("Task doesn't require cloud capabilities".to_string()))
                }
            }
        }
    }
    
    /// Route to Cloudflare Workers AI
    pub async fn route_to_cloudflare(&self, task: &Task) -> Result<String, ModelError> {
        let model = self.select_cloudflare_model(task);
        self.call_cloudflare_api(&model, &task.input).await
    }
    
    /// Route to OpenRouter
    pub async fn route_to_openrouter(&self, task: &Task) -> Result<String, ModelError> {
        let model = self.select_openrouter_model(task);
        self.call_openrouter_api(&model, &task.input).await
    }
    
    /// Select the best cloud model for a task
    fn route_to_best_cloud_model(&self, task: &Task) -> Result<String, ModelError> {
        // Simple heuristic: use Cloudflare for moderate tasks, OpenRouter for complex
        match task.complexity {
            TaskComplexity::Simple => {
                Err(ModelError::ModelNotFound("Use local models for simple tasks".to_string()))
            }
            TaskComplexity::Moderate => {
                // Use Cloudflare for moderate tasks (faster, cheaper)
                Ok("cloudflare".to_string())
            }
            TaskComplexity::Complex => {
                // Use OpenRouter for complex tasks (better models available)
                Ok("openrouter".to_string())
            }
        }
    }
    
    /// Select appropriate Cloudflare model
    fn select_cloudflare_model(&self, task: &Task) -> String {
        // Choose model based on required capabilities
        if task.required_capabilities.contains(&Capability::CodeAnalysis) {
            "codellama-7b-instruct".to_string()
        } else if task.required_capabilities.contains(&Capability::Reasoning) {
            "llama-2-7b-chat".to_string()
        } else {
            "mistral-7b-instruct".to_string()
        }
    }
    
    /// Select appropriate OpenRouter model
    fn select_openrouter_model(&self, task: &Task) -> String {
        // Choose model based on required capabilities
        if task.required_capabilities.contains(&Capability::CodeAnalysis) {
            "meta/codellama-34b-instruct".to_string()
        } else if task.required_capabilities.contains(&Capability::Reasoning) {
            "anthropic/claude-3-sonnet".to_string()
        } else {
            "openai/gpt-4".to_string()
        }
    }
    
    /// Call Cloudflare Workers AI API
    async fn call_cloudflare_api(&self, model: &str, input: &str) -> Result<String, ModelError> {
        let url = format!("{}/v1/chat/completions", self.config.cloudflare.base_url);
        
        let request_body = CloudflareRequest {
            model: model.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are a helpful AI assistant.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: input.to_string(),
                },
            ],
            max_tokens: 1000,
            temperature: 0.7,
        };
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.cloudflare.api_token))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ModelError::CloudApiError(format!("Cloudflare API error: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(ModelError::CloudApiError(format!(
                "Cloudflare API returned status: {}",
                response.status()
            )));
        }
        
        let response_body: CloudflareResponse = response
            .json()
            .await
            .map_err(|e| ModelError::CloudApiError(format!("Failed to parse response: {}", e)))?;
        
        Ok(response_body.choices[0].message.content.clone())
    }
    
    /// Call OpenRouter API
    async fn call_openrouter_api(&self, model: &str, input: &str) -> Result<String, ModelError> {
        let url = format!("{}/v1/chat/completions", self.config.openrouter.base_url);
        
        let request_body = OpenRouterRequest {
            model: model.to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are a helpful AI assistant.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: input.to_string(),
                },
            ],
            max_tokens: 1000,
            temperature: 0.7,
        };
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.openrouter.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ModelError::CloudApiError(format!("OpenRouter API error: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(ModelError::CloudApiError(format!(
                "OpenRouter API returned status: {}",
                response.status()
            )));
        }
        
        let response_body: OpenRouterResponse = response
            .json()
            .await
            .map_err(|e| ModelError::CloudApiError(format!("Failed to parse response: {}", e)))?;
        
        Ok(response_body.choices[0].message.content.clone())
    }
    
    /// Initialize model capabilities mapping
    fn initialize_model_capabilities() -> HashMap<ModelType, Vec<Capability>> {
        let mut capabilities = HashMap::new();
        
        // Cloudflare models
        capabilities.insert(ModelType::CloudflareLlama2, vec![
            Capability::Reasoning,
            Capability::ResponseGeneration,
        ]);
        
        capabilities.insert(ModelType::CloudflareMistral, vec![
            Capability::Reasoning,
            Capability::DataSynthesis,
        ]);
        
        capabilities.insert(ModelType::CloudflareCodeLlama, vec![
            Capability::CodeAnalysis,
            Capability::Reasoning,
        ]);
        
        // OpenRouter models
        capabilities.insert(ModelType::OpenRouterGpt4, vec![
            Capability::Reasoning,
            Capability::DataSynthesis,
            Capability::ResponseGeneration,
        ]);
        
        capabilities.insert(ModelType::OpenRouterClaude, vec![
            Capability::Reasoning,
            Capability::DataSynthesis,
            Capability::ResponseGeneration,
        ]);
        
        capabilities.insert(ModelType::OpenRouterCodeLlama, vec![
            Capability::CodeAnalysis,
            Capability::Reasoning,
        ]);
        
        capabilities
    }
}

/// Cloudflare API request
#[derive(Debug, Serialize)]
struct CloudflareRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f64,
}

/// OpenRouter API request
#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f64,
}

/// Message for chat completion
#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

/// Cloudflare API response
#[derive(Debug, Deserialize)]
struct CloudflareResponse {
    choices: Vec<Choice>,
}

/// OpenRouter API response
#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<Choice>,
}

/// Choice in API response
#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

/// Hybrid model manager that combines local and cloud models
pub struct HybridModelManager {
    local_composer: super::composer::ModelComposer,
    cloud_router: CloudModelRouter,
    fallback_strategy: FallbackStrategy,
}

impl HybridModelManager {
    /// Create a new hybrid model manager
    pub fn new(
        local_composer: super::composer::ModelComposer,
        cloud_router: CloudModelRouter,
        fallback_strategy: FallbackStrategy,
    ) -> Self {
        Self {
            local_composer,
            cloud_router,
            fallback_strategy,
        }
    }
    
    /// Process a task with local-first fallback to cloud
    pub async fn process_with_fallback(&self, task: &Task) -> Result<String, ModelError> {
        // Try local models first
        match self.local_composer.compose_pipeline(task).await {
            Ok(pipeline) => {
                match pipeline.execute(&task.input, &ModelContext::default()).await {
                    Ok(result) => Ok(result),
                    Err(_) => self.fallback_to_cloud(task).await,
                }
            }
            Err(_) => self.fallback_to_cloud(task).await,
        }
    }
    
    /// Fallback to cloud models
    async fn fallback_to_cloud(&self, task: &Task) -> Result<String, ModelError> {
        match &self.fallback_strategy {
            FallbackStrategy::LocalFirst | FallbackStrategy::FailureBased => {
                self.cloud_router.route_task(task).await
            }
            FallbackStrategy::ComplexityBased => {
                match task.complexity {
                    TaskComplexity::Simple => {
                        Err(ModelError::ModelNotFound("No suitable model available".to_string()))
                    }
                    TaskComplexity::Moderate => {
                        self.cloud_router.route_to_cloudflare(task).await
                    }
                    TaskComplexity::Complex => {
                        self.cloud_router.route_to_openrouter(task).await
                    }
                }
            }
            FallbackStrategy::CapabilityBased(_) => {
                self.cloud_router.route_task(task).await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cloud_config_creation() {
        let config = CloudConfig {
            cloudflare: CloudflareConfig {
                api_token: "test_token".to_string(),
                account_id: "test_account".to_string(),
                base_url: "https://api.cloudflare.com".to_string(),
                default_model: "llama-2-7b-chat".to_string(),
            },
            openrouter: OpenRouterConfig {
                api_key: "test_key".to_string(),
                base_url: "https://openrouter.ai/api".to_string(),
                default_model: "openai/gpt-4".to_string(),
            },
            fallback_strategy: FallbackStrategy::LocalFirst,
        };
        
        assert_eq!(config.cloudflare.api_token, "test_token");
        assert_eq!(config.openrouter.api_key, "test_key");
    }
    
    #[test]
    fn test_model_capabilities() {
        let capabilities = CloudModelRouter::initialize_model_capabilities();
        assert!(capabilities.contains_key(&ModelType::CloudflareLlama2));
        assert!(capabilities.contains_key(&ModelType::OpenRouterGpt4));
        
        let llama_capabilities = &capabilities[&ModelType::CloudflareLlama2];
        assert!(llama_capabilities.contains(&Capability::Reasoning));
        assert!(llama_capabilities.contains(&Capability::ResponseGeneration));
    }
} 