#!/bin/bash
# PROJECT VI - Smart Launcher for Linux/macOS
# Auto-installs dependencies and runs VI

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo ""
echo "========================================================"
echo "          PROJECT VI - Starting...                      "
echo "========================================================"
echo ""

# Check for Ollama
echo -e "${BLUE}[1/3] Checking for Ollama...${NC}"
if command -v ollama >/dev/null 2>&1; then
    echo -e "${GREEN}  ✓ Ollama found${NC}"
else
    echo -e "${YELLOW}  ! Ollama not found. Installing...${NC}"
    
    if ! curl -fsSL https://ollama.com/install.sh | sh; then
        echo -e "${RED}  ✗ Failed to install Ollama${NC}"
        echo "  Please install manually from: https://ollama.com"
        exit 1
    fi
    
    echo -e "${GREEN}  ✓ Ollama installed${NC}"
    sleep 2
fi
echo ""

# Start Ollama service if not running
if ! pgrep -x "ollama" > /dev/null; then
    echo -e "${YELLOW}  ! Starting Ollama service...${NC}"
    nohup ollama serve > /dev/null 2>&1 &
    sleep 3
fi

# Check for models
echo -e "${BLUE}[2/3] Checking for AI models...${NC}"

# Check for gemma2:2b
if ollama list 2>/dev/null | grep -q "gemma2:2b"; then
    echo -e "${GREEN}  ✓ gemma2:2b found${NC}"
else
    echo -e "${YELLOW}  ! Model gemma2:2b not found. Downloading...${NC}"
    echo "  ! This may take a few minutes (~1.1GB)"
    
    if ! ollama pull gemma2:2b; then
        echo -e "${RED}  ✗ Failed to download gemma2:2b${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}  ✓ gemma2:2b ready${NC}"
fi

# Check for tinyllama
if ollama list 2>/dev/null | grep -q "tinyllama"; then
    echo -e "${GREEN}  ✓ tinyllama found${NC}"
else
    echo -e "${YELLOW}  ! Model tinyllama not found. Downloading...${NC}"
    echo "  ! This may take a minute (~430MB)"
    
    if ! ollama pull tinyllama; then
        echo -e "${RED}  ✗ Failed to download tinyllama${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}  ✓ tinyllama ready${NC}"
fi
echo ""

# Launch VI
echo -e "${BLUE}[3/3] Launching PROJECT VI...${NC}"
echo ""
echo "========================================================"
echo "    All dependencies ready. Starting VI...             "
echo "========================================================"
echo ""

# Find and run the binary
if [ -f "./vi3" ]; then
    ./vi3
elif [ -f "./target/release/vi3" ]; then
    ./target/release/vi3
elif [ -f "/usr/local/bin/project-vi" ]; then
    /usr/local/bin/project-vi
else
    echo -e "${RED}✗ vi3 binary not found${NC}"
    echo "  Please run from the PROJECT VI directory"
    exit 1
fi


