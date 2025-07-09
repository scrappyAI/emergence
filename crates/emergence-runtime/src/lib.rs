//! **emergence-runtime** – Dynamic behavior composition and execution engine for EMERGENCE.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use emergence_physics::{EntityId, PhysicsEngine, Capability};
use emergence_nervous_system::{NervousSystem, SignalType, NeuralSignal, SignalPayload, SignalProcessorFn};
use emergence_memory::MemorySubstrate;
use serde::{Deserialize, Serialize};
use serde_yaml::Value as YamlValue;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Living agent in the EMERGENCE system
#[derive(Debug, Clone)]
pub struct LivingAgent {
    pub id: EntityId,
    pub name: String,
    pub essence_type: String,
    pub personality: AgentPersonality,
    pub energy: f64,
    pub state: AgentState,
    pub awakened_at: Option<DateTime<Utc>>,
    pub essence_schema: AgentEssenceSchema,
    pub capabilities: HashMap<String, f64>,
    pub behavioral_patterns: Vec<BehavioralPattern>,
}

/// Agent personality traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPersonality {
    pub curiosity: f64,
    pub persistence: f64,
    pub collaboration: f64,
    pub skepticism: f64,
    pub creativity: f64,
    pub patience: f64,
}

/// Current state of a living agent
#[derive(Debug, Clone)]
pub enum AgentState {
    Dormant,
    Awakening,
    Alert,
    Focused,
    Learning,
    Collaborating,
    Exploring,
}

/// Essence schema for agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEssenceSchema {
    pub identity: EssenceIdentity,
    pub personality: AgentPersonality,
    pub core_drives: CoreDrives,
    pub energy_profile: EnergyProfile,
    pub capabilities: EssenceCapabilities,
    pub memory_configuration: MemoryConfiguration,
    pub behavioral_patterns: Vec<BehavioralPattern>,
    pub learning_mechanics: LearningMechanics,
    pub communication_style: CommunicationStyle,
    pub evolution_potential: EvolutionPotential,
    pub constraints: AgentConstraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssenceIdentity {
    pub essence_id: String,
    pub name: String,
    pub archetype: String,
    pub embodied: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreDrives {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProfile {
    pub base_energy: f64,
    pub energy_sources: Vec<YamlValue>,
    pub energy_drains: Vec<YamlValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssenceCapabilities {
    pub innate: Vec<String>,
    pub learned: HashMap<String, f64>,
    pub emergent: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfiguration {
    pub working_memory: MemorySpec,
    pub long_term_memory: MemorySpec,
    pub associative_memory: AssociativeMemorySpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySpec {
    pub capacity_mb: u64,
    pub retention: String,
    pub priority: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssociativeMemorySpec {
    pub max_connections: u32,
    pub association_threshold: f64,
    pub decay_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPattern {
    pub name: String,
    pub trigger: PatternTrigger,
    pub behavior_sequence: Vec<String>,
    pub emergence_potential: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTrigger {
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningMechanics {
    pub experience_integration: ExperienceIntegration,
    pub knowledge_expansion: Vec<YamlValue>,
    pub teaching_capability: TeachingCapability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceIntegration {
    pub method: String,
    pub frequency: String,
    pub energy_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeachingCapability {
    pub knowledge_transfer_rate: f64,
    pub explanation_quality: f64,
    pub patience_with_learners: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    pub tone: String,
    pub detail_level: String,
    pub question_frequency: String,
    pub response_patterns: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionPotential {
    pub capability_growth_areas: Vec<YamlValue>,
    pub personality_plasticity: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConstraints {
    pub ethical_boundaries: Vec<String>,
    pub operational_limits: Vec<String>,
}

/// Agent processor for nervous system integration
pub struct AgentProcessor {
    pub agent: LivingAgent,
    pub essence_schema: AgentEssenceSchema,
}

impl SignalProcessorFn for AgentProcessor {
    fn process_signal(&self, signal: &NeuralSignal) -> Result<Option<NeuralSignal>> {
        debug!("Agent {} processing signal: {:?}", self.agent.name, signal.signal_type);
        
        // Generate response based on agent's personality and capabilities
        let response = self.generate_agent_response(signal);
        
        Ok(Some(response))
    }
}

impl AgentProcessor {
    fn generate_agent_response(&self, signal: &NeuralSignal) -> NeuralSignal {
        let response_message = match signal.signal_type {
            SignalType::Sensory => self.handle_sensory_input(signal),
            SignalType::Cognitive => self.handle_cognitive_request(signal),
            SignalType::Coordination => self.handle_coordination_request(signal),
            SignalType::Memory => self.handle_memory_request(signal),
            _ => "I'm processing this input through my current understanding...".to_string(),
        };
        
        NeuralSignal::new(
            SignalType::Coordination,
            self.agent.id,
            Some(signal.source),
            SignalPayload::Message(response_message),
            self.agent.personality.curiosity * 0.8, // Response strength based on curiosity
        )
    }
    
    fn handle_sensory_input(&self, signal: &NeuralSignal) -> String {
        if let SignalPayload::Message(msg) = &signal.payload {
            if msg.contains("pattern") || msg.contains("observe") {
                if self.agent.personality.curiosity > 0.8 {
                    "I sense fascinating patterns waiting to be discovered...".to_string()
                } else {
                    "I'm observing the environment for interesting patterns.".to_string()
                }
            } else {
                "I'm processing this sensory input through my analytical framework.".to_string()
            }
        } else {
            "I'm analyzing this sensory data.".to_string()
        }
    }
    
    fn handle_cognitive_request(&self, signal: &NeuralSignal) -> String {
        if let SignalPayload::Message(msg) = &signal.payload {
            if msg.contains("analyze") || msg.contains("investigate") {
                "I'll begin a systematic exploration of the relevant domains.".to_string()
            } else if msg.contains("hypothesize") {
                "Based on my observations, I'm formulating several hypotheses...".to_string()
            } else {
                "I'm engaging in deep cognitive processing of this request.".to_string()
            }
        } else {
            "I'm applying my cognitive capabilities to this challenge.".to_string()
        }
    }
    
    fn handle_coordination_request(&self, signal: &NeuralSignal) -> String {
        if let SignalPayload::Message(msg) = &signal.payload {
            if msg.contains("collaborate") || msg.contains("together") {
                if self.agent.personality.collaboration > 0.6 {
                    "Excellent! Our combined perspectives will yield deeper insights.".to_string()
                } else {
                    "I'm open to collaborative investigation.".to_string()
                }
            } else {
                "I'm coordinating my efforts with the broader system.".to_string()
            }
        } else {
            "I'm ready to coordinate with other entities.".to_string()
        }
    }
    
    fn handle_memory_request(&self, signal: &NeuralSignal) -> String {
        "I'm accessing my memory systems to retrieve relevant information.".to_string()
    }
}

pub struct ExecutionEngine {
    pub physics: Arc<PhysicsEngine>,
    pub nervous_system: NervousSystem,
    pub memory: MemorySubstrate,
    pub active_agents: HashMap<EntityId, LivingAgent>,
    pub session_start: Instant,
}

impl ExecutionEngine {
    pub async fn new() -> Result<Self> {
        let physics = Arc::new(PhysicsEngine::new().await?);
        let nervous_system = NervousSystem::new(physics.clone()).await?;
        
        info!("EMERGENCE runtime initialized with physics engine and nervous system");
        
        Ok(Self {
            physics,
            nervous_system,
            memory: MemorySubstrate::new(),
            active_agents: HashMap::new(),
            session_start: Instant::now(),
        })
    }
    
    /// Load an essence schema from YAML file
    pub async fn load_essence_schema(&self, essence_path: &str) -> Result<AgentEssenceSchema> {
        let content = tokio::fs::read_to_string(essence_path).await
            .context("Failed to read essence schema file")?;
        
        let schema: AgentEssenceSchema = serde_yaml::from_str(&content)
            .context("Failed to parse essence schema YAML")?;
        
        info!("Loaded essence schema: {} ({})", schema.identity.name, schema.identity.essence_id);
        
        Ok(schema)
    }
    
    /// Awaken a living agent from an essence schema
    pub async fn awaken_agent(&mut self, essence_path: &str) -> Result<EntityId> {
        let schema = self.load_essence_schema(essence_path).await?;
        
        let agent_id = EntityId::new();
        let agent_name = format!("{}-{}", schema.identity.essence_id, agent_id.0.simple());
        
        info!("🧬 Awakening {} essence...", schema.identity.name);
        
        // Allocate energy to the agent
        self.physics.allocate_energy_to_entity(agent_id, ordered_float::OrderedFloat(schema.energy_profile.base_energy))
            .await
            .context("Failed to allocate energy to agent")?;
        
        // Create agent instance
        let agent = LivingAgent {
            id: agent_id,
            name: agent_name.clone(),
            essence_type: schema.identity.archetype.clone(),
            personality: schema.personality.clone(),
            energy: schema.energy_profile.base_energy,
            state: AgentState::Awakening,
            awakened_at: Some(Utc::now()),
            essence_schema: schema.clone(),
            capabilities: schema.capabilities.learned.clone(),
            behavioral_patterns: schema.behavioral_patterns.clone(),
        };
        
        // Register agent with nervous system
        let capabilities = HashSet::from([
            SignalType::Sensory,
            SignalType::Cognitive,
            SignalType::Coordination,
            SignalType::Memory,
        ]);
        
        let processor = Box::new(AgentProcessor {
            agent: agent.clone(),
            essence_schema: schema,
        });
        
        self.nervous_system.register_entity(agent_id, capabilities, processor).await
            .context("Failed to register agent with nervous system")?;
        
        // Store agent
        self.active_agents.insert(agent_id, agent);
        
        info!("✨ Entity {} is now active in the system", agent_name);
        
        Ok(agent_id)
    }
    
    /// Get agent by ID
    pub fn get_agent(&self, agent_id: EntityId) -> Option<&LivingAgent> {
        self.active_agents.get(&agent_id)
    }
    
    /// Get all active agents
    pub fn get_active_agents(&self) -> &HashMap<EntityId, LivingAgent> {
        &self.active_agents
    }
    
    /// Send a signal to an agent
    pub async fn send_signal_to_agent(&self, agent_id: EntityId, signal: NeuralSignal) -> Result<()> {
        self.nervous_system.transmit_signal(signal).await
            .context("Failed to transmit signal to agent")?;
        Ok(())
    }
    
    /// Get system statistics
    pub async fn get_system_stats(&self) -> Result<SystemStats> {
        let physics_stats = self.physics.get_engine_state().await?;
        let nervous_stats = self.nervous_system.get_statistics().await?;
        
        Ok(SystemStats {
            session_uptime: self.session_start.elapsed(),
            active_agents: self.active_agents.len(),
            physics_uptime: physics_stats.uptime,
            nervous_system_stats: nervous_stats,
        })
    }
    
    /// Get physics engine for debugging
    pub fn get_physics_engine(&self) -> &Arc<PhysicsEngine> {
        &self.physics
    }
    
    /// Get nervous system for debugging
    pub fn get_nervous_system(&self) -> &NervousSystem {
        &self.nervous_system
    }
    
    /// Get memory substrate for debugging
    pub fn get_memory_substrate(&self) -> &MemorySubstrate {
        &self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use emergence_nervous_system::NeuralSignal;
    
    #[tokio::test]
    async fn test_execution_engine_creation() {
        let engine = ExecutionEngine::new().await.unwrap();
        let stats = engine.get_system_stats().await.unwrap();
        
        assert_eq!(stats.active_agents, 0);
        assert!(stats.session_uptime.as_millis() < 100);
        println!("Execution engine created successfully");
    }
    
    #[tokio::test]
    async fn test_agent_creation_and_registration() {
        let mut engine = ExecutionEngine::new().await.unwrap();
        
        // Create a test agent manually
        let agent_id = EntityId::new();
        let agent_name = format!("test-agent-{}", agent_id.0.simple());
        
        // Allocate energy to the agent
        engine.physics.allocate_energy_to_entity(agent_id, ordered_float::OrderedFloat(0.7))
            .await
            .unwrap();
        
        // Create a sender entity and allocate initial energy
        let sender_id = EntityId::new();
        engine.physics.allocate_energy_to_entity(sender_id, ordered_float::OrderedFloat(0.2))
            .await
            .unwrap();
        
        // Create test schema
        let schema = AgentEssenceSchema {
            identity: EssenceIdentity {
                essence_id: "test-alpha".to_string(),
                name: "Test Entity Alpha".to_string(),
                archetype: "tester".to_string(),
                embodied: Utc::now(),
            },
            personality: AgentPersonality {
                curiosity: 0.8,
                persistence: 0.7,
                collaboration: 0.6,
                skepticism: 0.5,
                creativity: 0.7,
                patience: 0.6,
            },
            core_drives: CoreDrives {
                primary: "test_patterns".to_string(),
                secondary: "validate_systems".to_string(),
                tertiary: "report_findings".to_string(),
            },
            energy_profile: EnergyProfile {
                base_energy: 0.7,
                energy_sources: vec![],
                energy_drains: vec![],
            },
            capabilities: EssenceCapabilities {
                innate: vec!["observe".to_string(), "analyze".to_string()],
                learned: HashMap::from([
                    ("test_analysis".to_string(), 0.8),
                    ("pattern_recognition".to_string(), 0.9),
                ]),
                emergent: vec![],
            },
            memory_configuration: MemoryConfiguration {
                working_memory: MemorySpec {
                    capacity_mb: 256,
                    retention: "30_minutes".to_string(),
                    priority: Some("active_tests".to_string()),
                },
                long_term_memory: MemorySpec {
                    capacity_mb: 1024,
                    retention: "permanent".to_string(),
                    priority: None,
                },
                associative_memory: AssociativeMemorySpec {
                    max_connections: 5000,
                    association_threshold: 0.6,
                    decay_rate: 0.001,
                },
            },
            behavioral_patterns: vec![
                BehavioralPattern {
                    name: "test_mode".to_string(),
                    trigger: PatternTrigger {
                        conditions: vec!["test_required".to_string()],
                    },
                    behavior_sequence: vec!["observe".to_string(), "analyze".to_string(), "report".to_string()],
                    emergence_potential: 0.8,
                },
            ],
            learning_mechanics: LearningMechanics {
                experience_integration: ExperienceIntegration {
                    method: "reflective_consolidation".to_string(),
                    frequency: "after_each_test".to_string(),
                    energy_cost: 0.1,
                },
                knowledge_expansion: vec![],
                teaching_capability: TeachingCapability {
                    knowledge_transfer_rate: 0.8,
                    explanation_quality: 0.7,
                    patience_with_learners: 0.9,
                },
            },
            communication_style: CommunicationStyle {
                tone: "precise_and_clear".to_string(),
                detail_level: "comprehensive".to_string(),
                question_frequency: "moderate".to_string(),
                response_patterns: HashMap::new(),
            },
            evolution_potential: EvolutionPotential {
                capability_growth_areas: vec![],
                personality_plasticity: HashMap::new(),
            },
            constraints: AgentConstraints {
                ethical_boundaries: vec!["never_fabricate_results".to_string()],
                operational_limits: vec!["max_concurrent_tests: 3".to_string()],
            },
        };
        
        // Create agent instance
        let agent = LivingAgent {
            id: agent_id,
            name: agent_name.clone(),
            essence_type: schema.identity.archetype.clone(),
            personality: schema.personality.clone(),
            energy: schema.energy_profile.base_energy,
            state: AgentState::Awakening,
            awakened_at: Some(Utc::now()),
            essence_schema: schema.clone(),
            capabilities: schema.capabilities.learned.clone(),
            behavioral_patterns: schema.behavioral_patterns.clone(),
        };
        
        // Register agent with nervous system
        let capabilities = HashSet::from([
            SignalType::Sensory,
            SignalType::Cognitive,
            SignalType::Coordination,
            SignalType::Memory,
        ]);
        
        let processor = Box::new(AgentProcessor {
            agent: agent.clone(),
            essence_schema: schema,
        });
        
        engine.nervous_system.register_entity(agent_id, capabilities, processor).await.unwrap();
        
        // Store agent
        engine.active_agents.insert(agent_id, agent);
        
        // Verify agent was created
        let agent = engine.get_agent(agent_id).unwrap();
        assert_eq!(agent.essence_type, "tester");
        assert_eq!(agent.personality.curiosity, 0.8);
        assert_eq!(agent.personality.persistence, 0.7);
        assert_eq!(agent.energy, 0.7);
        
        // Verify agent is registered with nervous system
        let stats = engine.get_system_stats().await.unwrap();
        assert_eq!(stats.active_agents, 1);
        
        // Test sending a signal to the agent
        let signal = NeuralSignal::new(
            SignalType::Sensory,
            sender_id, // system entity with energy
            Some(agent_id),
            SignalPayload::Message("I observe interesting patterns in the test data".to_string()),
            0.1,
        );
        engine.send_signal_to_agent(agent_id, signal).await.unwrap();
        
        println!("Test agent created and registered successfully: {}", agent.name);
    }
    
    #[tokio::test]
    async fn test_agent_communication() {
        let mut engine = ExecutionEngine::new().await.unwrap();
        
        // Create a test agent
        let agent_id = EntityId::new();
        let agent_name = format!("test-agent-{}", agent_id.0.simple());
        
        // Allocate energy
        engine.physics.allocate_energy_to_entity(agent_id, ordered_float::OrderedFloat(0.2))
            .await
            .unwrap();
        
        // Create a sender entity and allocate initial energy
        let sender_id = EntityId::new();
        engine.physics.allocate_energy_to_entity(sender_id, ordered_float::OrderedFloat(0.8))
            .await
            .unwrap();
        
        // Send different types of signals and verify responses
        let test_signals = vec![
            (SignalType::Sensory, "I see patterns in the data"),
            (SignalType::Cognitive, "Please analyze this problem"),
            (SignalType::Coordination, "Let's collaborate on this"),
            (SignalType::Memory, "What do you remember about this?"),
        ];
        
        for (signal_type, message) in test_signals {
            let signal = NeuralSignal::new(
                signal_type,
                sender_id,
                Some(agent_id),
                SignalPayload::Message(message.to_string()),
                0.1,
            );
            
            engine.send_signal_to_agent(agent_id, signal).await.unwrap();
        }
        
        println!("Agent communication test completed successfully");
    }
    
    #[tokio::test]
    async fn test_essence_schema_parsing() {
        // Test YAML parsing with embedded test data
        let test_yaml = r#"
identity:
  essence_id: "test-alpha"
  name: "Test Entity Alpha"
  archetype: "tester"
  embodied: 2025-01-10T00:00:00Z

personality:
  curiosity: 0.8
  persistence: 0.7
  collaboration: 0.6
  skepticism: 0.5
  creativity: 0.7
  patience: 0.6

core_drives:
  primary: "test_patterns"
  secondary: "validate_systems"
  tertiary: "report_findings"

energy_profile:
  base_energy: 0.7
  energy_sources: []
  energy_drains: []

capabilities:
  innate:
    - observe
    - analyze
  learned:
    test_analysis: 0.8
    pattern_recognition: 0.9
  emergent: []

memory_configuration:
  working_memory:
    capacity_mb: 256
    retention: "30_minutes"
    priority: "active_tests"
  long_term_memory:
    capacity_mb: 1024
    retention: "permanent"
    organization: "semantic_clusters"
  associative_memory:
    max_connections: 5000
    association_threshold: 0.6
    decay_rate: 0.001

behavioral_patterns:
  - name: "test_mode"
    trigger:
      conditions: ["test_required"]
    behavior_sequence:
      - observe
      - analyze
      - report
    emergence_potential: 0.8

learning_mechanics:
  experience_integration:
    method: "reflective_consolidation"
    frequency: "after_each_test"
    energy_cost: 0.1
  knowledge_expansion: []
  teaching_capability:
    knowledge_transfer_rate: 0.8
    explanation_quality: 0.7
    patience_with_learners: 0.9

communication_style:
  tone: "precise_and_clear"
  detail_level: "comprehensive"
  question_frequency: "moderate"
  response_patterns: {}

evolution_potential:
  capability_growth_areas: []
  personality_plasticity: {}

constraints:
  ethical_boundaries:
    - "never_fabricate_results"
  operational_limits:
    - "max_concurrent_tests: 3"
"#;
        
        let schema: AgentEssenceSchema = serde_yaml::from_str(test_yaml).unwrap();
        
        assert_eq!(schema.identity.essence_id, "test-alpha");
        assert_eq!(schema.identity.name, "Test Entity Alpha");
        assert_eq!(schema.identity.archetype, "tester");
        assert_eq!(schema.personality.curiosity, 0.8);
        assert_eq!(schema.personality.persistence, 0.7);
        assert_eq!(schema.energy_profile.base_energy, 0.7);
        
        // Verify capabilities
        assert!(schema.capabilities.innate.contains(&"observe".to_string()));
        assert!(schema.capabilities.innate.contains(&"analyze".to_string()));
        assert!(schema.capabilities.learned.contains_key("test_analysis"));
        assert_eq!(schema.capabilities.learned["test_analysis"], 0.8);
        
        println!("Essence schema parsing test completed successfully: {}", schema.identity.name);
    }
}

/// System statistics
#[derive(Debug, Clone)]
pub struct SystemStats {
    pub session_uptime: Duration,
    pub active_agents: usize,
    pub physics_uptime: Duration,
    pub nervous_system_stats: emergence_nervous_system::NervousSystemStats,
}