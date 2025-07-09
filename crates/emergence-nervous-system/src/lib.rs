//! **emergence-nervous-system** – Event-driven nervous system for EMERGENCE living agents.
//!
//! This crate provides the neural communication substrate that enables living agents
//! to sense, process, and respond to events in their environment. The nervous system
//! implements event-driven patterns with physics-constrained signal propagation,
//! neural pathway routing, and emergent behavior coordination.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_stream::wrappers::BroadcastStream;
use tracing::{debug, error, info};
use uuid::Uuid;

use emergence_physics::{EntityId, PhysicsEngine, PhysicsOperation};

/// Core nervous system that coordinates event-driven communication
pub struct NervousSystem {
    /// Physics engine for constraint enforcement
    physics_engine: Arc<PhysicsEngine>,
    /// Event broadcast channels for different signal types
    signal_channels: Arc<RwLock<HashMap<SignalType, broadcast::Sender<NeuralSignal>>>>,
    /// Neural pathways for routing signals between entities
    neural_pathways: Arc<RwLock<HashMap<EntityId, HashSet<EntityId>>>>,
    /// Active signal processors for each entity
    signal_processors: Arc<RwLock<HashMap<EntityId, SignalProcessor>>>,
    /// System configuration
    config: NervousSystemConfig,
    /// System start time for relative timing
    genesis_time: Instant,
    /// Unique system instance identifier
    instance_id: Uuid,
}

/// Configuration for the nervous system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NervousSystemConfig {
    /// Maximum signal propagation distance
    pub max_propagation_distance: u32,
    /// Signal decay rate per hop
    pub signal_decay_rate: f64,
    /// Maximum concurrent signals per entity
    pub max_concurrent_signals: usize,
    /// Signal processing timeout
    pub signal_timeout: Duration,
    /// Enable physics constraint enforcement
    pub enforce_physics: bool,
    /// Neural pathway formation threshold
    pub pathway_formation_threshold: f64,
}

/// Types of neural signals that can be transmitted
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalType {
    /// Sensory input from environment
    Sensory,
    /// Cognitive processing signals
    Cognitive,
    /// Motor/action output signals
    Motor,
    /// Emotional/affective signals
    Emotional,
    /// Memory access signals
    Memory,
    /// Coordination signals between entities
    Coordination,
    /// Emergency/high-priority signals
    Emergency,
}

/// Neural signal with physics-constrained properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralSignal {
    /// Unique signal identifier
    pub signal_id: Uuid,
    /// Signal type
    pub signal_type: SignalType,
    /// Source entity
    pub source: EntityId,
    /// Target entity (None for broadcast)
    pub target: Option<EntityId>,
    /// Signal payload
    pub payload: SignalPayload,
    /// Signal strength (0.0 to 1.0)
    pub strength: f64,
    /// Signal propagation distance
    pub propagation_distance: u32,
    /// Signal timestamp
    pub timestamp: DateTime<Utc>,
    /// Energy cost for signal transmission
    pub energy_cost: f64,
    /// Causal dependencies
    pub causal_dependencies: Vec<Uuid>,
}

/// Payload carried by neural signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalPayload {
    /// Text-based message
    Message(String),
    /// Structured data
    Data(serde_yaml::Value),
    /// Binary data
    Binary(Vec<u8>),
    /// Command/instruction
    Command(String),
    /// Query/request
    Query(String),
    /// Response/result
    Response(serde_yaml::Value),
    /// Event notification
    Event(String),
    /// State update
    StateUpdate(serde_yaml::Value),
}

/// Signal processor for individual entities
pub struct SignalProcessor {
    /// Entity identifier
    pub entity_id: EntityId,
    /// Signal processing capabilities
    pub capabilities: HashSet<SignalType>,
    /// Signal processing function
    pub processor: Box<dyn SignalProcessorFn + Send + Sync>,
    /// Current signal queue
    pub signal_queue: mpsc::Sender<NeuralSignal>,
    /// Processing statistics
    pub stats: ProcessingStats,
}

/// Signal processing function signature
pub trait SignalProcessorFn {
    fn process_signal(&self, signal: &NeuralSignal) -> Result<Option<NeuralSignal>>;
}

/// Signal processing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStats {
    /// Total signals processed
    pub signals_processed: u64,
    /// Average processing time
    pub avg_processing_time: Duration,
    /// Error count
    pub error_count: u64,
    /// Last processing timestamp
    pub last_processed: Option<DateTime<Utc>>,
}

/// Nervous system operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NervousSystemResult {
    /// Operation success status
    pub success: bool,
    /// Result message
    pub message: String,
    /// Operation duration
    pub duration: Duration,
    /// Energy consumed
    pub energy_consumed: f64,
    /// Signals generated
    pub signals_generated: u32,
}

/// Nervous system errors
#[derive(Debug, thiserror::Error)]
pub enum NervousSystemError {
    /// Entity not found
    #[error("Entity {entity} not found in nervous system")]
    EntityNotFound { entity: EntityId },
    
    /// Signal processing error
    #[error("Signal processing failed: {reason}")]
    SignalProcessingError { reason: String },
    
    /// Physics constraint violation
    #[error("Physics constraint violated: {reason}")]
    PhysicsViolation { reason: String },
    
    /// Signal timeout
    #[error("Signal processing timeout after {timeout:?}")]
    SignalTimeout { timeout: Duration },
    
    /// Invalid signal configuration
    #[error("Invalid signal configuration: {reason}")]
    InvalidConfiguration { reason: String },
}

impl Default for NervousSystemConfig {
    fn default() -> Self {
        Self {
            max_propagation_distance: 10,
            signal_decay_rate: 0.1,
            max_concurrent_signals: 100,
            signal_timeout: Duration::from_secs(30),
            enforce_physics: true,
            pathway_formation_threshold: 0.5,
        }
    }
}

impl NervousSystem {
    /// Create a new nervous system with physics engine integration
    pub async fn new(physics_engine: Arc<PhysicsEngine>) -> Result<Self> {
        let config = NervousSystemConfig::default();
        let instance_id = Uuid::new_v4();
        let genesis_time = Instant::now();
        
        // Initialize signal channels for each signal type
        let mut signal_channels = HashMap::new();
        for signal_type in [
            SignalType::Sensory,
            SignalType::Cognitive,
            SignalType::Motor,
            SignalType::Emotional,
            SignalType::Memory,
            SignalType::Coordination,
            SignalType::Emergency,
        ] {
            let (tx, _) = broadcast::channel(config.max_concurrent_signals);
            signal_channels.insert(signal_type, tx);
        }
        
        info!("Initializing nervous system with instance ID: {}", instance_id);
        
        Ok(Self {
            physics_engine,
            signal_channels: Arc::new(RwLock::new(signal_channels)),
            neural_pathways: Arc::new(RwLock::new(HashMap::new())),
            signal_processors: Arc::new(RwLock::new(HashMap::new())),
            config,
            genesis_time,
            instance_id,
        })
    }
    
    /// Register an entity with the nervous system
    pub async fn register_entity(
        &self,
        entity_id: EntityId,
        capabilities: HashSet<SignalType>,
        processor: Box<dyn SignalProcessorFn + Send + Sync>,
    ) -> Result<()> {
        info!("Registering entity {} with capabilities: {:?}", entity_id, capabilities);
        
        // Create signal queue for the entity
        let (tx, rx) = mpsc::channel(self.config.max_concurrent_signals);
        
        let signal_processor = SignalProcessor {
            entity_id,
            capabilities,
            processor,
            signal_queue: tx,
            stats: ProcessingStats {
                signals_processed: 0,
                avg_processing_time: Duration::from_millis(0),
                error_count: 0,
                last_processed: None,
            },
        };
        
        // Store the processor
        {
            let mut processors = self.signal_processors.write().await;
            processors.insert(entity_id, signal_processor);
        }
        
        // Initialize neural pathways
        {
            let mut pathways = self.neural_pathways.write().await;
            pathways.insert(entity_id, HashSet::new());
        }
        
        // Start signal processing loop
        let physics_engine = self.physics_engine.clone();
        let signal_channels = self.signal_channels.clone();
        let config = self.config.clone();
        
        tokio::spawn(async move {
            Self::process_entity_signals(
                entity_id,
                rx,
                physics_engine,
                signal_channels,
                config,
            ).await;
        });
        
        Ok(())
    }
    
    /// Transmit a neural signal through the nervous system
    pub async fn transmit_signal(&self, signal: NeuralSignal) -> Result<NervousSystemResult> {
        let start_time = Instant::now();
        
        debug!("Transmitting signal {} from {} to {:?}", 
               signal.signal_id, signal.source, signal.target);
        
        // Validate signal with physics engine
        if self.config.enforce_physics {
            self.validate_signal_physics(&signal).await?;
        }
        
        // Calculate energy cost
        let energy_cost = self.calculate_signal_energy_cost(&signal);
        
        // Transmit signal through appropriate channel
        let signal_result = self.route_signal(signal).await?;
        
        let duration = start_time.elapsed();
        
        Ok(NervousSystemResult {
            success: true,
            message: format!("Signal transmitted successfully"),
            duration,
            energy_consumed: energy_cost,
            signals_generated: signal_result.signals_generated,
        })
    }
    
    /// Create a signal stream for an entity
    pub async fn create_signal_stream(
        &self,
        entity_id: EntityId,
        signal_types: Vec<SignalType>,
    ) -> Result<impl Stream<Item = NeuralSignal>> {
        let channels = self.signal_channels.read().await;
        let mut streams = Vec::new();
        
        for signal_type in signal_types {
            if let Some(channel) = channels.get(&signal_type) {
                let stream = BroadcastStream::new(channel.subscribe());
                streams.push(stream);
            }
        }
        
        // Combine streams and filter for entity
        let combined_stream = futures::stream::select_all(streams)
            .filter_map(move |signal_result| {
                let entity_id = entity_id;
                async move {
                    match signal_result {
                        Ok(signal) => {
                            let entity_matches = signal.target.is_none() || signal.target == Some(entity_id);
                            if entity_matches {
                                Some(signal)
                            } else {
                                None
                            }
                        }
                        Err(_) => None,
                    }
                }
            });
        
        Ok(combined_stream)
    }
    
    /// Form neural pathway between entities
    pub async fn form_pathway(&self, from: EntityId, to: EntityId) -> Result<()> {
        info!("Forming neural pathway from {} to {}", from, to);
        
        let mut pathways = self.neural_pathways.write().await;
        
        // Add bidirectional pathway
        pathways.entry(from).or_insert_with(HashSet::new).insert(to);
        pathways.entry(to).or_insert_with(HashSet::new).insert(from);
        
        Ok(())
    }
    
    /// Get nervous system statistics
    pub async fn get_statistics(&self) -> Result<NervousSystemStats> {
        let processors = self.signal_processors.read().await;
        let pathways = self.neural_pathways.read().await;
        
        let mut total_signals = 0;
        let mut total_errors = 0;
        let mut avg_processing_time = Duration::from_millis(0);
        
        for processor in processors.values() {
            total_signals += processor.stats.signals_processed;
            total_errors += processor.stats.error_count;
            avg_processing_time += processor.stats.avg_processing_time;
        }
        
        let avg_time = if processors.len() > 0 {
            avg_processing_time / processors.len() as u32
        } else {
            Duration::from_millis(0)
        };
        
        Ok(NervousSystemStats {
            instance_id: self.instance_id,
            uptime: self.genesis_time.elapsed(),
            registered_entities: processors.len(),
            total_pathways: pathways.values().map(|p| p.len()).sum(),
            total_signals_processed: total_signals,
            total_errors: total_errors,
            avg_processing_time: avg_time,
        })
    }
    
    /// Validate signal with physics constraints
    async fn validate_signal_physics(&self, signal: &NeuralSignal) -> Result<()> {
        // Check energy constraints
        let energy_operation = PhysicsOperation::TransferEnergy {
            from: signal.source,
            to: signal.target.unwrap_or(signal.source), // Self-transfer for broadcast
            amount: ordered_float::OrderedFloat(signal.energy_cost),
        };
        
        let physics_result = self.physics_engine.execute_operation(energy_operation).await
            .context("Failed to validate signal physics")?;
        
        if !physics_result.success {
            return Err(NervousSystemError::PhysicsViolation {
                reason: physics_result.message,
            }.into());
        }
        
        Ok(())
    }
    
    /// Calculate energy cost for signal transmission
    fn calculate_signal_energy_cost(&self, signal: &NeuralSignal) -> f64 {
        let base_cost = 0.001; // Base energy cost
        let distance_multiplier = signal.propagation_distance as f64 * self.config.signal_decay_rate;
        let strength_multiplier = signal.strength;
        
        base_cost * (1.0 + distance_multiplier) * strength_multiplier
    }
    
    /// Route signal to appropriate channels
    async fn route_signal(&self, signal: NeuralSignal) -> Result<NervousSystemResult> {
        let channels = self.signal_channels.read().await;
        
        if let Some(channel) = channels.get(&signal.signal_type) {
            let _ = channel.send(signal.clone());
        }
        
        // Also send to target entity's processor if specified
        if let Some(target) = signal.target {
            let processors = self.signal_processors.read().await;
            if let Some(processor) = processors.get(&target) {
                let _ = processor.signal_queue.send(signal).await;
            }
        }
        
        Ok(NervousSystemResult {
            success: true,
            message: "Signal routed successfully".to_string(),
            duration: Duration::from_millis(1),
            energy_consumed: 0.0,
            signals_generated: 1,
        })
    }
    
    /// Process signals for a specific entity
    async fn process_entity_signals(
        entity_id: EntityId,
        mut rx: mpsc::Receiver<NeuralSignal>,
        physics_engine: Arc<PhysicsEngine>,
        signal_channels: Arc<RwLock<HashMap<SignalType, broadcast::Sender<NeuralSignal>>>>,
        config: NervousSystemConfig,
    ) {
        info!("Starting signal processing for entity {}", entity_id);
        
        while let Some(signal) = rx.recv().await {
            let start_time = Instant::now();
            
            debug!("Processing signal {} for entity {}", signal.signal_id, entity_id);
            
            // Process signal with timeout
            let processing_result = tokio::time::timeout(
                config.signal_timeout,
                Self::process_single_signal(signal, &physics_engine, &signal_channels)
            ).await;
            
            match processing_result {
                Ok(Ok(response_signal)) => {
                    if let Some(response) = response_signal {
                        // Transmit response signal
                        let _ = Self::transmit_response_signal(response, &signal_channels).await;
                    }
                }
                Ok(Err(e)) => {
                    error!("Signal processing error for entity {}: {}", entity_id, e);
                }
                Err(_) => {
                    error!("Signal processing timeout for entity {}", entity_id);
                }
            }
            
            let processing_time = start_time.elapsed();
            debug!("Signal processed in {:?} for entity {}", processing_time, entity_id);
        }
        
        info!("Signal processing stopped for entity {}", entity_id);
    }
    
    /// Process a single signal
    async fn process_single_signal(
        signal: NeuralSignal,
        _physics_engine: &Arc<PhysicsEngine>,
        _signal_channels: &Arc<RwLock<HashMap<SignalType, broadcast::Sender<NeuralSignal>>>>,
    ) -> Result<Option<NeuralSignal>> {
        // For now, just acknowledge the signal
        // In a real implementation, this would invoke the entity's processor
        debug!("Processing signal: {:?}", signal.signal_type);
        
        // Return a simple acknowledgment signal
        let response = NeuralSignal {
            signal_id: Uuid::new_v4(),
            signal_type: SignalType::Coordination,
            source: signal.target.unwrap_or(signal.source),
            target: Some(signal.source),
            payload: SignalPayload::Message("Signal received".to_string()),
            strength: signal.strength * 0.8, // Decay response strength
            propagation_distance: 0,
            timestamp: Utc::now(),
            energy_cost: 0.001,
            causal_dependencies: vec![signal.signal_id],
        };
        
        Ok(Some(response))
    }
    
    /// Transmit response signal
    async fn transmit_response_signal(
        response: NeuralSignal,
        signal_channels: &Arc<RwLock<HashMap<SignalType, broadcast::Sender<NeuralSignal>>>>,
    ) -> Result<()> {
        let channels = signal_channels.read().await;
        
        if let Some(channel) = channels.get(&response.signal_type) {
            let _ = channel.send(response);
        }
        
        Ok(())
    }
}

/// Nervous system statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NervousSystemStats {
    /// System instance identifier
    pub instance_id: Uuid,
    /// System uptime
    pub uptime: Duration,
    /// Number of registered entities
    pub registered_entities: usize,
    /// Total neural pathways
    pub total_pathways: usize,
    /// Total signals processed
    pub total_signals_processed: u64,
    /// Total processing errors
    pub total_errors: u64,
    /// Average processing time
    pub avg_processing_time: Duration,
}

impl NeuralSignal {
    /// Create a new neural signal
    pub fn new(
        signal_type: SignalType,
        source: EntityId,
        target: Option<EntityId>,
        payload: SignalPayload,
        strength: f64,
    ) -> Self {
        Self {
            signal_id: Uuid::new_v4(),
            signal_type,
            source,
            target,
            payload,
            strength: strength.clamp(0.0, 1.0),
            propagation_distance: 0,
            timestamp: Utc::now(),
            energy_cost: 0.001,
            causal_dependencies: Vec::new(),
        }
    }
    
    /// Create a broadcast signal
    pub fn broadcast(
        signal_type: SignalType,
        source: EntityId,
        payload: SignalPayload,
        strength: f64,
    ) -> Self {
        Self::new(signal_type, source, None, payload, strength)
    }
    
    /// Add causal dependency
    pub fn with_causal_dependency(mut self, dependency: Uuid) -> Self {
        self.causal_dependencies.push(dependency);
        self
    }
    
    /// Set energy cost
    pub fn with_energy_cost(mut self, cost: f64) -> Self {
        self.energy_cost = cost;
        self
    }
}

impl Default for NervousSystem {
    fn default() -> Self {
        panic!("NervousSystem cannot be created without a physics engine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use emergence_physics::PhysicsEngine;
    
    struct TestProcessor;
    
    impl SignalProcessorFn for TestProcessor {
        fn process_signal(&self, signal: &NeuralSignal) -> Result<Option<NeuralSignal>> {
            Ok(Some(NeuralSignal::new(
                SignalType::Coordination,
                signal.target.unwrap_or(signal.source),
                Some(signal.source),
                SignalPayload::Message("Test response".to_string()),
                0.5,
            )))
        }
    }
    
    #[tokio::test]
    async fn test_nervous_system_creation() {
        let physics_engine = Arc::new(PhysicsEngine::new().await.unwrap());
        let nervous_system = NervousSystem::new(physics_engine).await.unwrap();
        
        let stats = nervous_system.get_statistics().await.unwrap();
        assert_eq!(stats.registered_entities, 0);
        assert_eq!(stats.instance_id, nervous_system.instance_id);
    }
    
    #[tokio::test]
    async fn test_entity_registration() {
        let physics_engine = Arc::new(PhysicsEngine::new().await.unwrap());
        let nervous_system = NervousSystem::new(physics_engine).await.unwrap();
        
        let entity_id = EntityId::new();
        let capabilities = HashSet::from([SignalType::Sensory, SignalType::Cognitive]);
        let processor = Box::new(TestProcessor);
        
        nervous_system.register_entity(entity_id, capabilities, processor).await.unwrap();
        
        let stats = nervous_system.get_statistics().await.unwrap();
        assert_eq!(stats.registered_entities, 1);
    }
    
    #[tokio::test]
    async fn test_signal_transmission() {
        let physics_engine = Arc::new(PhysicsEngine::new().await.unwrap());
        let nervous_system = NervousSystem::new(physics_engine.clone()).await.unwrap();
        
        let entity_id = EntityId::new();
        // Allocate energy to the entity so it can transmit signals
        physics_engine
            .allocate_energy_to_entity(entity_id, ordered_float::OrderedFloat(0.1))
            .await
            .unwrap();
        
        let signal = NeuralSignal::new(
            SignalType::Sensory,
            entity_id,
            None,
            SignalPayload::Message("Test signal".to_string()),
            0.8,
        );
        
        let result = nervous_system.transmit_signal(signal).await.unwrap();
        assert!(result.success);
        assert_eq!(result.signals_generated, 1);
    }
}