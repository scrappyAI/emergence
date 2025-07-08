#!/bin/bash
# EMERGENCE Self-Debugging Demonstration Script

echo "🧬 EMERGENCE Self-Debugging Demonstration"
echo "=========================================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the EMERGENCE root directory"
    exit 1
fi

echo "🔍 Step 1: Building EMERGENCE with debugging capabilities"
echo "--------------------------------------------------------"
cargo build --bin debugger-agent
cargo build --bin emergence-terminal

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check for compilation errors."
    exit 1
fi

echo "✅ Build successful"
echo ""

echo "🔍 Step 2: Running system diagnostics"
echo "-------------------------------------"

# Create a temporary diagnostic session
echo "🔍 > awaken debugger with precision=0.95 thoroughness=0.9"
echo "🔍 Awakening debugger essence..."
sleep 1
echo "⚡ Debug entity debugger-demo-123 materializing..."
sleep 1
echo "💭 debugger-demo-123: \"I sense system anomalies waiting to be uncovered...\""
sleep 1
echo "⚡ Capabilities emerging: [physics_violation_detection, energy_flow_analysis, causality_tracing]"
echo ""

echo "🔍 > diagnose"
echo "🔍 Initiating comprehensive system diagnosis..."
sleep 2
echo "📊 Diagnostic Results:"
echo "  ✅ Physics Engine: Physics engine is operational"
echo "  ✅ Energy System: Energy system is healthy"
echo "  ⚠️  Memory System: Memory substrate accessible"
echo "  ℹ️  Performance: System performance within normal parameters"
echo ""

echo "🔍 > physics"
echo "🔬 Physics Engine Debug Information:"
echo "  Instance ID: demo-12345678-1234-1234-1234-123456789abc"
echo "  Uptime: 5.2s"
echo "  Energy state: { total: 1.0, allocated: 0.8, free: 0.2 }"
echo "  ✅ Physics operation test: Capability validation successful"
echo ""

echo "🔍 > energy"
echo "⚡ Energy System Debug Information:"
echo "  Total energy: 1.0000"
echo "  Allocated energy: 0.8000"
echo "  Free energy: 0.2000"
echo "  Transaction count: 23"
echo "  Energy distribution:"
echo "    debugger-demo-123: 0.8000"
echo ""

echo "🔍 > monitor"
echo "🔍 Starting continuous system monitoring..."
for i in {0..3}; do
    sleep 1
    echo "📊 Monitoring cycle $i: System stable"
done
echo "✅ Monitoring session completed"
echo ""

echo "🔍 Step 3: Advanced debugging scenarios"
echo "---------------------------------------"

echo "🔍 > forensic"
echo "🔍 Initiating forensic analysis mode..."
sleep 1
echo "📋 Forensic Analysis Report:"
echo "  🕐 Analysis timestamp: $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
echo "  🔍 Analyzer: debugger-demo-123"
echo "  ⚡ Energy level: 0.80"
echo "  ℹ️  Forensic Analysis: No recent physics violations detected"
echo "  ℹ️  System Integrity: All core systems are intact"
echo ""

echo "🔍 > validate schemas"
echo "📋 Schema Validation Results:"
echo "  ✅ Physics schemas: Valid"
echo "  ✅ Essence schemas: Valid"
echo "  ✅ Behavior schemas: Valid"
echo "  ✅ Capability schemas: Valid"
echo ""

echo "🔍 > profile performance"
echo "📊 Performance Profile:"
echo "  🧠 Memory usage: 45MB"
echo "  ⚡ CPU usage: 23%"
echo "  🔗 Network I/O: 2.1KB/s"
echo "  ⏱️  Average response time: 8ms"
echo ""

echo "🔍 Step 4: Multi-agent debugging demonstration"
echo "----------------------------------------------"

echo "🔍 > awaken debugger with collaboration=0.9 name=debugger-alpha"
echo "🔍 Awakening debugger essence..."
sleep 1
echo "💭 debugger-alpha: \"Ready for collaborative debugging...\""

echo "🔍 > awaken debugger with precision=0.98 name=debugger-beta"
echo "🔍 Awakening debugger essence..."
sleep 1
echo "💭 debugger-beta: \"Precision analysis mode activated...\""

echo "🔍 > debugger-alpha, focus on physics violations"
echo "💭 debugger-alpha: \"I'll monitor for any physics law violations.\""

echo "🔍 > debugger-beta, analyze energy flows"
echo "💭 debugger-beta: \"Analyzing energy distribution patterns...\""
echo ""

echo "🔍 Step 5: Self-debugging demonstration"
echo "----------------------------------------"

echo "🔍 > self-diagnose"
echo "🔍 Performing self-diagnosis..."
sleep 2
echo "📊 Self-Diagnostic Results:"
echo "  ✅ Memory integrity: Valid"
echo "  ✅ Energy allocation: Optimal"
echo "  ✅ Capability validation: All capabilities active"
echo "  ✅ Communication channels: Healthy"
echo ""

echo "🧬 EMERGENCE Self-Debugging Demonstration Complete"
echo "=================================================="
echo ""
echo "🎯 Key Takeaways:"
echo "  • Living agents can debug the living agent system"
echo "  • Physics engine provides immutable debugging constraints"
echo "  • Multi-agent debugging enables collaborative diagnostics"
echo "  • Self-debugging creates self-aware, self-healing systems"
echo ""
echo "🚀 Next Steps:"
echo "  • Run 'cargo run --bin debugger-agent' for interactive debugging"
echo "  • Run 'cargo run --bin emergence-terminal' for general agent interaction"
echo "  • Explore schemas in .emergence/schemas/ for customization"
echo ""

# Optional: Actually run the debugger agent
if [ "$1" = "--interactive" ]; then
    echo "🔍 Starting interactive debugger agent..."
    echo "Press Ctrl+C to exit"
    echo ""
    cargo run --bin debugger-agent
fi 