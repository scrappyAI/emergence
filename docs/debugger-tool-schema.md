# EMERGENCE Debugger Agent - LLM Tool Schema

## Overview

The EMERGENCE Debugger Agent provides programmatic access to system diagnostics, monitoring, and optimization capabilities. This schema defines the available tools for LLM integration.

## Tool Functions

### 1. Initialize Debugger Interface
**Function**: `init_debugger()`
**Description**: Creates a new debugger interface instance
**Returns**: `DebuggerInterface`
**Usage**: First step to access debugger capabilities

### 2. Awaken Debugger Agent
**Function**: `awaken_debugger(debugger: &mut DebuggerInterface)`
**Description**: Awakens the debugger agent with default personality traits
**Parameters**: 
- `debugger`: Mutable reference to debugger interface
**Returns**: `DebuggerAgent`
**Usage**: Activates the debugger agent for diagnostics

### 3. Perform System Diagnosis
**Function**: `quick_diagnosis(debugger: &DebuggerInterface)`
**Description**: Performs a comprehensive system diagnosis
**Parameters**:
- `debugger`: Reference to debugger interface
**Returns**: `DiagnosticSession`
**Usage**: Get system health status and identify issues

### 4. Get System Health Metrics
**Function**: `system_health(debugger: &DebuggerInterface)`
**Description**: Retrieves current system metrics and performance data
**Parameters**:
- `debugger`: Reference to debugger interface
**Returns**: `SystemMetrics`
**Usage**: Monitor real-time system performance

### 5. Get Available Strategies
**Function**: `available_strategies(debugger: &DebuggerInterface)`
**Description**: Lists available search and optimization strategies
**Parameters**:
- `debugger`: Reference to debugger interface
**Returns**: `Vec<SearchStrategy>`
**Usage**: Understand available diagnostic approaches

### 6. Trigger Optimization
**Function**: `trigger_optimization(debugger: &mut DebuggerInterface)`
**Description**: Triggers self-optimization and generates new strategies
**Parameters**:
- `debugger`: Mutable reference to debugger interface
**Returns**: `Vec<OptimizationRecord>`
**Usage**: Improve debugger performance and capabilities

### 7. Forensic Analysis
**Function**: `forensic_analysis(debugger: &DebuggerInterface, target_system: Option<&str>)`
**Description**: Performs deep forensic analysis of specified system
**Parameters**:
- `debugger`: Reference to debugger interface
- `target_system`: Optional target system name (default: "energy-system")
**Returns**: `Vec<DiagnosticFinding>`
**Usage**: Investigate specific system issues or violations

## Data Structures

### DiagnosticSession
```rust
pub struct DiagnosticSession {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub target_system: String,
    pub findings: Vec<DiagnosticFinding>,
    pub success: bool,
    pub search_strategy_used: Option<String>,
}
```

### DiagnosticFinding
```rust
pub struct DiagnosticFinding {
    pub severity: FindingSeverity,
    pub category: String,
    pub description: String,
    pub evidence: Vec<String>,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}
```

### SystemMetrics
```rust
pub struct SystemMetrics {
    pub physics_engine: PhysicsMetrics,
    pub energy_system: EnergyMetrics,
    pub memory_system: MemoryMetrics,
    pub nervous_system: NervousSystemMetrics,
    pub timestamp: DateTime<Utc>,
}
```

### SearchStrategy
```rust
pub struct SearchStrategy {
    pub name: String,
    pub description: String,
    pub success_rate: f64,
    pub energy_efficiency: f64,
    pub complexity: f64,
    pub last_used: Option<DateTime<Utc>>,
    pub usage_count: u32,
}
```

### OptimizationRecord
```rust
pub struct OptimizationRecord {
    pub timestamp: DateTime<Utc>,
    pub original_strategy: String,
    pub new_strategy: String,
    pub improvement_metrics: HashMap<String, f64>,
    pub reasoning: String,
}
```

## Usage Examples

### Basic System Health Check
```rust
let mut debugger = tools::init_debugger().await?;
tools::awaken_debugger(&mut debugger).await?;
let metrics = tools::system_health(&debugger).await?;
```

### Comprehensive Diagnosis
```rust
let diagnosis = tools::quick_diagnosis(&debugger).await?;
if !diagnosis.success {
    let strategies = tools::available_strategies(&debugger).await?;
    let optimizations = tools::trigger_optimization(&mut debugger).await?;
}
```

### Forensic Investigation
```rust
let findings = debugger.forensic_analysis(Some("energy-system")).await?;
for finding in findings {
    if matches!(finding.severity, FindingSeverity::Critical | FindingSeverity::Error) {
        // Handle critical issues
    }
}
```

## Integration Patterns

### 1. Monitoring Loop
```rust
loop {
    let metrics = tools::system_health(&debugger).await?;
    if metrics.energy_system.energy_efficiency < 0.8 {
        let diagnosis = tools::quick_diagnosis(&debugger).await?;
        // Handle low efficiency
    }
    tokio::time::sleep(Duration::from_secs(30)).await;
}
```

### 2. Adaptive Optimization
```rust
let strategies = tools::available_strategies(&debugger).await?;
let best_strategy = strategies.iter()
    .max_by(|a, b| a.success_rate.partial_cmp(&b.success_rate).unwrap())
    .unwrap();

if best_strategy.success_rate < 0.9 {
    let optimizations = tools::trigger_optimization(&mut debugger).await?;
    // Use new optimized strategies
}
```

### 3. Issue Investigation
```rust
// Initial diagnosis
let diagnosis = tools::quick_diagnosis(&debugger).await?;

// If issues found, perform forensic analysis
if !diagnosis.success {
    for finding in &diagnosis.findings {
        if matches!(finding.severity, FindingSeverity::Critical | FindingSeverity::Error) {
            let forensic = debugger.forensic_analysis(Some(&finding.category)).await?;
            // Analyze forensic findings
        }
    }
}
```

## Error Handling

All functions return `Result<T, anyhow::Error>`. Common error scenarios:

- **Debugger not awakened**: Call `awaken_debugger()` first
- **System unavailable**: Check if EMERGENCE runtime is running
- **Network issues**: Ensure connection to physics engine and memory substrate
- **Permission denied**: Verify agent has appropriate capabilities

## Performance Considerations

- **Initialization**: `init_debugger()` may take 1-2 seconds
- **Diagnosis**: `quick_diagnosis()` typically completes in 100-500ms
- **Optimization**: `trigger_optimization()` may take 2-5 seconds
- **Forensic Analysis**: `forensic_analysis()` can take 1-3 seconds

## Security Notes

- All diagnostic data is read-only
- No system modifications are performed
- Agent capabilities are validated before operations
- Energy consumption is monitored and limited

## Best Practices

1. **Initialize once**: Reuse debugger interface across operations
2. **Monitor regularly**: Use `system_health()` for continuous monitoring
3. **Optimize strategically**: Trigger optimization when performance degrades
4. **Handle errors gracefully**: Always check `Result` return values
5. **Use appropriate strategies**: Select strategies based on problem type 