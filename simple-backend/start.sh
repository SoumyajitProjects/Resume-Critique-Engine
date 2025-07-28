#!/bin/bash

echo "🚀 Starting Resume Critique Backend..."

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "📦 Creating virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment and install dependencies
echo "📦 Installing dependencies..."
source venv/bin/activate
pip install -r requirements.txt

# Start the server
echo "🔥 Starting FastAPI server on http://localhost:3000"
python3 main.py
