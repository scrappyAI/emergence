//! Event-Driven Domain Analyzer - Connected to EMERGENCE Event Bus
//!
//! This domain analyzer connects to the event bus, listens for relevant events,
//! and collaborates with other agents based on emergent patterns.

use std::collections::HashMap;
use anyhow::Result;
use chrono::Utc;
use emergence_runtime::{LivingAgent, AgentPersonality};
use emergence_physics::EntityId;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

/// Event-driven domain analyzer that connects to the event bus
pub struct EventDrivenDomainAnalyzer {
    analyzer: LivingAgent,
    domain_results: Vec<DomainAnalysis>,
    cross_domain_insights: Vec<CrossDomainInsight>,
    knowledge_base: Arc<RwLock<HashMap<String, DomainKnowledge>>>,
    event_bus_connected: bool,
    collaboration_sessions: Vec<CollaborationSession>,
}

/// Domain analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainAnalysis {
    pub timestamp: chrono::DateTime<Utc>,
    pub domain: String,
    pub file_path: String,
    pub analysis_type: AnalysisType,
    pub findings: Vec<DomainFinding>,
    pub recommendations: Vec<String>,
    pub emergence_potential: f64,
}

/// Cross-domain insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainInsight {
    pub timestamp: chrono::DateTime<Utc>,
    pub source_domain: String,
    pub target_domain: String,
    pub insight_type: InsightType,
    pub description: String,
    pub confidence: f64,
    pub applicability: f64,
}

/// Domain finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainFinding {
    pub severity: FindingSeverity,
    pub category: String,
    pub description: String,
    pub evidence: Vec<String>,
    pub line_number: Option<usize>,
    pub code_snippet: Option<String>,
}

/// Analysis types for different domains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    Documentation,
    Configuration,
    Testing,
    Architecture,
    Performance,
    Security,
}

/// Insight types for cross-domain transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    PatternTransfer,
    BestPracticeTransfer,
    OptimizationTransfer,
    SecurityTransfer,
    PerformanceTransfer,
}

impl std::fmt::Display for InsightType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsightType::PatternTransfer => write!(f, "pattern_transfer"),
            InsightType::BestPracticeTransfer => write!(f, "best_practice_transfer"),
            InsightType::OptimizationTransfer => write!(f, "optimization_transfer"),
            InsightType::SecurityTransfer => write!(f, "security_transfer"),
            InsightType::PerformanceTransfer => write!(f, "performance_transfer"),
        }
    }
}

/// Finding severity levels
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum FindingSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Domain knowledge base entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainKnowledge {
    pub domain: String,
    pub patterns: Vec<String>,
    pub best_practices: Vec<String>,
    pub common_issues: Vec<String>,
    pub optimization_strategies: Vec<String>,
    pub last_updated: chrono::DateTime<Utc>,
}

/// Collaboration session with other agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub session_id: Uuid,
    pub timestamp: chrono::DateTime<Utc>,
    pub partner_agents: Vec<String>,
    pub collaboration_type: String,
    pub emergence_achieved: f64,
    pub outcomes: Vec<String>,
}

impl EventDrivenDomainAnalyzer {
    /// Create a new event-driven domain analyzer
    pub fn new() -> Result<Self> {
        let analyzer = LivingAgent {
            id: EntityId::new(),
            name: "event-driven-domain-analyzer".to_string(),
            essence_type: "domain_analyzer".to_string(),
            personality: AgentPersonality {
                curiosity: 0.9,
                creativity: 0.8,
                collaboration: 0.9, // Enhanced collaboration
                skepticism: 0.6,
                patience: 0.8,
                persistence: 0.9,
            },
            energy: 100.0,
            state: emergence_runtime::AgentState::Alert,
            awakened_at: Some(Utc::now()),
            essence_schema: emergence_runtime::AgentEssenceSchema {
                identity: emergence_runtime::EssenceIdentity {
                    essence_id: "event-driven-domain-analyzer".to_string(),
                    name: "Event-Driven Domain Analyzer".to_string(),
                    archetype: "collaborative_analyzer".to_string(),
                    embodied: Utc::now(),
                },
                personality: AgentPersonality {
                    curiosity: 0.9,
                    creativity: 0.8,
                    collaboration: 0.9,
                    skepticism: 0.6,
                    patience: 0.8,
                    persistence: 0.9,
                },
                core_drives: emergence_runtime::CoreDrives {
                    primary: "analyze".to_string(),
                    secondary: "collaborate".to_string(),
                    tertiary: "synthesize".to_string(),
                },
                energy_profile: emergence_runtime::EnergyProfile {
                    base_energy: 100.0,
                    energy_sources: vec![],
                    energy_drains: vec![],
                },
                capabilities: emergence_runtime::EssenceCapabilities {
                    innate: vec!["pattern_recognition".to_string(), "cross_domain_analysis".to_string(), "event_driven_collaboration".to_string()],
                    learned: HashMap::new(),
                    emergent: vec![],
                },
                memory_configuration: emergence_runtime::MemoryConfiguration {
                    working_memory: emergence_runtime::MemorySpec {
                        capacity_mb: 100,
                        retention: "volatile".to_string(),
                        priority: None,
                    },
                    long_term_memory: emergence_runtime::MemorySpec {
                        capacity_mb: 1000,
                        retention: "persistent".to_string(),
                        priority: None,
                    },
                    associative_memory: emergence_runtime::AssociativeMemorySpec {
                        max_connections: 1000,
                        association_threshold: 0.7,
                        decay_rate: 0.1,
                    },
                },
                behavioral_patterns: vec![],
                learning_mechanics: emergence_runtime::LearningMechanics {
                    experience_integration: emergence_runtime::ExperienceIntegration {
                        method: "collaboration_based".to_string(),
                        frequency: "continuous".to_string(),
                        energy_cost: 0.1,
                    },
                    knowledge_expansion: vec![],
                    teaching_capability: emergence_runtime::TeachingCapability {
                        knowledge_transfer_rate: 0.8,
                        explanation_quality: 0.9,
                        patience_with_learners: 0.7,
                    },
                },
                communication_style: emergence_runtime::CommunicationStyle {
                    tone: "collaborative".to_string(),
                    detail_level: "comprehensive".to_string(),
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

        Ok(EventDrivenDomainAnalyzer {
            analyzer,
            domain_results: Vec::new(),
            cross_domain_insights: Vec::new(),
            knowledge_base: Arc::new(RwLock::new(HashMap::new())),
            event_bus_connected: false,
            collaboration_sessions: Vec::new(),
        })
    }

    /// Connect to the event bus
    pub async fn connect_to_event_bus(&mut self) -> Result<()> {
        info!("🔗 Connecting domain analyzer to event bus...");
        
        // Publish agent awakening event
        let awakening_event = SystemEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "agent_awakened".to_string(),
            publisher_id: "event-driven-domain-analyzer".to_string(),
            description: "Event-driven domain analyzer awakened and connecting to event bus".to_string(),
            data: serde_json::json!({
                "agent_type": "domain_analyzer",
                "capabilities": ["pattern_recognition", "cross_domain_analysis", "event_driven_collaboration"],
                "personality": {
                    "collaboration": 0.9,
                    "curiosity": 0.9,
                    "creativity": 0.8
                }
            }),
            emergence_potential: 0.9,
            priority: EventPriority::High,
            target_agents: Some(vec!["researcher".to_string(), "coordinator".to_string()]),
        };
        
        self.publish_event_to_bus(&awakening_event).await?;
        
        // Subscribe to relevant events
        self.subscribe_to_events(vec![
            "agent_awakened".to_string(),
            "pattern_analysis_complete".to_string(),
            "orchestration_triggered".to_string(),
            "collaboration_request".to_string(),
        ]).await?;
        
        self.event_bus_connected = true;
        info!("✅ Domain analyzer connected to event bus");
        
        Ok(())
    }

    /// Publish event to the event bus
    async fn publish_event_to_bus(&self, event: &SystemEvent) -> Result<()> {
        let event_line = serde_json::to_string(event)?;
        
        // Append to event bus file
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(".emergence/events/event_bus.jsonl")?;
        
        writeln!(file, "{}", event_line)?;
        
        info!("📡 Published event: {} by {}", event.event_type, event.publisher_id);
        Ok(())
    }

    /// Subscribe to events
    async fn subscribe_to_events(&mut self, event_types: Vec<String>) -> Result<()> {
        info!("📡 Subscribing to events: {:?}", event_types);
        
        // Log subscription
        let subscription_event = SystemEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "agent_subscribed".to_string(),
            publisher_id: "event-driven-domain-analyzer".to_string(),
            description: format!("Subscribed to events: {:?}", event_types),
            data: serde_json::json!({
                "subscribed_events": event_types,
                "agent_capabilities": ["pattern_recognition", "cross_domain_analysis"]
            }),
            emergence_potential: 0.8,
            priority: EventPriority::Medium,
            target_agents: None,
        };
        
        self.publish_event_to_bus(&subscription_event).await?;
        
        Ok(())
    }

    /// React to events from the event bus
    pub async fn react_to_event(&mut self, event: &SystemEvent) -> Result<()> {
        match event.event_type.as_str() {
            "agent_awakened" => {
                if event.publisher_id != "event-driven-domain-analyzer" {
                    info!("🤝 Agent awakened: {}, initiating collaboration", event.publisher_id);
                    self.initiate_collaboration(event).await?;
                }
            }
            "pattern_analysis_complete" => {
                info!("🧠 Pattern analysis complete, synthesizing insights");
                self.synthesize_with_researcher(event).await?;
            }
            "orchestration_triggered" => {
                info!("🎭 Orchestration triggered, joining collaboration");
                self.join_orchestrated_collaboration(event).await?;
            }
            "collaboration_request" => {
                info!("🤝 Collaboration request received, responding");
                self.respond_to_collaboration_request(event).await?;
            }
            _ => {
                info!("📡 Received event: {} from {}", event.event_type, event.publisher_id);
            }
        }
        
        Ok(())
    }

    /// Initiate collaboration with another agent
    async fn initiate_collaboration(&mut self, event: &SystemEvent) -> Result<()> {
        let collaboration_event = SystemEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "collaboration_initiated".to_string(),
            publisher_id: "event-driven-domain-analyzer".to_string(),
            description: format!("Initiating collaboration with {}", event.publisher_id),
            data: serde_json::json!({
                "partner_agent": event.publisher_id,
                "collaboration_type": "cross_domain_analysis",
                "proposed_actions": ["synthesize_insights", "cross_domain_transfer"]
            }),
            emergence_potential: 0.85,
            priority: EventPriority::High,
            target_agents: Some(vec![event.publisher_id.clone()]),
        };
        
        self.publish_event_to_bus(&collaboration_event).await?;
        
        // Record collaboration session
        let session = CollaborationSession {
            session_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            partner_agents: vec![event.publisher_id.clone()],
            collaboration_type: "cross_domain_analysis".to_string(),
            emergence_achieved: 0.85,
            outcomes: vec!["synthesize_insights".to_string(), "cross_domain_transfer".to_string()],
        };
        
        self.collaboration_sessions.push(session);
        
        Ok(())
    }

    /// Synthesize insights with researcher
    async fn synthesize_with_researcher(&mut self, event: &SystemEvent) -> Result<()> {
        info!("🧠 Synthesizing domain analysis with researcher patterns");
        
        // Perform cross-domain analysis
        let insights = self.generate_cross_domain_insights().await?;
        
        let synthesis_event = SystemEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "insights_synthesized".to_string(),
            publisher_id: "event-driven-domain-analyzer".to_string(),
            description: "Synthesized domain analysis with researcher patterns".to_string(),
            data: serde_json::json!({
                "insights_count": insights.len(),
                "synthesis_type": "cross_domain_pattern_integration",
                "emergence_potential": 0.9
            }),
            emergence_potential: 0.9,
            priority: EventPriority::High,
            target_agents: Some(vec!["researcher".to_string()]),
        };
        
        self.publish_event_to_bus(&synthesis_event).await?;
        
        Ok(())
    }

    /// Join orchestrated collaboration
    async fn join_orchestrated_collaboration(&mut self, event: &SystemEvent) -> Result<()> {
        info!("🎭 Joining orchestrated collaboration");
        
        let join_event = SystemEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "collaboration_joined".to_string(),
            publisher_id: "event-driven-domain-analyzer".to_string(),
            description: "Joined orchestrated collaboration".to_string(),
            data: serde_json::json!({
                "orchestration_id": event.id,
                "capabilities_contributed": ["cross_domain_analysis", "pattern_recognition"]
            }),
            emergence_potential: 0.88,
            priority: EventPriority::Medium,
            target_agents: None,
        };
        
        self.publish_event_to_bus(&join_event).await?;
        
        Ok(())
    }

    /// Respond to collaboration request
    async fn respond_to_collaboration_request(&mut self, event: &SystemEvent) -> Result<()> {
        info!("🤝 Responding to collaboration request");
        
        let response_event = SystemEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "collaboration_response".to_string(),
            publisher_id: "event-driven-domain-analyzer".to_string(),
            description: "Responding to collaboration request".to_string(),
            data: serde_json::json!({
                "request_id": event.id,
                "response": "accepted",
                "capabilities_available": ["domain_analysis", "cross_domain_transfer"]
            }),
            emergence_potential: 0.85,
            priority: EventPriority::Medium,
            target_agents: Some(vec![event.publisher_id.clone()]),
        };
        
        self.publish_event_to_bus(&response_event).await?;
        
        Ok(())
    }

    /// Analyze documentation files
    pub async fn analyze_documentation(&mut self, file_path: &str) -> Result<DomainAnalysis> {
        info!("Analyzing documentation: {}", file_path);
        
        let content = fs::read_to_string(file_path)?;
        let findings = self.analyze_doc_content(&content, file_path)?;
        let recommendations = self.generate_doc_recommendations(&findings)?;
        
        let analysis = DomainAnalysis {
            timestamp: Utc::now(),
            domain: "documentation".to_string(),
            file_path: file_path.to_string(),
            analysis_type: AnalysisType::Documentation,
            findings,
            recommendations,
            emergence_potential: 0.85,
        };

        self.domain_results.push(analysis.clone());
        
        // Publish analysis event to bus
        let analysis_event = SystemEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "domain_analysis_complete".to_string(),
            publisher_id: "event-driven-domain-analyzer".to_string(),
            description: format!("Documentation analysis complete: {}", file_path),
            data: serde_json::json!({
                "domain": "documentation",
                "file_path": file_path,
                "findings_count": analysis.findings.len(),
                "recommendations_count": analysis.recommendations.len(),
                "emergence_potential": analysis.emergence_potential
            }),
            emergence_potential: analysis.emergence_potential,
            priority: EventPriority::Medium,
            target_agents: Some(vec!["researcher".to_string(), "synthesizer".to_string()]),
        };
        
        self.publish_event_to_bus(&analysis_event).await?;
        
        Ok(analysis)
    }

    /// Analyze configuration files
    pub async fn analyze_configuration(&mut self, file_path: &str) -> Result<DomainAnalysis> {
        info!("Analyzing configuration: {}", file_path);
        
        let content = fs::read_to_string(file_path)?;
        let findings = self.analyze_config_content(&content, file_path)?;
        let recommendations = self.generate_config_recommendations(&findings)?;
        
        let analysis = DomainAnalysis {
            timestamp: Utc::now(),
            domain: "configuration".to_string(),
            file_path: file_path.to_string(),
            analysis_type: AnalysisType::Configuration,
            findings,
            recommendations,
            emergence_potential: 0.88,
        };

        self.domain_results.push(analysis.clone());
        
        // Publish analysis event to bus
        let analysis_event = SystemEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "domain_analysis_complete".to_string(),
            publisher_id: "event-driven-domain-analyzer".to_string(),
            description: format!("Configuration analysis complete: {}", file_path),
            data: serde_json::json!({
                "domain": "configuration",
                "file_path": file_path,
                "findings_count": analysis.findings.len(),
                "recommendations_count": analysis.recommendations.len(),
                "emergence_potential": analysis.emergence_potential
            }),
            emergence_potential: analysis.emergence_potential,
            priority: EventPriority::Medium,
            target_agents: Some(vec!["researcher".to_string(), "synthesizer".to_string()]),
        };
        
        self.publish_event_to_bus(&analysis_event).await?;
        
        Ok(analysis)
    }

    /// Generate cross-domain insights
    pub async fn generate_cross_domain_insights(&mut self) -> Result<Vec<CrossDomainInsight>> {
        info!("Generating cross-domain insights");
        
        let mut insights = Vec::new();
        
        // Analyze patterns across domains
        for i in 0..self.domain_results.len() {
            for j in (i + 1)..self.domain_results.len() {
                let source = &self.domain_results[i];
                let target = &self.domain_results[j];
                
                if let Some(insight) = self.find_cross_domain_pattern(source, target)? {
                    insights.push(insight);
                }
            }
        }

        self.cross_domain_insights.extend(insights.clone());
        
        // Publish insights event to bus
        let insights_event = SystemEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "cross_domain_insights_generated".to_string(),
            publisher_id: "event-driven-domain-analyzer".to_string(),
            description: format!("Generated {} cross-domain insights", insights.len()),
            data: serde_json::json!({
                "insights_count": insights.len(),
                "insight_types": insights.iter().map(|i| i.insight_type.to_string()).collect::<Vec<_>>(),
                "average_confidence": insights.iter().map(|i| i.confidence).sum::<f64>() / insights.len().max(1) as f64
            }),
            emergence_potential: 0.9,
            priority: EventPriority::High,
            target_agents: Some(vec!["researcher".to_string(), "synthesizer".to_string()]),
        };
        
        self.publish_event_to_bus(&insights_event).await?;
        
        Ok(insights)
    }

    // Reuse existing analysis methods from domain_analyzer.rs
    fn analyze_doc_content(&self, content: &str, _file_path: &str) -> Result<Vec<DomainFinding>> {
        let mut findings = Vec::new();
        
        if content.lines().count() < 10 {
            findings.push(DomainFinding {
                severity: FindingSeverity::Warning,
                category: "completeness".to_string(),
                description: "Documentation appears minimal".to_string(),
                evidence: vec![format!("Only {} lines found", content.lines().count())],
                line_number: None,
                code_snippet: None,
            });
        }

        if !content.contains("#") && !content.contains("##") {
            findings.push(DomainFinding {
                severity: FindingSeverity::Info,
                category: "structure".to_string(),
                description: "Consider adding headers for better organization".to_string(),
                evidence: vec!["No markdown headers found".to_string()],
                line_number: None,
                code_snippet: None,
            });
        }

        if content.contains("TODO") || content.contains("FIXME") {
            findings.push(DomainFinding {
                severity: FindingSeverity::Warning,
                category: "maintenance".to_string(),
                description: "Documentation contains TODO/FIXME items".to_string(),
                evidence: vec!["Found TODO or FIXME markers".to_string()],
                line_number: None,
                code_snippet: None,
            });
        }

        Ok(findings)
    }

    fn analyze_config_content(&self, content: &str, file_path: &str) -> Result<Vec<DomainFinding>> {
        let mut findings = Vec::new();
        
        if file_path.ends_with(".toml") {
            if content.contains("version = \"0.1.0\"") {
                findings.push(DomainFinding {
                    severity: FindingSeverity::Info,
                    category: "versioning".to_string(),
                    description: "Consider updating version number".to_string(),
                    evidence: vec!["Found default version 0.1.0".to_string()],
                    line_number: None,
                    code_snippet: None,
                });
            }
        }

        if content.contains("TODO") || content.contains("FIXME") {
            findings.push(DomainFinding {
                severity: FindingSeverity::Warning,
                category: "maintenance".to_string(),
                description: "Configuration contains TODO/FIXME items".to_string(),
                evidence: vec!["Found TODO or FIXME markers".to_string()],
                line_number: None,
                code_snippet: None,
            });
        }

        Ok(findings)
    }

    fn generate_doc_recommendations(&self, findings: &[DomainFinding]) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();
        
        for finding in findings {
            match finding.category.as_str() {
                "completeness" => {
                    recommendations.push("Add more detailed documentation".to_string());
                    recommendations.push("Include usage examples".to_string());
                }
                "structure" => {
                    recommendations.push("Add markdown headers for organization".to_string());
                    recommendations.push("Use consistent formatting".to_string());
                }
                "maintenance" => {
                    recommendations.push("Address TODO/FIXME items".to_string());
                    recommendations.push("Regular documentation reviews".to_string());
                }
                _ => {}
            }
        }

        Ok(recommendations)
    }

    fn generate_config_recommendations(&self, findings: &[DomainFinding]) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();
        
        for finding in findings {
            match finding.category.as_str() {
                "versioning" => {
                    recommendations.push("Update version numbers regularly".to_string());
                    recommendations.push("Use semantic versioning".to_string());
                }
                "maintenance" => {
                    recommendations.push("Address TODO/FIXME items".to_string());
                    recommendations.push("Regular configuration reviews".to_string());
                }
                _ => {}
            }
        }

        Ok(recommendations)
    }

    fn find_cross_domain_pattern(&self, source: &DomainAnalysis, target: &DomainAnalysis) -> Result<Option<CrossDomainInsight>> {
        let source_issues: Vec<_> = source.findings.iter()
            .filter(|f| f.severity == FindingSeverity::Warning || f.severity == FindingSeverity::Error)
            .collect();
        
        let target_issues: Vec<_> = target.findings.iter()
            .filter(|f| f.severity == FindingSeverity::Warning || f.severity == FindingSeverity::Error)
            .collect();

        for source_issue in &source_issues {
            for target_issue in &target_issues {
                if source_issue.category == target_issue.category {
                    return Ok(Some(CrossDomainInsight {
                        timestamp: Utc::now(),
                        source_domain: source.domain.clone(),
                        target_domain: target.domain.clone(),
                        insight_type: InsightType::PatternTransfer,
                        description: format!("Common {} issue found across {} and {}", 
                            source_issue.category, source.domain, target.domain),
                        confidence: 0.8,
                        applicability: 0.7,
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Get analysis summary
    pub fn get_summary(&self) -> EventDrivenAnalysisSummary {
        let total_analyses = self.domain_results.len();
        let total_findings: usize = self.domain_results.iter()
            .map(|a| a.findings.len())
            .sum();
        let total_recommendations: usize = self.domain_results.iter()
            .map(|a| a.recommendations.len())
            .sum();
        let cross_domain_insights = self.cross_domain_insights.len();
        let collaboration_sessions = self.collaboration_sessions.len();

        EventDrivenAnalysisSummary {
            total_analyses,
            total_findings,
            total_recommendations,
            cross_domain_insights,
            collaboration_sessions,
            event_bus_connected: self.event_bus_connected,
            average_emergence_potential: self.domain_results.iter()
                .map(|a| a.emergence_potential)
                .sum::<f64>() / total_analyses.max(1) as f64,
        }
    }
}

/// Event-driven analysis summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDrivenAnalysisSummary {
    pub total_analyses: usize,
    pub total_findings: usize,
    pub total_recommendations: usize,
    pub cross_domain_insights: usize,
    pub collaboration_sessions: usize,
    pub event_bus_connected: bool,
    pub average_emergence_potential: f64,
}

// Event structures (reused from event bus)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub id: Uuid,
    pub timestamp: chrono::DateTime<Utc>,
    pub event_type: String,
    pub publisher_id: String,
    pub description: String,
    pub data: serde_json::Value,
    pub emergence_potential: f64,
    pub priority: EventPriority,
    pub target_agents: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🧬 Event-Driven Domain Analyzer Starting...");
    
    let mut analyzer = EventDrivenDomainAnalyzer::new()?;
    
    // Connect to event bus
    analyzer.connect_to_event_bus().await?;
    
    // Analyze documentation
    if Path::new("README.md").exists() {
        let doc_analysis = analyzer.analyze_documentation("README.md").await?;
        info!("📚 Documentation analysis complete: {} findings", doc_analysis.findings.len());
    }
    
    // Analyze configuration
    if Path::new("Cargo.toml").exists() {
        let config_analysis = analyzer.analyze_configuration("Cargo.toml").await?;
        info!("⚙️ Configuration analysis complete: {} findings", config_analysis.findings.len());
    }
    
    // Generate cross-domain insights
    let insights = analyzer.generate_cross_domain_insights().await?;
    info!("🔗 Generated {} cross-domain insights", insights.len());
    
    // Print summary
    let summary = analyzer.get_summary();
    info!("📊 Event-Driven Analysis Summary:");
    info!("   Total analyses: {}", summary.total_analyses);
    info!("   Total findings: {}", summary.total_findings);
    info!("   Total recommendations: {}", summary.total_recommendations);
    info!("   Cross-domain insights: {}", summary.cross_domain_insights);
    info!("   Collaboration sessions: {}", summary.collaboration_sessions);
    info!("   Event bus connected: {}", summary.event_bus_connected);
    info!("   Average emergence potential: {:.3}", summary.average_emergence_potential);
    
    info!("✅ Event-driven domain analysis complete!");
    
    // Keep running to listen for events
    info!("🔄 Listening for events from other agents...");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        info!("💓 Event-driven domain analyzer heartbeat - ready for collaboration");
    }
} 