# Combinator Universe - Dioxus WASM Game

**One game to explore the mathematical universe**

## Build & Run

```bash
# Install dioxus-cli
cargo install dioxus-cli

# Serve locally
dx serve

# Build for production
dx build --release
```

## Features

- 🧬 **Breed Memes**: Genetic algorithm with Y combinator
- 🌌 **8D Navigation**: Explore hyperspace
- 📊 **Real Perf Data**: Each meme is a performance trace
- 🎮 **Interactive**: Click, breed, navigate

## Game Mechanics

```rust
// Everything is a fixed point
mkycombinator!("Universe") => Universe(Universe)

// Breed memes
breed!(parent1, parent2) => child

// Navigate 8D space
move_player([x, y, z, w, ...])
```

## Deploy

The game compiles to WASM and runs in any browser!

```bash
# Build
dx build --release

# Output: dist/
# Deploy dist/ to any static host
```
