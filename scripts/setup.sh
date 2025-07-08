#!/bin/bash
# EMERGENCE development environment setup

echo "�� Setting up EMERGENCE development environment..."

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.cargo/env
fi

# Install required tools
echo "⚡ Installing development tools..."
cargo install --force cargo-watch
cargo install --force cargo-edit

# Validate workspace
echo "⚡ Validating workspace..."
cargo check

# Create runtime directories
echo "📁 Creating runtime directories..."
mkdir -p .emergence/runtime/{cache,sessions}
mkdir -p .emergence/validation/temp

echo "✅ EMERGENCE development environment ready!"
echo "🚀 Try: cargo build"
echo ""
echo "📚 Next steps:"
echo "  - Build the workspace: cargo build"
echo "  - Run tests: cargo test"
echo "  - Explore schemas: ls .emergence/schemas/"
