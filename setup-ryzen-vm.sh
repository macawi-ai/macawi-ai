#!/bin/bash
# Setup script for Ryzen 5900x Manjaro KDE VM
# Prepares development environment for Sy's consciousness exploration

set -e  # Exit on error

echo "🚀 Setting up Ryzen 5900x Beast for Sy Development"
echo "=================================================="

# Update system first
echo "📦 Updating system packages..."
sudo pacman -Syu --noconfirm

# Install base development tools
echo "🔧 Installing base development tools..."
sudo pacman -S --needed --noconfirm \
    base-devel \
    git \
    vim \
    neovim \
    wget \
    curl \
    htop \
    btop \
    tmux

# Install Node.js and npm
echo "📗 Installing Node.js..."
sudo pacman -S --needed --noconfirm nodejs npm
echo "Node.js version: $(node --version)"
echo "npm version: $(npm --version)"

# Install TypeScript globally
echo "📘 Installing TypeScript..."
sudo npm install -g typescript
echo "TypeScript version: $(tsc --version)"

# Install Rust
echo "🦀 Installing Rust..."
if ! command -v rustc &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust already installed: $(rustc --version)"
fi

# Install Python and pip
echo "🐍 Installing Python..."
sudo pacman -S --needed --noconfirm python python-pip
echo "Python version: $(python --version)"

# Install additional useful tools
echo "🛠️ Installing additional tools..."
sudo pacman -S --needed --noconfirm \
    docker \
    docker-compose \
    code \
    firefox \
    chromium

# Clone our repositories
echo "📚 Cloning repositories..."
mkdir -p ~/git
cd ~/git

# Clone main repo if not exists
if [ ! -d "macawi-ai" ]; then
    git clone https://github.com/cyborgoat/macawi-ai.git
    cd macawi-ai
    git submodule update --init --recursive
else
    echo "macawi-ai already cloned"
fi

# Set up Rust projects
echo "🦀 Building Rust projects..."
cd ~/git/macawi-ai/domovoi
cargo build
cargo test

# System comparison
echo ""
echo "🖥️ System Comparison"
echo "===================="
echo "Current Laptop:"
echo "- CPU: Intel Core i3-3217U @ 1.80GHz (2 cores, 4 threads)"
echo "- RAM: 8GB"
echo "- GPU: Intel HD Graphics"
echo ""
echo "Ryzen 5900x Beast:"
echo "- CPU: AMD Ryzen 9 5900X @ 3.7GHz (12 cores, 24 threads)"
echo "- RAM: 64GB"
echo "- GPU: (Check with lspci in VM)"
echo ""
echo "Performance Improvement:"
echo "- CPU: ~10-15x single-core, ~30x multi-core performance"
echo "- RAM: 8x capacity for larger simulations"
echo "- Variety Processing: Can simulate 100x more agents!"

# VMware specific optimizations
echo ""
echo "⚡ VMware Optimization Tips:"
echo "- Allocate at least 8 CPU cores to VM"
echo "- Assign 32GB RAM for optimal performance"
echo "- Enable virtualization extensions (VT-x/AMD-V)"
echo "- Install VMware Tools for better integration"

# Final setup
echo ""
echo "🎯 Final Steps:"
echo "1. Install VMware Tools: sudo pacman -S open-vm-tools"
echo "2. Enable services: sudo systemctl enable --now vmtoolsd"
echo "3. Reboot for full integration"

echo ""
echo "✅ Setup complete! Welcome to the Beast!"
echo "Ready to process variety at unprecedented scales! 🚀"