#!/bin/bash

# Build script for Visual Novel Recommendation Engine Web App

echo "🚀 Building Visual Novel Recommendation Engine Web App..."

# Build the WASM package
echo "🔧 Building WASM package..."
wasm-pack build --target web --out-dir web/pkg --release

if [ $? -eq 0 ]; then
    echo "✅ WASM package built successfully!"
else
    echo "❌ Failed to build WASM package"
    exit 1
fi

# Create docs directory for GitHub Pages
echo "📁 Setting up GitHub Pages directory..."
rm -rf docs
mkdir -p docs

# Copy web files to docs directory
cp -r web/* docs/

# Copy data files to docs directory
echo "📊 Copying data files..."
cp data/vn_titles docs/
cp data/tags_vn docs/
cp data/vndb-votes-2025-06-02 docs/

echo "🎉 Build completed successfully!"
echo ""
echo "GitHub Pages setup:"
echo "  1. Push the 'docs' directory to your GitHub repository"
echo "  2. Enable GitHub Pages in repository settings"
echo "  3. Set source to 'Deploy from a branch' -> 'main' -> '/docs'"
echo ""
echo "To test locally:"
echo "  ./serve.sh"
