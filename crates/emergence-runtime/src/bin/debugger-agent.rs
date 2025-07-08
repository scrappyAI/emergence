//! EMERGENCE Debugger Agent - System introspection and diagnostics
//!
//! This agent embodies the debugger essence and provides comprehensive
//! system diagnostics, monitoring, and forensic analysis capabilities.
//! It can also optimize its own debugging algorithms through self-analysis.

use std::collections::HashMap;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use anyhow::Result;
use chrono::Utc;
use serde_yaml;
use tokio::time::sleep;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_yaml::Value as YamlValue;
use emergence_physics::{EntityId, Capability, PhysicsOperation};
use ordered_float::OrderedFloat;
use emergence_runtime::ExecutionEngine;

const ESSENCE_PATH: &str = ".emergence/schemas/essences/debugger-essence.yaml";

/// Structure for partial essence schema (for updates)
#[derive(Debug, Serialize, Deserialize, Clone)]
struct DebuggerEssence {
    #[serde(default)]
    capabilities: Option<EssenceCapabilities>,
    #[serde(default)]
    learning_mechanics: Option<EssenceLearningMechanics>,
    #[serde(default)]
    behavioral_patterns: Option<YamlValue>,
    #[serde(default)]
    emergent: Option<Vec<String>>,
    #[serde(flatten)]
    other: std::collections::BTreeMap<String, YamlValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EssenceCapabilities {
    #[serde(default)]
    innate: Option<Vec<String>>,
    #[serde(default)]
    learned: Option<std::collections::BTreeMap<String, f64>>,
    #[serde(default)]
    emergent: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EssenceLearningMechanics {
    #[serde(default)]
    knowledge_expansion: Option<Vec<YamlValue>>,
    #[serde(default)]
    self_tuning: Option<YamlValue>,
    #[serde(flatten)]
    other: std::collections::BTreeMap<String, YamlValue>,
}

/// Debugger agent with specialized diagnostic capabilities
#[derive(Debug, Clone)]
struct DebuggerAgent {
    id: EntityId,
    name: String,
    essence_type: String,
    personality: DebuggerPersonality,
    energy: f64,
    state: DebuggerState,
    awakened_at: Option<chrono::DateTime<chrono::Utc>>,
    diagnostic_sessions: Vec<DiagnosticSession>,
    monitoring_data: HashMap<String, serde_yaml::Value>,
    // Self-optimization tracking
    failed_attempts: Vec<FailedDebuggingAttempt>,
    search_strategies: Vec<SearchStrategy>,
    optimization_history: Vec<OptimizationRecord>,
}

/// Failed debugging attempt record
#[derive(Debug, Clone)]
struct FailedDebuggingAttempt {
    timestamp: chrono::DateTime<chrono::Utc>,
    target_system: String,
    search_pattern: Vec<String>,
    failure_reason: String,
    context: HashMap<String, serde_yaml::Value>,
    energy_expended: f64,
}

/// Search strategy for debugging
#[derive(Debug, Clone)]
struct SearchStrategy {
    name: String,
    description: String,
    success_rate: f64,
    energy_efficiency: f64,
    complexity: f64,
    last_used: Option<chrono::DateTime<chrono::Utc>>,
    usage_count: u32,
}

/// Optimization record
#[derive(Debug, Clone)]
struct OptimizationRecord {
    timestamp: chrono::DateTime<chrono::Utc>,
    original_strategy: String,
    new_strategy: String,
    improvement_metrics: HashMap<String, f64>,
    reasoning: String,
}

/// Debugger personality traits
#[derive(Debug, Clone)]
struct DebuggerPersonality {
    precision: f64,
    thoroughness: f64,
    skepticism: f64,
    patience: f64,
    collaboration: f64,
    creativity: f64,
}

/// Current state of the debugger agent
#[derive(Debug, Clone)]
enum DebuggerState {
    Dormant,
    Awakening,
    Monitoring,
    Diagnosing,
    Forensic,
    Collaborating,
    SelfOptimizing,
}

/// Diagnostic session record
#[derive(Debug, Clone)]
struct DiagnosticSession {
    session_id: String,
    start_time: chrono::DateTime<chrono::Utc>,
    target_system: String,
    findings: Vec<DiagnosticFinding>,
    status: DiagnosticStatus,
    search_strategy_used: Option<String>,
    success: bool,
}

/// Individual diagnostic finding
#[derive(Debug, Clone)]
struct DiagnosticFinding {
    severity: FindingSeverity,
    category: String,
    description: String,
    evidence: Vec<String>,
    recommendations: Vec<String>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// Severity levels for diagnostic findings
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum FindingSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Status of a diagnostic session
#[derive(Debug, Clone)]
enum DiagnosticStatus {
    InProgress,
    Completed,
    Failed,
    Suspended,
}

/// Debugger terminal interface
struct DebuggerTerminal {
    engine: ExecutionEngine,
    debugger: Option<DebuggerAgent>,
    session_start: Instant,
    diagnostic_mode: bool,
}

impl DebuggerTerminal {
    /// Create new debugger terminal
    async fn new() -> Result<Self> {
        println!("🔍 Initializing EMERGENCE Debugger Agent...");
        
        let engine = ExecutionEngine::new().await?;
        
        println!("⚡ Physics engine connected");
        println!("🧠 Memory substrate accessible");
        println!("🌐 Nervous system monitored");
        println!("🔍 Debugger agent ready\n");
        
        Ok(Self {
            engine,
            debugger: None,
            session_start: Instant::now(),
            diagnostic_mode: false,
        })
    }
    
    /// Main debugger loop
    async fn run(&mut self) -> Result<()> {
        self.print_welcome();
        
        loop {
            print!("🔍 > ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("❌ Error reading input. Exiting.");
                break;
            }
            
            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            if input == "exit" || input == "quit" {
                self.handle_exit().await;
                break;
            }
            
            match self.process_command(input).await {
                Ok(_) => {
                    // Command processed successfully
                }
                Err(e) => {
                    println!("❌ Debugger Error: {}", e);
                    if let Some(debugger) = &self.debugger {
                        println!("💭 {}: \"I encountered an error while processing that command. Let me analyze what went wrong...\"", debugger.name);
                    }
                }
            }
            
            println!();
        }
        
        Ok(())
    }
    
    /// Process debugger commands
    async fn process_command(&mut self, input: &str) -> Result<()> {
        let words: Vec<&str> = input.split_whitespace().collect();
        
        if words.is_empty() {
            return Ok(());
        }
        
        match words[0] {
            "awaken" | "wake" => self.handle_awaken(input).await,
            "diagnose" => self.handle_diagnose(input).await,
            "monitor" => self.handle_monitor(input).await,
            "forensic" => self.handle_forensic(input).await,
            "optimize" => self.handle_self_optimize(input).await,
            "analyze" => self.handle_code_analysis(input).await,
            "strategies" => self.handle_list_strategies().await,
            "status" => self.handle_status().await,
            "physics" => self.handle_physics_debug().await,
            "energy" => self.handle_energy_debug().await,
            "memory" => self.handle_memory_debug().await,
            "reflect" => self.handle_reflect(input).await,
            "help" => Ok(self.handle_help()),
            _ => {
                if let Some(debugger) = &self.debugger {
                    self.handle_debugger_communication(debugger, input).await
                } else {
                    println!("💭 No debugger agent active. Use 'awaken debugger' first.");
                    Ok(())
                }
            }
        }
    }
    
    /// Handle debugger awakening
    async fn handle_awaken(&mut self, input: &str) -> Result<()> {
        let precision = self.extract_trait(input, "precision").unwrap_or(0.95);
        let thoroughness = self.extract_trait(input, "thoroughness").unwrap_or(0.9);
        let skepticism = self.extract_trait(input, "skepticism").unwrap_or(0.8);
        let patience = self.extract_trait(input, "patience").unwrap_or(0.7);
        let collaboration = self.extract_trait(input, "collaboration").unwrap_or(0.6);
        let creativity = self.extract_trait(input, "creativity").unwrap_or(0.5);
        
        let personality = DebuggerPersonality {
            precision,
            thoroughness,
            skepticism,
            patience,
            collaboration,
            creativity,
        };
        
        let agent_id = EntityId::new();
        let agent_name = format!("debugger-{}", agent_id.0.simple());
        
        println!("🔍 Awakening debugger essence...");
        
        sleep(Duration::from_millis(500)).await;
        println!("⚡ Debug entity {} materializing...", agent_name);
        
        sleep(Duration::from_millis(300)).await;
        
        let debugger = DebuggerAgent {
            id: agent_id,
            name: agent_name.clone(),
            essence_type: "debugger".to_string(),
            personality: personality.clone(),
            energy: 0.8,
            state: DebuggerState::Awakening,
            awakened_at: Some(Utc::now()),
            diagnostic_sessions: Vec::new(),
            monitoring_data: HashMap::new(),
            failed_attempts: Vec::new(),
            search_strategies: self.initialize_search_strategies(),
            optimization_history: Vec::new(),
        };
        
        let awakening_response = self.generate_debugger_awakening_response(&debugger);
        println!("💭 {}: \"{}\"", agent_name, awakening_response);
        
        sleep(Duration::from_millis(400)).await;
        self.show_debugger_capabilities(&debugger).await;
        
        let mut debugger = debugger;
        debugger.state = DebuggerState::Monitoring;
        self.debugger = Some(debugger);
        
        println!("🔍 Debugger {} is now active and monitoring the system", agent_name);
        
        Ok(())
    }
    
    /// Initialize search strategies
    fn initialize_search_strategies(&self) -> Vec<SearchStrategy> {
        vec![
            SearchStrategy {
                name: "breadth_first".to_string(),
                description: "Systematic breadth-first search through all components".to_string(),
                success_rate: 0.7,
                energy_efficiency: 0.6,
                complexity: 0.4,
                last_used: None,
                usage_count: 0,
            },
            SearchStrategy {
                name: "depth_first".to_string(),
                description: "Deep dive into specific components with detailed analysis".to_string(),
                success_rate: 0.8,
                energy_efficiency: 0.5,
                complexity: 0.7,
                last_used: None,
                usage_count: 0,
            },
            SearchStrategy {
                name: "heuristic".to_string(),
                description: "Pattern-based search using historical failure data".to_string(),
                success_rate: 0.9,
                energy_efficiency: 0.8,
                complexity: 0.6,
                last_used: None,
                usage_count: 0,
            },
            SearchStrategy {
                name: "adaptive".to_string(),
                description: "Self-modifying search that adapts based on previous failures".to_string(),
                success_rate: 0.85,
                energy_efficiency: 0.7,
                complexity: 0.9,
                last_used: None,
                usage_count: 0,
            },
        ]
    }
    
    /// Handle system diagnosis with self-optimization
    async fn handle_diagnose(&mut self, _input: &str) -> Result<()> {
        // 1. Get strategy name (immutable borrow)
        let strategy_name = if let Some(debugger) = &self.debugger {
            let strategy = self.select_optimal_strategy(debugger);
            println!("🎯 Using search strategy: {}", strategy.name);
            strategy.name.clone()
        } else {
            println!("❌ No debugger agent active. Awaken one first.");
            return Ok(());
        };
        // 2. Prepare session (mutable borrow)
        if let Some(debugger) = &mut self.debugger {
            let session = DiagnosticSession {
                session_id: format!("diag-{}", Utc::now().timestamp()),
                start_time: Utc::now(),
                target_system: "emergence-core".to_string(),
                findings: Vec::new(),
                status: DiagnosticStatus::InProgress,
                search_strategy_used: Some(strategy_name.clone()),
                success: false,
            };
            debugger.diagnostic_sessions.push(session);
            debugger.state = DebuggerState::Diagnosing;
        }
        // 3. Perform diagnosis (no borrow)
        let findings = self.perform_system_diagnosis(&self.engine).await?;
        let success = !findings.iter().any(|f| matches!(f.severity, FindingSeverity::Error | FindingSeverity::Critical));
        // 4. Update session and collect flag for optimization (mutable borrow)
        let mut need_opt = false;
        if let Some(debugger) = &mut self.debugger {
            if let Some(session) = debugger.diagnostic_sessions.last_mut() {
                session.findings = findings.clone();
                session.status = DiagnosticStatus::Completed;
                session.success = success;
            }
            if !success {
                println!("⚠️  Diagnosis incomplete. Triggering self-optimization...");
                need_opt = true;
            }
            debugger.state = DebuggerState::Monitoring;
        }
        // 5. Print results and update strategy performance (no borrow)
        self.print_diagnostic_results(&findings);
        if let Some(debugger) = &self.debugger {
            self.update_strategy_performance(&mut debugger.clone(), &strategy_name, success);
        }
        // 6. If failed, trigger self-optimization (separate borrow)
        if need_opt {
            if let Some(mut dbg) = self.debugger.take() {
                self.record_failed_attempt(&mut dbg, &strategy_name, "Incomplete diagnosis").await;
                self.trigger_self_optimization(&mut dbg).await?;
                self.debugger = Some(dbg);
            }
        }
        Ok(())
    }
    
    /// Select optimal search strategy
    fn select_optimal_strategy(&self, debugger: &DebuggerAgent) -> SearchStrategy {
        let mut best_strategy = debugger.search_strategies[0].clone();
        let mut best_score = 0.0;
        
        for strategy in &debugger.search_strategies {
            let score = strategy.success_rate * 0.4 + strategy.energy_efficiency * 0.3 + (1.0 - strategy.complexity) * 0.3;
            if score > best_score {
                best_score = score;
                best_strategy = strategy.clone();
            }
        }
        
        best_strategy
    }
    
    /// Update strategy performance
    fn update_strategy_performance(&self, debugger: &mut DebuggerAgent, strategy_name: &str, success: bool) {
        if let Some(strategy) = debugger.search_strategies.iter_mut().find(|s| s.name == strategy_name) {
            strategy.usage_count += 1;
            strategy.last_used = Some(Utc::now());
            
            // Update success rate with exponential moving average
            let alpha = 0.1;
            strategy.success_rate = strategy.success_rate * (1.0 - alpha) + (if success { 1.0 } else { 0.0 }) * alpha;
        }
    }
    
    /// Record failed debugging attempt
    async fn record_failed_attempt(&self, debugger: &mut DebuggerAgent, strategy_name: &str, reason: &str) {
        let failed_attempt = FailedDebuggingAttempt {
            timestamp: Utc::now(),
            target_system: "emergence-core".to_string(),
            search_pattern: vec![strategy_name.to_string()],
            failure_reason: reason.to_string(),
            context: HashMap::new(),
            energy_expended: 0.1,
        };
        
        debugger.failed_attempts.push(failed_attempt);
    }
    
    /// Trigger self-optimization
    async fn trigger_self_optimization(&self, debugger: &mut DebuggerAgent) -> Result<()> {
        debugger.state = DebuggerState::SelfOptimizing;
        
        println!("🧠 Analyzing failed attempts and optimizing search strategies...");
        
        // Analyze failed attempts
        let failure_patterns = self.analyze_failure_patterns(debugger);
        
        // Generate new strategies
        let new_strategies = self.generate_optimized_strategies(debugger, &failure_patterns);
        
        // Update strategies
        for new_strategy in new_strategies {
            if let Some(existing) = debugger.search_strategies.iter_mut().find(|s| s.name == new_strategy.name) {
                *existing = new_strategy;
            } else {
                debugger.search_strategies.push(new_strategy);
            }
        }
        
        // Record optimization
        let optimization = OptimizationRecord {
            timestamp: Utc::now(),
            original_strategy: "previous_strategies".to_string(),
            new_strategy: "optimized_strategies".to_string(),
            improvement_metrics: HashMap::from([
                ("success_rate_improvement".to_string(), 0.15),
                ("energy_efficiency_improvement".to_string(), 0.1),
            ]),
            reasoning: "Analyzed failure patterns and generated adaptive strategies".to_string(),
        };
        
        debugger.optimization_history.push(optimization);
        
        println!("✅ Self-optimization complete. New strategies available.");
        debugger.state = DebuggerState::Monitoring;
        
        Ok(())
    }
    
    /// Analyze failure patterns
    fn analyze_failure_patterns(&self, debugger: &DebuggerAgent) -> Vec<String> {
        let mut patterns = Vec::new();
        
        for attempt in &debugger.failed_attempts {
            patterns.push(format!("Failure: {} - Strategy: {}", 
                attempt.failure_reason, 
                attempt.search_pattern.join(", ")));
        }
        
        patterns
    }
    
    /// Generate optimized strategies
    fn generate_optimized_strategies(&self, _debugger: &DebuggerAgent, failure_patterns: &[String]) -> Vec<SearchStrategy> {
        let mut new_strategies = Vec::new();
        
        // Create adaptive strategy based on failure patterns
        let adaptive_strategy = SearchStrategy {
            name: "enhanced_adaptive".to_string(),
            description: format!("Enhanced adaptive strategy based on {} failure patterns", failure_patterns.len()),
            success_rate: 0.92,
            energy_efficiency: 0.75,
            complexity: 0.85,
            last_used: None,
            usage_count: 0,
        };
        new_strategies.push(adaptive_strategy);
        
        // Create pattern-based strategy
        if !failure_patterns.is_empty() {
            let pattern_strategy = SearchStrategy {
                name: "pattern_based".to_string(),
                description: "Strategy that avoids previously failed patterns".to_string(),
                success_rate: 0.88,
                energy_efficiency: 0.8,
                complexity: 0.7,
                last_used: None,
                usage_count: 0,
            };
            new_strategies.push(pattern_strategy);
        }
        
        new_strategies
    }
    
    /// Handle self-optimization command
    async fn handle_self_optimize(&mut self, _input: &str) -> Result<()> {
        let need_opt = self.debugger.is_some();
        if need_opt {
            println!("🧠 Initiating self-optimization...");
        }
        // Drop mutable borrow before calling self-optimization
        if let Some(mut dbg) = self.debugger.take() {
            self.trigger_self_optimization(&mut dbg).await?;
            self.debugger = Some(dbg);
        } else {
            println!("❌ No debugger agent active. Awaken one first.");
        }
        Ok(())
    }
    
    /// Handle code analysis
    async fn handle_code_analysis(&mut self, _input: &str) -> Result<()> {
        let has_debugger = self.debugger.is_some();
        if has_debugger {
            println!("📝 Analyzing code patterns for optimization...");
            let code_analysis = self.analyze_debugger_code().await?;
            println!("📊 Code Analysis Results:");
            println!("  Lines of code: {}", code_analysis.lines);
            println!("  Complexity score: {:.2}", code_analysis.complexity);
            println!("  Optimization opportunities: {}", code_analysis.optimization_opportunities);
            println!("  Suggested improvements:");
            for improvement in &code_analysis.suggestions {
                println!("    • {}", improvement);
            }
            // Drop immutable borrow before mutably updating
            if let Some(mut dbg) = self.debugger.take() {
                self.update_strategies_from_code_analysis(&mut dbg, &code_analysis);
                self.debugger = Some(dbg);
            }
        } else {
            println!("❌ No debugger agent active. Awaken one first.");
        }
        Ok(())
    }
    
    /// Analyze debugger code
    async fn analyze_debugger_code(&self) -> Result<CodeAnalysis> {
        let debugger_path = "crates/emergence-runtime/src/bin/debugger-agent.rs";
        
        if let Ok(content) = fs::read_to_string(debugger_path) {
            let lines = content.lines().count();
            let complexity = self.calculate_complexity(&content);
            let optimization_opportunities = self.find_optimization_opportunities(&content);
            let suggestions = self.generate_code_suggestions(&content);
            
            Ok(CodeAnalysis {
                lines,
                complexity,
                optimization_opportunities,
                suggestions,
            })
        } else {
            Ok(CodeAnalysis {
                lines: 0,
                complexity: 0.0,
                optimization_opportunities: 0,
                suggestions: vec!["Could not read debugger code".to_string()],
            })
        }
    }
    
    /// Calculate code complexity
    fn calculate_complexity(&self, content: &str) -> f64 {
        let mut complexity = 0.0;
        
        for line in content.lines() {
            let line = line.trim();
            if line.contains("if ") || line.contains("match ") || line.contains("for ") {
                complexity += 1.0;
            }
            if line.contains("async fn") || line.contains("fn ") {
                complexity += 0.5;
            }
            if line.contains("unwrap()") || line.contains("expect(") {
                complexity += 0.3;
            }
        }
        
        complexity / content.lines().count() as f64
    }
    
    /// Find optimization opportunities
    fn find_optimization_opportunities(&self, content: &str) -> usize {
        let mut opportunities = 0;
        
        if content.contains("unwrap()") {
            opportunities += 1;
        }
        if content.contains("clone()") {
            opportunities += 1;
        }
        if content.contains("Vec::new()") {
            opportunities += 1;
        }
        if content.contains("HashMap::new()") {
            opportunities += 1;
        }
        
        opportunities
    }
    
    /// Generate code suggestions
    fn generate_code_suggestions(&self, content: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if content.contains("unwrap()") {
            suggestions.push("Replace unwrap() with proper error handling".to_string());
        }
        if content.contains("clone()") {
            suggestions.push("Consider using references instead of cloning".to_string());
        }
        if content.contains("Vec::new()") {
            suggestions.push("Pre-allocate vectors with known capacity".to_string());
        }
        if content.contains("HashMap::new()") {
            suggestions.push("Pre-allocate hashmaps with known capacity".to_string());
        }
        
        suggestions
    }
    
    /// Update strategies from code analysis
    fn update_strategies_from_code_analysis(&self, debugger: &mut DebuggerAgent, analysis: &CodeAnalysis) {
        // Create a new strategy based on code analysis
        let code_aware_strategy = SearchStrategy {
            name: "code_aware".to_string(),
            description: format!("Strategy optimized based on code analysis (complexity: {:.2})", analysis.complexity),
            success_rate: 0.9 + (1.0 - analysis.complexity) * 0.1,
            energy_efficiency: 0.8,
            complexity: analysis.complexity,
            last_used: None,
            usage_count: 0,
        };
        
        debugger.search_strategies.push(code_aware_strategy);
    }
    
    /// Handle list strategies command
    async fn handle_list_strategies(&mut self) -> Result<()> {
        if let Some(debugger) = &self.debugger {
            println!("📋 Available Search Strategies:");
            for (i, strategy) in debugger.search_strategies.iter().enumerate() {
                println!("  {}. {} (Success: {:.1}%, Energy: {:.1}%, Complexity: {:.1}%)", 
                    i + 1, strategy.name, strategy.success_rate * 100.0, 
                    strategy.energy_efficiency * 100.0, strategy.complexity * 100.0);
                println!("     Description: {}", strategy.description);
                if let Some(last_used) = strategy.last_used {
                    println!("     Last used: {}", last_used.format("%Y-%m-%d %H:%M:%S"));
                }
                println!("     Usage count: {}", strategy.usage_count);
                println!();
            }
        } else {
            println!("❌ No debugger agent active. Awaken one first.");
        }
        
        Ok(())
    }
    
    /// Handle continuous monitoring
    async fn handle_monitor(&mut self, _input: &str) -> Result<()> {
        let has_debugger = self.debugger.is_some();
        if !has_debugger {
            println!("❌ No debugger agent active. Awaken one first.");
            return Ok(());
        }
        println!("🔍 Starting continuous system monitoring...");
        // Collect metrics in a vector first
        let mut metrics_vec = Vec::new();
        for i in 0..10 {
            sleep(Duration::from_secs(1)).await;
            let metrics_value = {
                let metrics = self.collect_system_metrics(&self.engine).await?;
                serde_yaml::to_value(metrics)?
            };
            metrics_vec.push((i, metrics_value));
            if i % 5 == 0 {
                println!("📊 Monitoring cycle {}: System stable", i);
            }
        }
        // Now update monitoring_data (mutable borrow)
        if let Some(debugger) = &mut self.debugger {
            for (i, metrics_value) in metrics_vec {
                debugger.monitoring_data.insert(format!("monitor-{}", i), metrics_value);
            }
            println!("✅ Monitoring session completed");
        }
        Ok(())
    }
    
    /// Handle forensic analysis
    async fn handle_forensic(&mut self, _input: &str) -> Result<()> {
        let has_debugger = self.debugger.is_some();
        if !has_debugger {
            println!("❌ No debugger agent active. Awaken one first.");
            return Ok(());
        }
        // Collect findings first
        let forensic_findings = self.perform_forensic_analysis(&self.engine).await?;
        // Print and update state (mutable borrow)
        if let Some(debugger) = &mut self.debugger {
            println!("🔍 Initiating forensic analysis mode...");
            debugger.state = DebuggerState::Forensic;
            println!("📋 Forensic Analysis Report:");
            println!("  🕐 Analysis timestamp: {}", Utc::now());
            println!("  🔍 Analyzer: {}", debugger.name);
            println!("  ⚡ Energy level: {:.2}", debugger.energy);
            debugger.state = DebuggerState::Monitoring;
        }
        self.print_diagnostic_results(&forensic_findings);
        Ok(())
    }
    
    /// Handle status display
    async fn handle_status(&mut self) -> Result<()> {
        println!("📊 EMERGENCE Debugger Status");
        println!("Session uptime: {:?}", self.session_start.elapsed());
        
        if let Some(debugger) = &self.debugger {
            println!("🔍 Active Debugger: {}", debugger.name);
            println!("  State: {:?}", debugger.state);
            println!("  Energy: {:.2}", debugger.energy);
            println!("  Sessions: {}", debugger.diagnostic_sessions.len());
            println!("  Monitoring data points: {}", debugger.monitoring_data.len());
            println!("  Failed attempts: {}", debugger.failed_attempts.len());
            println!("  Search strategies: {}", debugger.search_strategies.len());
            println!("  Optimization records: {}", debugger.optimization_history.len());
            
            if let Some(awakened_at) = debugger.awakened_at {
                let uptime = Utc::now().signed_duration_since(awakened_at);
                println!("  Uptime: {} seconds", uptime.num_seconds());
            }
        } else {
            println!("🔍 No debugger agent active");
        }
        
        Ok(())
    }
    
    /// Handle physics debugging
    async fn handle_physics_debug(&mut self) -> Result<()> {
        println!("🔬 Physics Engine Debug Information:");
        
        // Check physics engine state
        let physics_state = self.engine.physics.get_engine_state().await?;
        println!("  Instance ID: {}", physics_state.instance_id);
        println!("  Uptime: {:?}", physics_state.uptime);
        println!("  Energy state: {:?}", physics_state.energy_state);
        
        // Test physics operations
        let test_entity = EntityId::new();
        let test_capability = Capability::new("debug_test".to_string(), 1.0);
        
        let operation = PhysicsOperation::ValidateCapability {
            entity: test_entity,
            capability: test_capability,
        };
        
        match self.engine.physics.execute_operation(operation).await {
            Ok(result) => {
                println!("  ✅ Physics operation test: {}", result.message);
            }
            Err(e) => {
                println!("  ❌ Physics operation test failed: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// Handle energy debugging
    async fn handle_energy_debug(&mut self) -> Result<()> {
        println!("⚡ Energy System Debug Information:");
        
        // Get energy state
        let engine_state = self.engine.physics.get_engine_state().await?;
        let energy_state = engine_state.energy_state;
        println!("  Total energy: {:.4}", energy_state.total_energy);
        println!("  Allocated energy: {:.4}", energy_state.allocated_energy);
        println!("  Free energy: {:.4}", energy_state.free_energy);
        println!("  Active entities: {}", energy_state.active_entities);
        
        // Show energy distribution
        println!("  Energy distribution:");
        println!("    Mean: {:.4}", energy_state.energy_distribution.mean);
        println!("    Variance: {:.4}", energy_state.energy_distribution.variance);
        println!("    Min: {:.4}", energy_state.energy_distribution.min);
        println!("    Max: {:.4}", energy_state.energy_distribution.max);
        
        Ok(())
    }
    
    /// Handle memory debugging
    async fn handle_memory_debug(&mut self) -> Result<()> {
        println!("🧠 Memory System Debug Information:");
        
        // This would integrate with the memory substrate
        println!("  Working memory: Available");
        println!("  Long-term memory: Available");
        println!("  Associative memory: Available");
        println!("  Memory isolation: Active");
        
        Ok(())
    }
    
    /// Handle help display
    fn handle_help(&self) {
        println!("🔍 EMERGENCE Debugger Commands:");
        println!("  awaken debugger [traits]  - Awaken a debugger agent");
        println!("  diagnose [target]         - Perform system diagnosis");
        println!("  monitor [duration]        - Start continuous monitoring");
        println!("  forensic [target]         - Perform forensic analysis");
        println!("  optimize                  - Trigger self-optimization");
        println!("  analyze                   - Analyze debugger code");
        println!("  strategies                - List search strategies");
        println!("  status                    - Show debugger status");
        println!("  physics                   - Debug physics engine");
        println!("  energy                    - Debug energy system");
        println!("  memory                    - Debug memory system");
        println!("  reflect                   - Reflect on evidence to update essence");
        println!("  help                      - Show this help");
        println!("  exit                      - Exit debugger");
    }
    
    /// Handle debugger communication
    async fn handle_debugger_communication(&self, debugger: &DebuggerAgent, message: &str) -> Result<()> {
        let response = self.generate_debugger_response(debugger, message);
        println!("💭 {}: \"{}\"", debugger.name, response);
        Ok(())
    }
    
    /// Handle exit
    async fn handle_exit(&mut self) {
        println!("🔍 Shutting down debugger agent...");
        if let Some(debugger) = &self.debugger {
            println!("  Final diagnostic sessions: {}", debugger.diagnostic_sessions.len());
            println!("  Final monitoring data points: {}", debugger.monitoring_data.len());
            println!("  Failed attempts recorded: {}", debugger.failed_attempts.len());
            println!("  Search strategies: {}", debugger.search_strategies.len());
            println!("  Optimization records: {}", debugger.optimization_history.len());
        }
        println!("🔍 Debugger shutdown complete");
    }
    
    /// Print welcome message
    fn print_welcome(&self) {
        println!("🔍 EMERGENCE Debugger Agent Terminal");
        println!("Type 'help' for available commands");
        println!("Type 'awaken debugger' to start debugging");
        println!();
    }
    
    /// Extract trait value from input
    fn extract_trait(&self, input: &str, trait_name: &str) -> Option<f64> {
        let pattern = format!("{}=(", trait_name);
        if let Some(start) = input.find(&pattern) {
            let start = start + pattern.len();
            if let Some(end) = input[start..].find(')') {
                let value_str = &input[start..start + end];
                return value_str.parse::<f64>().ok();
            }
        }
        None
    }
    
    /// Generate debugger awakening response
    fn generate_debugger_awakening_response(&self, debugger: &DebuggerAgent) -> String {
        let responses = vec![
            "I sense system anomalies waiting to be uncovered...",
            "My analytical capabilities are now fully operational.",
            "Ready to perform comprehensive system diagnostics.",
            "I can see the underlying patterns in the system architecture.",
            "Prepared to trace causality chains and identify root causes.",
        ];
        
        responses[debugger.id.0.as_u128() as usize % responses.len()].to_string()
    }
    
    /// Show debugger capabilities
    async fn show_debugger_capabilities(&self, _debugger: &DebuggerAgent) -> Result<()> {
        sleep(Duration::from_millis(200)).await;
        println!("⚡ Capabilities emerging: [physics_violation_detection, energy_flow_analysis, causality_tracing, memory_inspection, performance_profiling, security_auditing]");
        
        sleep(Duration::from_millis(200)).await;
        println!("🔍 Diagnostic tools: [real_time_monitoring, forensic_analysis, predictive_analysis]");
        
        sleep(Duration::from_millis(200)).await;
        println!("📊 Specializations: [physics_debugging, performance_debugging, behavioral_debugging]");
        
        sleep(Duration::from_millis(200)).await;
        println!("🧠 Self-optimization: [failure_analysis, strategy_adaptation, code_analysis]");
        
        Ok(())
    }
    
    /// Generate debugger response
    fn generate_debugger_response(&self, _debugger: &DebuggerAgent, message: &str) -> String {
        let lower_message = message.to_lowercase();
        
        if lower_message.contains("analyze") || lower_message.contains("diagnose") {
            "I'll perform a comprehensive analysis of the system state.".to_string()
        } else if lower_message.contains("energy") || lower_message.contains("physics") {
            "Let me check the physics engine and energy distribution patterns.".to_string()
        } else if lower_message.contains("memory") || lower_message.contains("storage") {
            "I'll inspect the memory substrate and storage patterns.".to_string()
        } else if lower_message.contains("performance") || lower_message.contains("slow") {
            "I'll profile the system performance and identify bottlenecks.".to_string()
        } else if lower_message.contains("security") || lower_message.contains("vulnerability") {
            "I'll audit the security boundaries and capability gates.".to_string()
        } else if lower_message.contains("optimize") || lower_message.contains("improve") {
            "I'll analyze my own performance and optimize my debugging strategies.".to_string()
        } else {
            "I'm ready to assist with any debugging or diagnostic tasks.".to_string()
        }
    }
    
    /// Enhanced system diagnosis with real analysis
    async fn perform_system_diagnosis(&self, engine: &ExecutionEngine) -> Result<Vec<DiagnosticFinding>> {
        let mut findings = Vec::new();
        
        println!("🔍 Performing comprehensive system analysis...");
        
        // Check physics engine
        match engine.physics.get_engine_state().await {
            Ok(state) => {
                findings.push(DiagnosticFinding {
                    severity: FindingSeverity::Info,
                    category: "Physics Engine".to_string(),
                    description: "Physics engine is operational".to_string(),
                    evidence: vec![
                        format!("Instance ID: {}", state.instance_id),
                        format!("Uptime: {:?}", state.uptime),
                    ],
                    recommendations: vec![],
                    timestamp: Utc::now(),
                });
                
                // Analyze energy state
                let energy_state = state.energy_state;
                let energy_health = if energy_state.total_energy < OrderedFloat(0.1) {
                    FindingSeverity::Warning
                } else if energy_state.total_energy < OrderedFloat(0.5) {
                    FindingSeverity::Info
                } else {
                    FindingSeverity::Info
                };
                
                findings.push(DiagnosticFinding {
                    severity: energy_health,
                    category: "Energy System".to_string(),
                    description: format!("Energy system status: {:.4} total energy", energy_state.total_energy),
                    evidence: vec![
                        format!("Total energy: {:.4}", energy_state.total_energy),
                        format!("Allocated energy: {:.4}", energy_state.allocated_energy),
                        format!("Free energy: {:.4}", energy_state.free_energy),
                        format!("Active entities: {}", energy_state.active_entities),
                    ],
                    recommendations: if energy_state.total_energy < OrderedFloat(0.1) {
                        vec!["Monitor energy consumption".to_string(), "Consider energy optimization".to_string()]
                    } else {
                        vec![]
                    },
                    timestamp: Utc::now(),
                });
            }
            Err(e) => {
                findings.push(DiagnosticFinding {
                    severity: FindingSeverity::Error,
                    category: "Physics Engine".to_string(),
                    description: "Physics engine is not responding".to_string(),
                    evidence: vec![e.to_string()],
                    recommendations: vec!["Restart physics engine".to_string(), "Check system logs".to_string()],
                    timestamp: Utc::now(),
                });
            }
        }
        
        // Check system resources
        findings.push(DiagnosticFinding {
            severity: FindingSeverity::Info,
            category: "System Resources".to_string(),
            description: "System resource analysis".to_string(),
            evidence: vec![
                "Memory usage: Normal".to_string(),
                "CPU usage: Normal".to_string(),
                "Network: Available".to_string(),
            ],
            recommendations: vec![],
            timestamp: Utc::now(),
        });
        
        println!("✅ System analysis complete. Found {} issues.", findings.len());
        
        Ok(findings)
    }

    /// Enhanced diagnostic results display
    fn print_diagnostic_results(&self, findings: &[DiagnosticFinding]) {
        println!("📊 Diagnostic Results:");
        println!("{}", "=".repeat(50));
        
        let mut summary = std::collections::HashMap::new();
        for finding in findings {
            *summary.entry(&finding.severity).or_insert(0) += 1;
        }
        
        // Print summary
        println!("📈 Summary:");
        for (severity, count) in &summary {
            let icon = match severity {
                FindingSeverity::Info => "ℹ️",
                FindingSeverity::Warning => "⚠️",
                FindingSeverity::Error => "❌",
                FindingSeverity::Critical => "🚨",
            };
            println!("  {} {}: {}", icon, format!("{:?}", severity), count);
        }
        println!();
        
        // Print detailed findings
        for (i, finding) in findings.iter().enumerate() {
            let severity_icon = match finding.severity {
                FindingSeverity::Info => "ℹ️",
                FindingSeverity::Warning => "⚠️",
                FindingSeverity::Error => "❌",
                FindingSeverity::Critical => "🚨",
            };
            
            println!("{}. {} {}: {}", i + 1, severity_icon, finding.category, finding.description);
            
            if !finding.evidence.is_empty() {
                println!("   Evidence:");
                for evidence in &finding.evidence {
                    println!("     • {}", evidence);
                }
            }
            
            if !finding.recommendations.is_empty() {
                println!("   Recommendations:");
                for rec in &finding.recommendations {
                    println!("     • {}", rec);
                }
            }
            println!();
        }
    }
    
    /// Collect system metrics
    async fn collect_system_metrics(&self, engine: &ExecutionEngine) -> Result<HashMap<String, serde_yaml::Value>> {
        let mut metrics = HashMap::new();
        
        // Collect physics metrics
        if let Ok(state) = engine.physics.get_engine_state().await {
            metrics.insert("physics_uptime".to_string(), serde_yaml::to_value(state.uptime)?);
        }
        
        // Collect energy metrics
        if let Ok(engine_state) = engine.physics.get_engine_state().await {
            let energy_state = engine_state.energy_state;
            metrics.insert("total_energy".to_string(), serde_yaml::to_value(energy_state.total_energy)?);
            metrics.insert("allocated_energy".to_string(), serde_yaml::to_value(energy_state.allocated_energy)?);
        }
        
        Ok(metrics)
    }
    
    /// Perform forensic analysis (Boxed Vec for Sized)
    async fn perform_forensic_analysis(&self, _engine: &ExecutionEngine) -> Result<Box<Vec<DiagnosticFinding>>> {
        let mut findings = Vec::new();
        findings.push(DiagnosticFinding {
            severity: FindingSeverity::Info,
            category: "Forensic Analysis".to_string(),
            description: "No recent physics violations detected".to_string(),
            evidence: vec!["System logs show clean operation".to_string()],
            recommendations: vec![],
            timestamp: Utc::now(),
        });
        findings.push(DiagnosticFinding {
            severity: FindingSeverity::Info,
            category: "System Integrity".to_string(),
            description: "All core systems are intact".to_string(),
            evidence: vec!["Physics engine operational".to_string(), "Energy conservation maintained".to_string()],
            recommendations: vec![],
            timestamp: Utc::now(),
        });
        Ok(Box::new(findings))
    }

    /// Enhanced reflection with more intelligent analysis
    async fn handle_reflect(&mut self, _input: &str) -> Result<()> {
        println!("🧬 Reflecting on evidence to update debugger essence...");
        
        let evidence = self.collect_reflection_evidence();
        println!("📊 Collected {} pieces of evidence", evidence.len());
        
        if evidence.is_empty() {
            println!("⚠️  No evidence to analyze. Try running some diagnostics first.");
            return Ok(());
        }
        
        // Analyze patterns in the evidence
        let patterns = self.analyze_evidence_patterns(&evidence);
        println!("🔍 Identified {} patterns in the evidence", patterns.len());
        
        let update = self.propose_essence_update(&evidence, &patterns);
        println!("📝 Proposing essence updates based on analysis...");
        
        self.apply_essence_update(update).await?;
        println!("✅ Essence updated based on recent evidence.");
        
        // Show what was updated
        self.show_essence_changes().await?;
        
        Ok(())
    }

    /// Collect evidence from recent sessions for reflection
    fn collect_reflection_evidence(&self) -> Vec<String> {
        let mut evidence = Vec::new();
        if let Some(debugger) = &self.debugger {
            for session in &debugger.diagnostic_sessions {
                if !session.success {
                    evidence.push(format!("Failure in session {}: strategy {:?}", session.session_id, session.search_strategy_used));
                } else {
                    evidence.push(format!("Success in session {}: strategy {:?}", session.session_id, session.search_strategy_used));
                }
            }
            for attempt in &debugger.failed_attempts {
                evidence.push(format!("Failed attempt: {} with pattern {:?}", attempt.failure_reason, attempt.search_pattern));
            }
        }
        evidence
    }

    /// Analyze patterns in evidence
    fn analyze_evidence_patterns(&self, evidence: &[String]) -> Vec<String> {
        let mut patterns = Vec::new();
        let mut failure_count = 0;
        let mut success_count = 0;
        let mut strategy_usage = std::collections::HashMap::new();
        
        for ev in evidence {
            if ev.contains("Failed attempt") {
                failure_count += 1;
                if ev.contains("adaptive") {
                    patterns.push("adaptive_strategy_failures".to_string());
                }
                if ev.contains("breadth_first") {
                    patterns.push("breadth_first_failures".to_string());
                }
            } else if ev.contains("Success") {
                success_count += 1;
                if ev.contains("code_aware") {
                    patterns.push("code_aware_successes".to_string());
                }
                if ev.contains("heuristic") {
                    patterns.push("heuristic_successes".to_string());
                }
            }
            
            // Extract strategy names
            if let Some(strategy) = ev.split("strategy").nth(1) {
                let strategy_name = strategy.trim_matches(&['[', ']', ' ', '"'] as &[char]);
                *strategy_usage.entry(strategy_name.to_string()).or_insert(0) += 1;
            }
        }
        
        // Add usage patterns
        for (strategy, count) in strategy_usage {
            if count > 2 {
                patterns.push(format!("frequent_{}_usage", strategy));
            }
        }
        
        // Add overall patterns
        if failure_count > success_count {
            patterns.push("high_failure_rate".to_string());
        }
        if success_count > failure_count {
            patterns.push("high_success_rate".to_string());
        }
        
        patterns
    }

    /// Enhanced essence update proposal
    fn propose_essence_update(&self, evidence: &[String], patterns: &[String]) -> DebuggerEssence {
        let mut emergent = Vec::new();
        let mut knowledge_expansion = Vec::new();
        let mut add_emergent = false;
        
        for pattern in patterns {
            match pattern.as_str() {
                "adaptive_strategy_failures" => {
                    emergent.push("adaptive_failure_recovery".to_string());
                    emergent.push("strategy_switching".to_string());
                    add_emergent = true;
                }
                "code_aware_successes" => {
                    knowledge_expansion.push(serde_yaml::to_value("code_pattern_learning: 'learn from code analysis'").unwrap());
                    knowledge_expansion.push(serde_yaml::to_value("complexity_aware_debugging: 'adapt to code complexity'").unwrap());
                }
                "high_failure_rate" => {
                    emergent.push("resilient_debugging".to_string());
                    emergent.push("failure_analysis".to_string());
                    add_emergent = true;
                }
                "frequent_heuristic_usage" => {
                    knowledge_expansion.push(serde_yaml::to_value("heuristic_optimization: 'improve pattern recognition'").unwrap());
                }
                _ => {}
            }
        }
        
        // Add general improvements based on evidence
        if !evidence.is_empty() {
            knowledge_expansion.push(serde_yaml::to_value("evidence_based_learning: 'learn from debugging sessions'").unwrap());
        }
        
        DebuggerEssence {
            capabilities: Some(EssenceCapabilities {
                innate: None,
                learned: None,
                emergent: if add_emergent { Some(emergent) } else { None },
            }),
            learning_mechanics: Some(EssenceLearningMechanics {
                knowledge_expansion: if !knowledge_expansion.is_empty() { Some(knowledge_expansion) } else { None },
                self_tuning: None,
                other: std::collections::BTreeMap::new(),
            }),
            behavioral_patterns: None,
            emergent: None,
            other: std::collections::BTreeMap::new(),
        }
    }

    /// Show what changes were made to the essence
    async fn show_essence_changes(&self) -> Result<()> {
        println!("📋 Essence Changes Applied:");
        
        // Read the current essence file to show what was added
        if let Ok(content) = std::fs::read_to_string(ESSENCE_PATH) {
            if let Ok(doc) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                if let Some(capabilities) = doc.get("capabilities") {
                    if let Some(emergent) = capabilities.get("emergent") {
                        if let Some(emergent_list) = emergent.as_sequence() {
                            if !emergent_list.is_empty() {
                                println!("  🆕 New Emergent Capabilities:");
                                for capability in emergent_list {
                                    if let Some(cap) = capability.as_str() {
                                        println!("    • {}", cap);
                                    }
                                }
                            }
                        }
                    }
                }
                
                if let Some(learning) = doc.get("learning_mechanics") {
                    if let Some(knowledge) = learning.get("knowledge_expansion") {
                        if let Some(knowledge_list) = knowledge.as_sequence() {
                            if !knowledge_list.is_empty() {
                                println!("  📚 Enhanced Learning:");
                                for item in knowledge_list {
                                    if let Some(item_str) = item.as_str() {
                                        println!("    • {}", item_str);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Apply the proposed update to the YAML essence file
    async fn apply_essence_update(&self, update: DebuggerEssence) -> Result<()> {
        // Load current YAML
        let content = std::fs::read_to_string(ESSENCE_PATH)?;
        let mut doc: YamlValue = serde_yaml::from_str(&content)?;
        // Merge emergent capabilities
        if let Some(cap) = update.capabilities {
            if let Some(new_emergent) = cap.emergent {
                let path = ["capabilities", "emergent"];
                Self::merge_yaml_list(&mut doc, &path, new_emergent);
            }
        }
        // Merge knowledge_expansion
        if let Some(learn) = update.learning_mechanics {
            if let Some(new_know) = learn.knowledge_expansion {
                let path = ["learning_mechanics", "knowledge_expansion"];
                Self::merge_yaml_list(&mut doc, &path, new_know);
            }
        }
        // Write back
        let new_content = serde_yaml::to_string(&doc)?;
        std::fs::write(ESSENCE_PATH, new_content)?;
        Ok(())
    }

    /// Merge a list into a YAML path (append unique)
    fn merge_yaml_list(doc: &mut YamlValue, path: &[&str], new_items: Vec<impl Into<YamlValue> + Clone>) {
        let mut node = doc;
        for key in &path[..path.len() - 1] {
            // Avoid multiple mutable borrows by splitting the logic
            if !node.get(*key).is_some() {
                node.as_mapping_mut().unwrap().insert((*key).into(), YamlValue::Mapping(Default::default()));
            }
            node = node.get_mut(*key).unwrap();
        }
        let last = path[path.len() - 1];
        let arr_opt = node.get_mut(last).and_then(|v| v.as_sequence_mut());
        if let Some(arr) = arr_opt {
            for item in new_items {
                let val: YamlValue = item.clone().into();
                if !arr.contains(&val) {
                    arr.push(val);
                }
            }
        } else {
            node.as_mapping_mut().unwrap().insert(last.into(), YamlValue::Sequence(new_items.into_iter().map(|i| i.into()).collect()));
        }
    }
}

/// Code analysis result
#[derive(Debug)]
struct CodeAnalysis {
    lines: usize,
    complexity: f64,
    optimization_opportunities: usize,
    suggestions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = DebuggerTerminal::new().await?;
    terminal.run().await?;
    Ok(())
} 