//! Example: Using EMERGENCE Debugger Agent as an LLM Tool
//!
//! This example demonstrates how LLMs can programmatically access the debugger
//! agent's capabilities for system diagnostics, monitoring, and optimization.

use emergence_runtime::debugger::{DebuggerInterface, tools};
use anyhow::Result;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔍 EMERGENCE Debugger Agent - LLM Tool Example");
    println!("===============================================\n");

    // Initialize debugger interface
    println!("1. Initializing debugger interface...");
    let mut debugger = tools::init_debugger().await?;
    println!("✅ Debugger interface ready\n");

    // Awaken debugger agent
    println!("2. Awakening debugger agent...");
    let agent = tools::awaken_debugger(&mut debugger).await?;
    println!("✅ Debugger agent awakened: {}", agent.name);
    println!("   Energy: {:.1}", agent.energy);
    println!("   State: {:?}", agent.state);
    println!("   Strategies available: {}", agent.search_strategies.len());
    println!();

    // Analyze the broken code file we created
    println!("3. Analyzing broken code file...");
    let broken_file = "/tmp/emergence-debug-test/broken.rs";
    if let Ok(analysis) = tools::analyze_code(&debugger, broken_file).await {
        println!("✅ Code analysis completed");
        println!("   File: {}", analysis.file_path);
        println!("   Language: {}", analysis.language);
        println!("   Complexity score: {:.2}", analysis.complexity_score);
        println!("   Issues found: {}", analysis.issues.len());
        println!("   Patterns identified: {}", analysis.patterns.len());
        
        for issue in &analysis.issues {
            println!("   - [{:?}] {}: {}", issue.severity, issue.category, issue.description);
            if let Some(line) = issue.line_number {
                println!("     Line: {}", line);
            }
            if let Some(snippet) = &issue.code_snippet {
                println!("     Code: {}", snippet);
            }
            println!("     Explanation: {}", issue.explanation);
            if let Some(fix) = &issue.suggested_fix {
                println!("     Suggested fix: {}", fix);
            }
        }
        
        println!("   Recommendations:");
        for rec in &analysis.recommendations {
            println!("     • {}", rec);
        }
    } else {
        println!("❌ Could not analyze broken file (file may not exist)");
    }
    println!();

    // Analyze compilation errors
    println!("4. Analyzing compilation errors...");
    let error_log = r#"warning: unused variable: `x`
 --> broken.rs:1:17
  |
1 | fn main() { let x = 1/0; }
  |                 ^ help: if this is intentional, prefix it with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

error: this operation will panic at runtime
 --> broken.rs:1:21
  |
1 | fn main() { let x = 1/0; }
  |                     ^^^ attempt to divide `1_i32` by zero
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: aborting due to 1 previous error; 1 warning emitted"#;

    if let Ok(errors) = tools::analyze_errors(&debugger, error_log).await {
        println!("✅ Compilation error analysis completed");
        println!("   Errors found: {}", errors.len());
        
        for error in &errors {
            println!("   - Type: {}", error.error_type);
            println!("     Message: {}", error.message);
            if let Some(line) = error.line {
                println!("     Line: {}", line);
            }
            if let Some(suggestion) = &error.suggestion {
                println!("     Suggestion: {}", suggestion);
            }
            println!("     Context: {}", error.context);
        }
    }
    println!();

    // Analyze error logs for diagnostic findings
    println!("5. Analyzing error logs for diagnostic findings...");
    if let Ok(findings) = tools::analyze_logs(&debugger, error_log).await {
        println!("✅ Error log analysis completed");
        println!("   Diagnostic findings: {}", findings.len());
        
        for finding in &findings {
            println!("   - [{:?}] {}: {}", finding.severity, finding.category, finding.description);
            println!("     Evidence:");
            for evidence in &finding.evidence {
                println!("       • {}", evidence);
            }
            println!("     Recommendations:");
            for rec in &finding.recommendations {
                println!("       • {}", rec);
            }
        }
    }
    println!();

    // Perform system diagnosis
    println!("6. Performing system diagnosis...");
    let diagnosis = tools::quick_diagnosis(&debugger).await?;
    println!("✅ Diagnosis completed");
    println!("   Session ID: {}", diagnosis.session_id);
    println!("   Target: {}", diagnosis.target_system);
    println!("   Success: {}", diagnosis.success);
    println!("   Findings: {}", diagnosis.findings.len());
    
    for finding in &diagnosis.findings {
        println!("   - [{:?}] {}: {}", finding.severity, finding.category, finding.description);
    }
    println!();

    // Get system health metrics
    println!("7. Collecting system health metrics...");
    let metrics = tools::system_health(&debugger).await?;
    println!("✅ System metrics collected");
    println!("   Physics Engine:");
    println!("     Active entities: {}", metrics.physics_engine.active_entities);
    println!("     Operations/sec: {:.1}", metrics.physics_engine.physics_operations_per_second);
    println!("   Energy System:");
    println!("     Total energy: {:.1}", metrics.energy_system.total_energy);
    println!("     Efficiency: {:.1}%", metrics.energy_system.energy_efficiency * 100.0);
    println!("   Nervous System:");
    println!("     Active agents: {}", metrics.nervous_system.active_agents);
    println!("     Coordination: {:.1}%", metrics.nervous_system.coordination_efficiency * 100.0);
    println!();

    // Get available strategies
    println!("8. Retrieving available search strategies...");
    let strategies = tools::available_strategies(&debugger).await?;
    println!("✅ Found {} strategies:", strategies.len());
    for strategy in &strategies {
        println!("   - {}: {} (Success: {:.1}%, Efficiency: {:.1}%)", 
            strategy.name, strategy.description, 
            strategy.success_rate * 100.0, strategy.energy_efficiency * 100.0);
    }
    println!();

    // Trigger optimization
    println!("9. Triggering self-optimization...");
    let optimizations = tools::trigger_optimization(&mut debugger).await?;
    println!("✅ Optimization completed");
    for opt in &optimizations {
        println!("   Generated: {} (from {})", opt.new_strategy, opt.original_strategy);
        println!("   Reasoning: {}", opt.reasoning);
        for (metric, value) in &opt.improvement_metrics {
            println!("   - {}: {:.1}%", metric, value * 100.0);
        }
    }
    println!();

    // Perform forensic analysis
    println!("10. Performing forensic analysis...");
    let forensic_findings = debugger.forensic_analysis(Some("energy-system")).await?;
    println!("✅ Forensic analysis completed");
    println!("   Findings: {}", forensic_findings.len());
    for finding in &forensic_findings {
        println!("   - [{:?}] {}: {}", finding.severity, finding.category, finding.description);
    }
    println!();

    // Get updated strategies after optimization
    println!("11. Checking updated strategies...");
    let updated_strategies = tools::available_strategies(&debugger).await?;
    println!("✅ Updated strategies available: {}", updated_strategies.len());
    for strategy in &updated_strategies {
        if strategy.name.contains("enhanced") || strategy.name.contains("pattern") {
            println!("   - {}: {} (Success: {:.1}%, Efficiency: {:.1}%)", 
                strategy.name, strategy.description, 
                strategy.success_rate * 100.0, strategy.energy_efficiency * 100.0);
        }
    }
    println!();

    println!("🎉 LLM Tool Example Completed Successfully!");
    println!("\nThis demonstrates how LLMs can:");
    println!("  • Initialize and awaken debugger agents");
    println!("  • Analyze code files for issues and patterns");
    println!("  • Parse and analyze compilation errors");
    println!("  • Extract diagnostic findings from error logs");
    println!("  • Perform system diagnostics and monitoring");
    println!("  • Access optimization strategies and metrics");
    println!("  • Trigger self-optimization processes");
    println!("  • Perform forensic analysis");
    println!("  • Get real-time system health data");

    Ok(())
} 