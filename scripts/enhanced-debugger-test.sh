#!/bin/bash

# Enhanced EMERGENCE Debugger Agent Test
# This script demonstrates the full capabilities of the self-optimizing debugger

echo "🧠 Enhanced EMERGENCE Debugger Agent Test"
echo "=========================================="
echo ""

# Function to send commands to debugger
send_command() {
    local cmd="$1"
    echo "Command: $cmd"
    echo "$cmd" | cargo run --bin debugger-agent
    echo ""
    sleep 1
}

echo "🚀 Starting enhanced debugger test..."
echo ""

# Test 1: Awaken debugger with optimized traits
echo "Test 1: Awakening debugger with optimized personality"
send_command "awaken debugger precision=0.95 thoroughness=0.9 skepticism=0.8 patience=0.7 creativity=0.6"

# Test 2: Show initial strategies
echo "Test 2: Displaying initial search strategies"
send_command "strategies"

# Test 3: Perform comprehensive diagnosis
echo "Test 3: Performing comprehensive system diagnosis"
send_command "diagnose"

# Test 4: Show status
echo "Test 4: Checking debugger status"
send_command "status"

# Test 5: Analyze debugger's own code
echo "Test 5: Analyzing debugger code for optimization"
send_command "analyze"

# Test 6: Trigger self-optimization
echo "Test 6: Triggering self-optimization"
send_command "optimize"

# Test 7: Show updated strategies
echo "Test 7: Displaying optimized search strategies"
send_command "strategies"

# Test 8: Perform another diagnosis to see improvements
echo "Test 8: Performing diagnosis with optimized strategies"
send_command "diagnose"

# Test 9: Monitor system
echo "Test 9: Monitoring system performance"
send_command "monitor"

# Test 10: Perform forensic analysis
echo "Test 10: Performing forensic analysis"
send_command "forensic"

# Test 11: Reflect and update essence
echo "Test 11: Reflecting on evidence and updating essence"
send_command "reflect"

# Test 12: Show final status
echo "Test 12: Final debugger status"
send_command "status"

# Test 13: Debug physics engine
echo "Test 13: Debugging physics engine"
send_command "physics"

# Test 14: Debug energy system
echo "Test 14: Debugging energy system"
send_command "energy"

# Test 15: Exit
echo "Test 15: Exiting debugger"
send_command "exit"

echo ""
echo "✅ Enhanced debugger test completed!"
echo ""
echo "🎯 Key Enhancements Demonstrated:"
echo "  • Improved input handling and error recovery"
echo "  • Enhanced diagnostic analysis with detailed findings"
echo "  • Intelligent pattern recognition in evidence analysis"
echo "  • Self-optimization with strategy adaptation"
echo "  • Real-time essence schema updates"
echo "  • Comprehensive system monitoring and forensic analysis"
echo "  • Better feedback and detailed output"
echo ""
echo "🧠 Self-Optimization Features:"
echo "  • Evidence collection from debugging sessions"
echo "  • Pattern analysis in failures and successes"
echo "  • Dynamic strategy generation based on patterns"
echo "  • Essence schema updates with new capabilities"
echo "  • Learning from code complexity analysis"
echo "  • Performance tracking and improvement"
echo ""
echo "🔍 Diagnostic Capabilities:"
echo "  • Physics engine state analysis"
echo "  • Energy system monitoring"
echo "  • System resource assessment"
echo "  • Detailed findings with evidence and recommendations"
echo "  • Severity-based issue categorization"
echo "  • Comprehensive reporting with summaries"
echo "" 