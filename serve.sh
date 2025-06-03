#!/bin/bash

# Development server script

echo "🌐 Starting development server..."

# Check if we have either web or docs directory
if [ -f "docs/index.html" ]; then
    echo "📁 Using docs directory (GitHub Pages ready)"
    cd docs
elif [ -f "web/index.html" ]; then
    echo "📁 Using web directory"
    cd web
else
    echo "❌ Neither docs/index.html nor web/index.html found. Please run build_web.sh first."
    exit 1
fi

# Try different server options
if command -v python3 &> /dev/null; then
    echo "🐍 Starting Python HTTP server on http://localhost:8000"
    echo "🌐 Open http://localhost:8000 in your browser"
    python3 -m http.server 8000
elif command -v python &> /dev/null; then
    echo "🐍 Starting Python HTTP server on http://localhost:8000"
    echo "🌐 Open http://localhost:8000 in your browser"
    python -m http.server 8000
elif command -v node &> /dev/null && command -v npx &> /dev/null; then
    echo "📦 Starting Node.js HTTP server on http://localhost:8000"
    echo "🌐 Open http://localhost:8000 in your browser"
    npx http-server -p 8000
else
    echo "❌ No suitable HTTP server found. Please install Python 3 or Node.js"
    exit 1
fi
