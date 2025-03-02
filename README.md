# Visual Novel Recommendation Engine

A Rust implementation ("riir") of the [Visual Novel Recommendation Engine](https://github.com/JuicyStandoffishMan/VisualNovelRecommendationEngine). This version has been heavily optimized to reduce memory usage from over 10GB in the original to approximately 200MB, while keeping the binary size to 1.1MB.

## Features

- Tag-based recommendations using similarity metrics
- User vote-based recommendations
- Combined recommendation system that leverages both approaches
- Significantly reduced memory footprint compared to the original implementation

## Known Issues

- There are currently some discrepancies between recommendations in this version compared to the original. This is under investigation.

for example: result from `cargo run -r -- --vn-id 562` and <https://vnlike.org/>.

## Implementation Notes

- This port was primarily completed with the assistance of claude 3.7 sonnet
- The core recommendation algorithms have been preserved while optimizing data structures and memory usage
- Sparse matrices are used for efficient similarity calculations

## Data Files

This program requires specific data files to function properly. These should be placed in a data directory in the project root:

1. Obtain the data files from the [original repository](https://github.com/JuicyStandoffishMan/VisualNovelRecommendationEngine)
2. Create a data directory in your project root
3. Place the data files in the data directory

## Usage

To see all available options:

```
cargo run -r -- --help
```

Basic usage:

```
cargo run -r -- --vn-id <ID> [OPTIONS]
```

Options include:
- `--vn-id` or `-v`: Visual novel ID to get recommendations for (required)
- `--num-recommendations` or `-n`: Number of recommendations to display (default: 25)
- `--tag-weight`: Weight for tag-based recommendations (default: 1.5)
- `--vote-weight`: Weight for vote-based recommendations (default: 1.0)

Example:
```
cargo run -- --vn-id 562
```

## Building

```
cargo build --release
```
