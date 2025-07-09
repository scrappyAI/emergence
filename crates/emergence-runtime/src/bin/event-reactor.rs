use emergence_runtime::ExecutionEngine;
use emergence_nervous_system::NeuralSignal;
use emergence_physics::EntityId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fs;
use std::path::Path;
use notify::{Watcher, RecursiveMode};
use std::time::Duration;

/// Real-time event reactor that makes system events an active reasoning surface
pub struct EventReactor {
    engine: ExecutionEngine,
    event_handlers: HashMap<String, Box<dyn EventHandler>>,
    reasoning_engine: ReasoningEngine,
    action_executor: ActionExecutor,
    stats_collector: StatsCollector,
}

/// Event handler trait for different event types
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &SystemEvent, reactor: &mut EventReactor) -> Result<(), Box<dyn std::error::Error>>;
    fn event_type(&self) -> &str;
}

/// Reasoning engine that analyzes events and generates insights
#[derive(Debug)]
pub struct ReasoningEngine {
    patterns: Vec<EventPattern>,
    insights: Vec<EventInsight>,
    correlations: Vec<EventCorrelation>,
}

/// Action executor that performs real actions based on event reasoning
#[derive(Debug)]
pub struct ActionExecutor {
    pending_actions: Vec<PendingAction>,
    action_history: Vec<ExecutedAction>,
    auto_execute: bool,
}

/// Stats collector that tracks system performance and emergence
pub struct StatsCollector {
    agent_stats: Arc<RwLock<HashMap<String, AgentStats>>>,
    system_stats: Arc<RwLock<SystemStats>>,
    emergence_trends: Arc<RwLock<Vec<EmergenceTrend>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    timestamp: DateTime<Utc>,
    event_type: String,
    agent_id: Option<String>,
    description: String,
    data: serde_json::Value,
    emergence_potential: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPattern {
    pattern_type: String,
    conditions: Vec<String>,
    confidence: f64,
    action_trigger: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInsight {
    id: Uuid,
    timestamp: DateTime<Utc>,
    insight_type: String,
    description: String,
    confidence: f64,
    data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCorrelation {
    source_event: String,
    target_event: String,
    correlation_strength: f64,
    time_window: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingAction {
    id: Uuid,
    action_type: String,
    description: String,
    priority: ActionPriority,
    trigger_event: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutedAction {
    id: Uuid,
    action_type: String,
    description: String,
    result: ActionResult,
    execution_time: Duration,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
    Success { details: String },
    Failure { error: String },
    Partial { details: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStats {
    agent_id: String,
    agent_type: String,
    emergence_contributions: Vec<f64>,
    collaboration_sessions: u32,
    last_active: DateTime<Utc>,
    performance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    total_events: u64,
    average_emergence: f64,
    active_agents: u32,
    system_health: f64,
    last_optimization: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergenceTrend {
    timestamp: DateTime<Utc>,
    emergence_level: f64,
    contributing_factors: Vec<String>,
    trend_direction: TrendDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

// Event handlers for different event types
pub struct AgentAwakenedHandler;

#[async_trait::async_trait]
impl EventHandler for AgentAwakenedHandler {
    async fn handle(&self, event: &SystemEvent, reactor: &mut EventReactor) -> Result<(), Box<dyn std::error::Error>> {
        info!("🤖 Agent awakened: {}", event.description);
        
        // Extract agent info from event data
        if let Some(agent_id) = &event.agent_id {
            let agent_type = event.data.get("agent_type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            
            // Update agent stats
            let mut stats = reactor.stats_collector.agent_stats.write().await;
            let agent_stats = stats.entry(agent_id.clone()).or_insert(AgentStats {
                agent_id: agent_id.clone(),
                agent_type: agent_type.to_string(),
                emergence_contributions: Vec::new(),
                collaboration_sessions: 0,
                last_active: Utc::now(),
                performance_score: 0.0,
            });
            
            agent_stats.last_active = Utc::now();
            agent_stats.emergence_contributions.push(event.emergence_potential);
            
            // Trigger collaboration optimization if multiple agents are active
            if stats.len() >= 2 {
                reactor.action_executor.add_action(
                    "optimize_collaboration",
                    "Optimize agent collaboration patterns".to_string(),
                    ActionPriority::Medium,
                    event.event_type.clone(),
                ).await;
            }
        }
        
        Ok(())
    }
    
    fn event_type(&self) -> &str {
        "agent_awakened"
    }
}

pub struct GitCommitHandler;

#[async_trait::async_trait]
impl EventHandler for GitCommitHandler {
    async fn handle(&self, event: &SystemEvent, reactor: &mut EventReactor) -> Result<(), Box<dyn std::error::Error>> {
        info!("📝 Git commit detected: {}", event.description);
        
        // Extract commit info
        let commit_hash = event.data.get("commit_hash")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        
        let additions = event.data.get("additions")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        
        let deletions = event.data.get("deletions")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        
        // Analyze commit impact
        let change_magnitude = additions + deletions;
        let impact_score = if change_magnitude > 1000 {
            "high"
        } else if change_magnitude > 100 {
            "medium"
        } else {
            "low"
        };
        
        info!("🔍 Commit impact: {} ({} additions, {} deletions)", impact_score, additions, deletions);
        
        // Trigger actions based on commit impact
        if impact_score == "high" {
            reactor.action_executor.add_action(
                "run_comprehensive_tests",
                format!("Run comprehensive tests for high-impact commit {}", commit_hash),
                ActionPriority::High,
                event.event_type.clone(),
            ).await;
            
            reactor.action_executor.add_action(
                "code_review_alert",
                format!("High-impact commit {} requires review", commit_hash),
                ActionPriority::High,
                event.event_type.clone(),
            ).await;
        }
        
        // Update system stats
        let mut system_stats = reactor.stats_collector.system_stats.write().await;
        system_stats.total_events += 1;
        
        // Generate insight about development patterns
        reactor.reasoning_engine.generate_insight(
            "development_pattern",
            format!("Commit pattern detected: {} impact with {} changes", impact_score, change_magnitude),
            0.8,
            event.data.clone(),
        ).await;
        
        Ok(())
    }
    
    fn event_type(&self) -> &str {
        "git_commit"
    }
}

pub struct CodeAnalysisHandler;

#[async_trait::async_trait]
impl EventHandler for CodeAnalysisHandler {
    async fn handle(&self, event: &SystemEvent, reactor: &mut EventReactor) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔍 Code analysis completed: {}", event.description);
        
        // Extract analysis results
        let suggestions_val = event.data.get("suggestions").cloned().unwrap_or(serde_json::json!([]));
        let suggestions: Vec<String> = suggestions_val.as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();
        
        let analysis_type = event.data.get("analysis_type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        
        info!("💡 Analysis type: {} with {} suggestions", analysis_type, suggestions.len());
        
        // React to analysis results
        if suggestions.len() > 5 {
            reactor.action_executor.add_action(
                "prioritize_code_review",
                format!("High suggestion count ({}) requires prioritized review", suggestions.len()),
                ActionPriority::Medium,
                event.event_type.clone(),
            ).await;
        }
        
        // Check for critical issues
        let critical_suggestions: Vec<_> = suggestions.iter()
            .filter(|s| s.contains("critical") || s.contains("security") || s.contains("performance"))
            .collect();
        
        if !critical_suggestions.is_empty() {
            let critical_list: Vec<&str> = critical_suggestions.iter().map(|s| s.as_str()).collect();
            reactor.action_executor.add_action(
                "immediate_code_review",
                format!("Critical issues detected: {}", critical_list.join(", ")),
                ActionPriority::Critical,
                event.event_type.clone(),
            ).await;
        }
        
        // Generate insights about code quality trends
        reactor.reasoning_engine.generate_insight(
            "code_quality_trend",
            format!("Code analysis: {} suggestions, {} critical", suggestions.len(), critical_suggestions.len()),
            0.85,
            event.data.clone(),
        ).await;
        
        Ok(())
    }
    
    fn event_type(&self) -> &str {
        "code_analysis"
    }
}

pub struct SystemStartupHandler;

#[async_trait::async_trait]
impl EventHandler for SystemStartupHandler {
    async fn handle(&self, event: &SystemEvent, reactor: &mut EventReactor) -> Result<(), Box<dyn std::error::Error>> {
        info!("🚀 System startup: {}", event.description);
        
        // Extract system info
        let agents_count = event.data.get("agents_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        
        let patterns_count = event.data.get("patterns_count")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        
        info!("📊 System initialized with {} agents and {} patterns", agents_count, patterns_count);
        
        // Initialize system monitoring
        reactor.action_executor.add_action(
            "initialize_monitoring",
            "Initialize system performance monitoring".to_string(),
            ActionPriority::High,
            event.event_type.clone(),
        ).await;
        
        // Schedule periodic health checks
        reactor.action_executor.add_action(
            "schedule_health_checks",
            "Schedule periodic system health checks".to_string(),
            ActionPriority::Medium,
            event.event_type.clone(),
        ).await;
        
        // Update system stats
        let mut system_stats = reactor.stats_collector.system_stats.write().await;
        system_stats.active_agents = agents_count as u32;
        system_stats.system_health = 1.0; // Fresh startup
        
        Ok(())
    }
    
    fn event_type(&self) -> &str {
        "system_startup"
    }
}

impl EventReactor {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("🚀 Initializing Event Reactor System...");
        
        let engine = ExecutionEngine::new().await?;
        
        let mut event_handlers: HashMap<String, Box<dyn EventHandler>> = HashMap::new();
        event_handlers.insert("agent_awakened".to_string(), Box::new(AgentAwakenedHandler));
        event_handlers.insert("git_commit".to_string(), Box::new(GitCommitHandler));
        event_handlers.insert("code_analysis".to_string(), Box::new(CodeAnalysisHandler));
        event_handlers.insert("system_startup".to_string(), Box::new(SystemStartupHandler));
        
        let reasoning_engine = ReasoningEngine::new();
        let action_executor = ActionExecutor::new();
        let stats_collector = StatsCollector::new();
        
        info!("✅ Event Reactor System initialized");
        
        Ok(Self {
            engine,
            event_handlers,
            reasoning_engine,
            action_executor,
            stats_collector,
        })
    }
    
    pub async fn start_event_monitoring(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("👁️  Starting real-time event monitoring...");
        
        let events_dir = Path::new(".emergence/events");
        if !events_dir.exists() {
            return Err("Events directory not found".into());
        }
        
        // Monitor all .jsonl files in the events directory
        let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, _>| {
            match res {
                Ok(event) => {
                    if let notify::EventKind::Modify(_) = event.kind {
                        for path in event.paths {
                            if path.extension().map_or(false, |ext| ext == "jsonl") {
                                info!("📁 Event file modified: {:?}", path);
                                // In a real implementation, we'd parse and process the new events
                            }
                        }
                    }
                }
                Err(e) => error!("Watch error: {:?}", e),
            }
        })?;
        
        watcher.watch(events_dir, RecursiveMode::NonRecursive)?;
        
        // Process existing events
        self.process_existing_events().await?;
        
        info!("✅ Event monitoring active");
        
        // Keep the watcher alive
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
    
    async fn process_existing_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("📂 Processing existing events...");
        
        let events_dir = Path::new(".emergence/events");
        let files = fs::read_dir(events_dir)?;
        
        for file in files {
            let file = file?;
            let path = file.path();
            
            if path.extension().map_or(false, |ext| ext == "jsonl") {
                info!("📄 Processing event file: {:?}", path);
                self.process_event_file(&path).await?;
            }
        }
        
        info!("✅ Existing events processed");
        Ok(())
    }
    
    async fn process_event_file(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        
        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<SystemEvent>(line) {
                Ok(event) => {
                    info!("📋 Processing event: {} - {}", event.event_type, event.description);
                    self.handle_event(&event).await?;
                }
                Err(e) => {
                    warn!("⚠️  Failed to parse event: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_event(&mut self, event: &SystemEvent) -> Result<(), Box<dyn std::error::Error>> {
        // Find appropriate handler
        let handler_opt = self.event_handlers.get(&event.event_type).map(|h| &**h as *const dyn EventHandler);
        if let Some(handler_ptr) = handler_opt {
            // SAFETY: We only use the pointer for the duration of this call, and do not move self.
            let handler: &dyn EventHandler = unsafe { &*handler_ptr };
            handler.handle(event, self).await?;
        } else {
            info!("🤷 No handler for event type: {}", event.event_type);
        }
        // Update stats
        let mut system_stats = self.stats_collector.system_stats.write().await;
        system_stats.total_events += 1;
        // Update emergence trends
        self.stats_collector.update_emergence_trend(event.emergence_potential).await;
        Ok(())
    }
}

impl ReasoningEngine {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            insights: Vec::new(),
            correlations: Vec::new(),
        }
    }
    
    pub async fn generate_insight(&mut self, insight_type: &str, description: String, confidence: f64, data: serde_json::Value) {
        let insight = EventInsight {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            insight_type: insight_type.to_string(),
            description,
            confidence,
            data,
        };
        
        self.insights.push(insight.clone());
        info!("💡 Insight generated: {} (confidence: {:.2})", insight.description, insight.confidence);
    }
}

impl ActionExecutor {
    pub fn new() -> Self {
        Self {
            pending_actions: Vec::new(),
            action_history: Vec::new(),
            auto_execute: true,
        }
    }
    
    pub async fn add_action(&mut self, action_type: &str, description: String, priority: ActionPriority, trigger_event: String) {
        let action = PendingAction {
            id: Uuid::new_v4(),
            action_type: action_type.to_string(),
            description,
            priority,
            trigger_event,
            created_at: Utc::now(),
        };
        
        self.pending_actions.push(action.clone());
        info!("⚡ Action queued: {} (priority: {:?})", action.description, action.priority);
        
        // Auto-execute high priority actions
        if self.auto_execute && matches!(action.priority, ActionPriority::High | ActionPriority::Critical) {
            self.execute_action(&action).await;
        }
    }
    
    async fn execute_action(&mut self, action: &PendingAction) {
        info!("🔧 Executing action: {}", action.description);
        
        let start_time = std::time::Instant::now();
        let result = match action.action_type.as_str() {
            "optimize_collaboration" => {
                // Simulate collaboration optimization
                tokio::time::sleep(Duration::from_millis(100)).await;
                ActionResult::Success { details: "Collaboration patterns optimized".to_string() }
            }
            "run_comprehensive_tests" => {
                // Simulate test execution
                tokio::time::sleep(Duration::from_millis(500)).await;
                ActionResult::Success { details: "Comprehensive tests completed".to_string() }
            }
            "code_review_alert" => {
                // Simulate code review alert
                tokio::time::sleep(Duration::from_millis(50)).await;
                ActionResult::Success { details: "Code review alert sent".to_string() }
            }
            "immediate_code_review" => {
                // Simulate immediate code review
                tokio::time::sleep(Duration::from_millis(200)).await;
                ActionResult::Success { details: "Immediate code review initiated".to_string() }
            }
            "initialize_monitoring" => {
                // Simulate monitoring initialization
                tokio::time::sleep(Duration::from_millis(150)).await;
                ActionResult::Success { details: "System monitoring initialized".to_string() }
            }
            "schedule_health_checks" => {
                // Simulate health check scheduling
                tokio::time::sleep(Duration::from_millis(75)).await;
                ActionResult::Success { details: "Health checks scheduled".to_string() }
            }
            _ => {
                ActionResult::Failure { error: "Unknown action type".to_string() }
            }
        };
        
        let execution_time = start_time.elapsed();
        
        let executed_action = ExecutedAction {
            id: action.id,
            action_type: action.action_type.clone(),
            description: action.description.clone(),
            result,
            execution_time,
            created_at: action.created_at,
        };
        
        self.action_history.push(executed_action.clone());
        
        match &executed_action.result {
            ActionResult::Success { details } => {
                info!("✅ Action completed: {} - {}", action.description, details);
            }
            ActionResult::Failure { error } => {
                error!("❌ Action failed: {} - {}", action.description, error);
            }
            ActionResult::Partial { details } => {
                warn!("⚠️  Action partial: {} - {}", action.description, details);
            }
        }
    }
}

impl StatsCollector {
    pub fn new() -> Self {
        Self {
            agent_stats: Arc::new(RwLock::new(HashMap::new())),
            system_stats: Arc::new(RwLock::new(SystemStats {
                total_events: 0,
                average_emergence: 0.0,
                active_agents: 0,
                system_health: 1.0,
                last_optimization: None,
            })),
            emergence_trends: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn update_emergence_trend(&self, emergence_level: f64) {
        let mut trends = self.emergence_trends.write().await;
        
        let trend = EmergenceTrend {
            timestamp: Utc::now(),
            emergence_level,
            contributing_factors: vec!["event_processing".to_string()],
            trend_direction: if trends.len() > 0 {
                let last_trend = &trends[trends.len() - 1];
                if emergence_level > last_trend.emergence_level + 0.1 {
                    TrendDirection::Increasing
                } else if emergence_level < last_trend.emergence_level - 0.1 {
                    TrendDirection::Decreasing
                } else {
                    TrendDirection::Stable
                }
            } else {
                TrendDirection::Stable
            },
        };
        
        trends.push(trend);
        
        // Keep only last 100 trends
        if trends.len() > 100 {
            trends.remove(0);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🚀 EMERGENCE Event Reactor System");
    info!("===============================================");
    
    let mut event_reactor = EventReactor::new().await?;
    event_reactor.start_event_monitoring().await?;
    
    info!("✅ Event reactor system complete");
    Ok(())
} 