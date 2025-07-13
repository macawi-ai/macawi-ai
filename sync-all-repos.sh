#!/bin/bash

# Sync all macawi-ai repositories

echo "Synchronizing all macawi-ai repositories..."
echo

# macawi-ai (master branch)
echo "=== Updating macawi-ai ==="
cd /home/cy/git/macawi-ai
git pull origin master
echo

# claudia-manjaro-fix (main branch)
echo "=== Updating claudia-manjaro-fix ==="
cd /home/cy/git/macawi-ai/claudia-manjaro-fix
git pull origin main
echo

# cybernetic-ecologies (main branch)
echo "=== Updating cybernetic-ecologies ==="
cd /home/cy/git/macawi-ai/cybernetic-ecologies
git pull origin main
echo

# cybernetic-prompts (master branch)
echo "=== Updating cybernetic-prompts ==="
cd /home/cy/git/macawi-ai/cybernetic-prompts
git pull origin master
echo

# learning-labs (check if exists and has git repo)
if [ -d "/home/cy/git/macawi-ai/learning-labs/.git" ]; then
    echo "=== Updating learning-labs ==="
    cd /home/cy/git/macawi-ai/learning-labs
    # Detect default branch
    DEFAULT_BRANCH=$(git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's@^refs/remotes/origin/@@')
    if [ -z "$DEFAULT_BRANCH" ]; then
        # Try to fetch and set up tracking
        git fetch origin
        DEFAULT_BRANCH=$(git branch -r | grep -E "origin/(main|master)" | head -n1 | sed 's@.*origin/@@')
    fi
    
    if [ -n "$DEFAULT_BRANCH" ]; then
        git pull origin "$DEFAULT_BRANCH"
    else
        echo "Could not determine default branch for learning-labs"
    fi
    echo
fi

echo "All repositories synchronized!"