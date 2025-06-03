# Visual Novel Recommendation Engine Web App

A visual novel recommendation engine web application based on Rust and WebAssembly.

## Features

- 🎮 Tag-based recommendation algorithm
- 👥 User rating-based collaborative filtering
- 📊 Hybrid recommendation system
- 📁 Automatic data loading support
- 🌐 Modern web interface
- ⚡ High-performance computing with WebAssembly

## Quick Start

### Requirements

- Rust (latest stable version)
- wasm-pack
- Python 3 or Node.js (for local server)

### Installation Steps

1. **Clone the project**
   ```bash
   git clone <your-repo-url>
   cd visual_novel_recommendation_engine
   ```

2. **Build WebAssembly module**
   ```bash
   ./build_web.sh
   ```

3. **Start development server**
   ```bash
   ./serve.sh
   ```

4. **Open in browser**
   Visit http://localhost:8000

## How to Use

### Basic Recommendations

1. Enter a visual novel ID in the "Visual Novel ID" field (e.g., 17)
2. Click the "Get Recommendations" button
3. View three different types of recommendation results:
   - **Combined Recommendations**: Hybrid recommendations combining tags and user ratings
   - **Tag Recommendations**: Content similarity-based recommendations
   - **User Recommendations**: User rating similarity-based recommendations

### Data Loading

The application automatically loads the following data files from the same directory:
- `vn_titles`: Visual novel title data
- `tags_vn`: Tag relationship data  
- `vndb-votes-*`: User rating data

The loading progress is displayed in real-time on the interface.

## Technical Architecture

### Backend (Rust + WebAssembly)
- **sprs**: Sparse matrix computation
- **csv**: CSV file parsing
- **wasm-bindgen**: Rust-JavaScript interop
- **serde**: Serialization/deserialization

### Frontend (HTML + JavaScript + CSS)
- **Tailwind CSS**: Modern UI framework
- **Font Awesome**: Icon library
- **Vanilla JavaScript**: No framework dependencies

### Recommendation Algorithms
1. **Tag Similarity**: Calculates cosine similarity based on visual novel tags
2. **User Collaborative Filtering**: Finds similar users based on rating patterns
3. **Hybrid Recommendations**: Weighted average of both methods

## Development

### Project Structure
```
├── src/
│   ├── main.rs          # Command line interface
│   ├── lib.rs           # Library entry point
│   ├── recommender.rs   # Recommendation algorithm core
│   ├── data.rs          # Data structure definitions
│   └── wasm.rs          # WebAssembly bindings
├── web/
│   ├── index.html       # Main page
│   ├── index.js         # JavaScript logic
│   └── pkg/             # Generated WASM package
├── data/                # Data files
└── build_web.sh         # Build script
```

### Local Development

1. **Rebuild after modifying Rust code**:
   ```bash
   ./build_web.sh
   ```

2. **Start server only**:
   ```bash
   ./serve.sh
   ```

3. **Run native command line version**:
   ```bash
   cargo run --release -- --vn-id 17
   ```

## Troubleshooting

### WASM module loading failed
- Ensure wasm-pack is installed: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
- Rebuild: `./build_web.sh`

### CORS errors
- Ensure access through HTTP server, not by directly opening HTML files
- Use the provided `./serve.sh` script

### Empty recommendation results
- Confirm the input VN ID exists in the database
- Check if data files are loaded correctly

## Contributing

Issues and Pull Requests are welcome!

## License

[Add your license information here]
