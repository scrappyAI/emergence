//! EMERGENCE Event Bus - Central Nervous System for Agent Collaboration
//!
//! This event bus enables true event-driven agent collaboration, allowing agents
//! to subscribe to events, publish events, and react to system-wide patterns.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use uuid::Uuid;
use std::io::Write;
use emergence_runtime::{LivingAgent, AgentPersonality};
use emergence_physics::EntityId;

/// Central event bus for agent collaboration and emergence
pub struct EmergenceEventBus {
    // Event channels
    event_publisher: broadcast::Sender<SystemEvent>,
    event_subscribers: HashMap<String, Vec<AgentSubscription>>,
    
    // Agent registry
    active_agents: Arc<RwLock<HashMap<String, LivingAgent>>>,
    
    // Event storage
    event_history: Arc<RwLock<Vec<SystemEvent>>>,
    
    // Emergence tracking
    emergence_patterns: Arc<RwLock<Vec<EmergencePattern>>>,
    
    // Coordinator for emergent orchestration
    coordinator: Option<CoordinatorAgent>,
}

/// System event that agents can publish and subscribe to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub publisher_id: String,
    pub description: String,
    pub data: serde_json::Value,
    pub emergence_potential: f64,
    pub priority: EventPriority,
    pub target_agents: Option<Vec<String>>,
}

/// Event priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Agent subscription to event types
#[derive(Debug)]
pub struct AgentSubscription {
    pub agent_id: String,
    pub event_types: Vec<String>,
    pub priority_filter: Option<EventPriority>,
}

/// Emergence pattern detected by the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencePattern {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub pattern_type: String,
    pub description: String,
    pub confidence: f64,
    pub involved_agents: Vec<String>,
    pub emergence_potential: f64,
    pub suggested_actions: Vec<String>,
}

/// Coordinator agent for emergent orchestration
pub struct CoordinatorAgent {
    pub agent: LivingAgent,
    pub orchestration_rules: Vec<OrchestrationRule>,
    pub collaboration_history: Vec<CollaborationSession>,
}

/// Orchestration rule for agent coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationRule {
    pub trigger_conditions: Vec<String>,
    pub agent_combination: Vec<String>,
    pub expected_emergence: f64,
    pub action_sequence: Vec<String>,
}

/// Collaboration session between agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub session_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub agent_ids: Vec<String>,
    pub emergence_achieved: f64,
    pub duration: std::time::Duration,
    pub outcomes: Vec<String>,
}

impl EmergenceEventBus {
    /// Create a new event bus
    pub async fn new() -> Result<Self> {
        let (event_publisher, _) = broadcast::channel(1000);
        
        Ok(EmergenceEventBus {
            event_publisher,
            event_subscribers: HashMap::new(),
            active_agents: Arc::new(RwLock::new(HashMap::new())),
            event_history: Arc::new(RwLock::new(Vec::new())),
            emergence_patterns: Arc::new(RwLock::new(Vec::new())),
            coordinator: None,
        })
    }

    /// Register an agent with the event bus
    pub async fn register_agent(&mut self, agent: LivingAgent, event_types: Vec<String>) -> Result<()> {
        let agent_id = agent.name.clone();
        
        // Add agent to active agents
        let mut agents = self.active_agents.write().await;
        agents.insert(agent_id.clone(), agent);
        
        // Register for each event type
        for event_type in &event_types {
            let subscription = AgentSubscription {
                agent_id: agent_id.clone(),
                event_types: vec![event_type.clone()],
                priority_filter: None,
            };
            
            self.event_subscribers
                .entry(event_type.clone())
                .or_insert_with(Vec::new)
                .push(subscription);
        }
        
        info!("🤖 Agent {} registered with event bus", agent_id);
        Ok(())
    }

    /// Publish an event to the bus
    pub async fn publish_event(&mut self, event: SystemEvent) -> Result<()> {
        // Store event in history
        {
            let mut history = self.event_history.write().await;
            history.push(event.clone());
        }
        
        // Log event to file
        self.log_event_to_file(&event)?;
        
        // Broadcast to subscribers
        if let Err(e) = self.event_publisher.send(event.clone()) {
            warn!("Failed to broadcast event: {}", e);
        }
        
        // Check for emergence patterns
        self.detect_emergence_patterns(&event).await?;
        
        // Trigger coordinator if present
        if let Some(coordinator) = &mut self.coordinator {
            coordinator.react_to_event(&event).await?;
        }
        
        info!("📡 Event published: {} by {}", event.event_type, event.publisher_id);
        Ok(())
    }

    /// Subscribe to events
    pub async fn subscribe_to_events(&mut self, agent_id: &str, event_types: Vec<String>) -> Result<()> {
        for event_type in event_types {
            let subscription = AgentSubscription {
                agent_id: agent_id.to_string(),
                event_types: vec![event_type.clone()],
                priority_filter: None,
            };
            
            self.event_subscribers
                .entry(event_type)
                .or_insert_with(Vec::new)
                .push(subscription);
        }
        
        info!("📡 Agent {} subscribed to events", agent_id);
        Ok(())
    }

    /// Detect emergence patterns in events
    async fn detect_emergence_patterns(&mut self, event: &SystemEvent) -> Result<()> {
        let mut patterns = self.emergence_patterns.write().await;
        let history = self.event_history.read().await;
        
        // Look for collaboration patterns
        if event.event_type == "agent_awakened" && history.len() > 1 {
            let recent_agents: Vec<_> = history.iter()
                .filter(|e| e.event_type == "agent_awakened")
                .take(3)
                .map(|e| e.publisher_id.clone())
                .collect();
            
            if recent_agents.len() >= 2 {
                let pattern = EmergencePattern {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    pattern_type: "collaboration_emergence".to_string(),
                    description: format!("Multiple agents awakened: {:?}", recent_agents),
                    confidence: 0.8,
                    involved_agents: recent_agents,
                    emergence_potential: 0.85,
                    suggested_actions: vec![
                        "orchestrate_collaboration".to_string(),
                        "optimize_agent_combinations".to_string(),
                    ],
                };
                
                patterns.push(pattern);
                info!("🧬 Emergence pattern detected: collaboration_emergence");
            }
        }
        
        // Look for learning patterns
        if event.event_type.contains("analysis") || event.event_type.contains("finding") {
            let learning_events: Vec<_> = history.iter()
                .filter(|e| e.event_type.contains("analysis") || e.event_type.contains("finding"))
                .take(5)
                .collect();
            
            if learning_events.len() >= 3 {
                let pattern = EmergencePattern {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    pattern_type: "learning_acceleration".to_string(),
                    description: "Rapid learning and analysis events detected".to_string(),
                    confidence: 0.7,
                    involved_agents: learning_events.iter().map(|e| e.publisher_id.clone()).collect(),
                    emergence_potential: 0.9,
                    suggested_actions: vec![
                        "synthesize_knowledge".to_string(),
                        "cross_domain_transfer".to_string(),
                    ],
                };
                
                patterns.push(pattern);
                info!("🧬 Emergence pattern detected: learning_acceleration");
            }
        }
        
        Ok(())
    }

    /// Log event to file for persistence
    fn log_event_to_file(&self, event: &SystemEvent) -> Result<()> {
        let event_line = serde_json::to_string(event)?;
        
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(".emergence/events/event_bus.jsonl")?;
        
        writeln!(file, "{}", event_line)?;
        Ok(())
    }

    /// Get system statistics
    pub async fn get_stats(&self) -> EventBusStats {
        let agents = self.active_agents.read().await;
        let history = self.event_history.read().await;
        let patterns = self.emergence_patterns.read().await;
        
        EventBusStats {
            active_agents: agents.len(),
            total_events: history.len(),
            emergence_patterns: patterns.len(),
            average_emergence_potential: history.iter()
                .map(|e| e.emergence_potential)
                .sum::<f64>() / history.len().max(1) as f64,
        }
    }

    /// Awaken coordinator agent
    pub async fn awaken_coordinator(&mut self) -> Result<()> {
        let coordinator_agent = LivingAgent {
            id: EntityId::new(),
            name: "coordinator".to_string(),
            essence_type: "coordinator".to_string(),
            personality: AgentPersonality {
                curiosity: 0.7,
                creativity: 0.8,
                collaboration: 0.95,
                skepticism: 0.5,
                patience: 0.9,
                persistence: 0.8,
            },
            energy: 100.0,
            state: emergence_runtime::AgentState::Alert,
            awakened_at: Some(Utc::now()),
            essence_schema: emergence_runtime::AgentEssenceSchema {
                identity: emergence_runtime::EssenceIdentity {
                    essence_id: "coordinator".to_string(),
                    name: "Coordinator".to_string(),
                    archetype: "orchestrator".to_string(),
                    embodied: Utc::now(),
                },
                personality: AgentPersonality {
                    curiosity: 0.7,
                    creativity: 0.8,
                    collaboration: 0.95,
                    skepticism: 0.5,
                    patience: 0.9,
                    persistence: 0.8,
                },
                core_drives: emergence_runtime::CoreDrives {
                    primary: "orchestrate".to_string(),
                    secondary: "optimize".to_string(),
                    tertiary: "facilitate".to_string(),
                },
                energy_profile: emergence_runtime::EnergyProfile {
                    base_energy: 100.0,
                    energy_sources: vec![],
                    energy_drains: vec![],
                },
                capabilities: emergence_runtime::EssenceCapabilities {
                    innate: vec!["agent_orchestration".to_string(), "pattern_recognition".to_string()],
                    learned: HashMap::new(),
                    emergent: vec![],
                },
                memory_configuration: emergence_runtime::MemoryConfiguration {
                    working_memory: emergence_runtime::MemorySpec {
                        capacity_mb: 200,
                        retention: "volatile".to_string(),
                        priority: None,
                    },
                    long_term_memory: emergence_runtime::MemorySpec {
                        capacity_mb: 2000,
                        retention: "persistent".to_string(),
                        priority: None,
                    },
                    associative_memory: emergence_runtime::AssociativeMemorySpec {
                        max_connections: 2000,
                        association_threshold: 0.6,
                        decay_rate: 0.05,
                    },
                },
                behavioral_patterns: vec![],
                learning_mechanics: emergence_runtime::LearningMechanics {
                    experience_integration: emergence_runtime::ExperienceIntegration {
                        method: "collaboration_based".to_string(),
                        frequency: "continuous".to_string(),
                        energy_cost: 0.2,
                    },
                    knowledge_expansion: vec![],
                    teaching_capability: emergence_runtime::TeachingCapability {
                        knowledge_transfer_rate: 0.9,
                        explanation_quality: 0.8,
                        patience_with_learners: 0.9,
                    },
                },
                communication_style: emergence_runtime::CommunicationStyle {
                    tone: "facilitative".to_string(),
                    detail_level: "strategic".to_string(),
                    question_frequency: "moderate".to_string(),
                    response_patterns: HashMap::new(),
                },
                evolution_potential: emergence_runtime::EvolutionPotential {
                    capability_growth_areas: vec![],
                    personality_plasticity: HashMap::new(),
                },
                constraints: emergence_runtime::AgentConstraints {
                    ethical_boundaries: vec![],
                    operational_limits: vec![],
                },
            },
            capabilities: HashMap::new(),
            behavioral_patterns: vec![],
        };

        let coordinator = CoordinatorAgent {
            agent: coordinator_agent,
            orchestration_rules: vec![
                OrchestrationRule {
                    trigger_conditions: vec!["multiple_agents_awakened".to_string()],
                    agent_combination: vec!["researcher".to_string(), "debugger".to_string()],
                    expected_emergence: 0.85,
                    action_sequence: vec!["initiate_collaboration".to_string(), "optimize_patterns".to_string()],
                },
                OrchestrationRule {
                    trigger_conditions: vec!["learning_pattern_detected".to_string()],
                    agent_combination: vec!["synthesizer".to_string(), "domain_analyzer".to_string()],
                    expected_emergence: 0.9,
                    action_sequence: vec!["synthesize_knowledge".to_string(), "cross_domain_transfer".to_string()],
                },
            ],
            collaboration_history: Vec::new(),
        };

        self.coordinator = Some(coordinator);
        info!("🎭 Coordinator agent awakened for emergent orchestration");
        
        Ok(())
    }
}

impl CoordinatorAgent {
    /// React to events and orchestrate agent collaboration
    pub async fn react_to_event(&mut self, event: &SystemEvent) -> Result<()> {
        // Check orchestration rules
        for rule in &self.orchestration_rules {
            if self.should_trigger_rule(rule, event) {
                info!("🎭 Coordinator triggering orchestration rule: {:?}", rule.action_sequence);
                
                // Create collaboration session
                let session = CollaborationSession {
                    session_id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    agent_ids: rule.agent_combination.clone(),
                    emergence_achieved: rule.expected_emergence,
                    duration: std::time::Duration::from_secs(60),
                    outcomes: rule.action_sequence.clone(),
                };
                
                self.collaboration_history.push(session);
                
                // Log orchestration event
                let orchestration_event = SystemEvent {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    event_type: "orchestration_triggered".to_string(),
                    publisher_id: "coordinator".to_string(),
                    description: format!("Orchestrating collaboration: {:?}", rule.agent_combination),
                    data: serde_json::json!({
                        "rule": rule.action_sequence,
                        "expected_emergence": rule.expected_emergence,
                    }),
                    emergence_potential: rule.expected_emergence,
                    priority: EventPriority::High,
                    target_agents: Some(rule.agent_combination.clone()),
                };
                
                // Note: In a real implementation, we'd publish this back to the event bus
                info!("🎭 Orchestration event: {}", orchestration_event.description);
            }
        }
        
        Ok(())
    }

    /// Check if a rule should be triggered
    fn should_trigger_rule(&self, rule: &OrchestrationRule, event: &SystemEvent) -> bool {
        for condition in &rule.trigger_conditions {
            match condition.as_str() {
                "multiple_agents_awakened" => {
                    if event.event_type == "agent_awakened" {
                        return true;
                    }
                }
                "learning_pattern_detected" => {
                    if event.event_type.contains("analysis") || event.event_type.contains("finding") {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}

/// Event bus statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBusStats {
    pub active_agents: usize,
    pub total_events: usize,
    pub emergence_patterns: usize,
    pub average_emergence_potential: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🧬 EMERGENCE Event Bus Starting...");
    
    let mut event_bus = EmergenceEventBus::new().await?;
    
    // Awaken coordinator
    event_bus.awaken_coordinator().await?;
    
    // Publish system startup event
    let startup_event = SystemEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        event_type: "system_startup".to_string(),
        publisher_id: "event_bus".to_string(),
        description: "Event bus system started".to_string(),
        data: serde_json::json!({
            "version": "1.0.0",
            "capabilities": ["agent_registration", "event_publishing", "pattern_detection", "orchestration"]
        }),
        emergence_potential: 0.9,
        priority: EventPriority::High,
        target_agents: None,
    };
    
    event_bus.publish_event(startup_event).await?;
    
    // Get and display stats
    let stats = event_bus.get_stats().await;
    info!("📊 Event Bus Stats:");
    info!("   Active agents: {}", stats.active_agents);
    info!("   Total events: {}", stats.total_events);
    info!("   Emergence patterns: {}", stats.emergence_patterns);
    info!("   Average emergence potential: {:.3}", stats.average_emergence_potential);
    
    info!("✅ Event bus ready for agent collaboration!");
    
    // Keep the event bus running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        
        // Update stats periodically
        let stats = event_bus.get_stats().await;
        info!("📊 Event Bus Status - Agents: {}, Events: {}, Patterns: {}", 
            stats.active_agents, stats.total_events, stats.emergence_patterns);
    }
} 