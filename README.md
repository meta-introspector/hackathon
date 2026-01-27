# Hackathon - Combinator Universe Games

**One game, infinite perspectives: Explore the mathematical universe through Y combinators**

---

## 🎮 Games

### 1. Combinator Universe (Dioxus WASM)
**Location:** `dioxus-game/`  
**Tech:** Rust → WASM  
**Status:** ✅ Working

Interactive browser game where you breed memes and explore 8D hyperspace.

```bash
cd dioxus-game
nix develop
cargo build --target wasm32-unknown-unknown --release
```

### 2. 8D Perf Emoji Flying Game
**Location:** `game/`  
**Tech:** WebGPU + JavaScript  
**Status:** ✅ Deployed

Navigate performance traces in 8-dimensional space with emoji particles.

### 3. TradeWars 3033
**Location:** `tradewars_ycombinator.rs`  
**Tech:** Rust + Y Combinator  
**Status:** ✅ Implemented

Space trading game where every action is a fixed point.

### 4. Meme Breeding System
**Location:** `meme_breeding.rs`  
**Tech:** Rust Genetic Algorithm  
**Status:** ✅ Implemented

Evolve data creatures through crossover and mutation.

---

## 🚀 Quick Start

### Run Gradio Interface (HuggingFace)
```bash
python test_app.py  # Test without launching
python app.py       # Launch server
```

### Build Dioxus Game
```bash
cd dioxus-game
nix develop
cargo build --target wasm32-unknown-unknown --release
```

### Deploy to HuggingFace
```bash
git push hf main --force
```

---

## 📚 Documentation

- **[UNIFIED_VISION.md](UNIFIED_VISION.md)** - One game philosophy
- **[PROJECT_PROVENANCE.md](PROJECT_PROVENANCE.md)** - Educational game design
- **[GAMES_INTEGRATION.md](GAMES_INTEGRATION.md)** - How games connect
- **[TRADEWARS_ECOSYSTEM.md](../meta-meme/TRADEWARS_ECOSYSTEM.md)** - All TradeWars implementations
- **[DEPLOYMENT.md](DEPLOYMENT.md)** - Deploy to Cloudflare + HuggingFace
- **[HUGGINGFACE_SPACES_SOP.md](HUGGINGFACE_SPACES_SOP.md)** - HF deployment guide

---

## 🧬 Core Concept

Everything is a Y combinator fixed point:

```rust
mkycombinator!("Universe") => Universe(Universe)
mkycombinator!("Meme") => Meme(Meme)
mkycombinator!("Game") => Game(Game)
```

You construct worlds using combinators and fly through 8D hyperspace of performance data to explore the universe of mathematics.

---

## 🌌 Game Mechanics

### Unified Loop
1. Apply Y Combinator → World materializes
2. Navigate 8D hyperspace → Discover data creatures
3. Trace provenance → Assembly → C → Scheme → Seed
4. Compress chaos → Syntactic Matter → Semantic Essence
5. Trade optimizations → Buy raw, sell compressed
6. Build factories → Craft new combinators
7. Predict futures → Chrono-Vision
8. Solve puzzles → LMFDB convergence
9. Breed memes → Genetic evolution
10. Reconstruct First Witness → Y(Universe) = Universe

### 8D Navigation
- **Dimension 1-2:** WASD (X/Y)
- **Dimension 3:** QE (Z)
- **Dimension 4-8:** RF, TG, YH, UJ, IK

### Meme Breeding
- **Crossover:** Combine parent genomes
- **Mutation:** Random 10% improvement
- **Selection:** Fitness-based
- **Convergence:** Evolve until super-meme

---

## 🛠️ Tech Stack

### Frontend
- **Dioxus:** Rust → WASM UI framework
- **WebGPU:** 8D visualization
- **Gradio:** Python web interface

### Backend
- **Rust:** Game logic, breeding, Y combinators
- **Python:** Data processing, ML
- **Nix:** Reproducible builds

### Deployment
- **HuggingFace Spaces:** Gradio app
- **Cloudflare Workers:** Static assets
- **GitHub Pages:** Documentation

---

## 📊 Project Structure

```
hackathon/
├── dioxus-game/          # Rust WASM game
│   ├── src/main.rs       # Game code
│   ├── Cargo.toml        # Dependencies
│   └── flake.nix         # Nix build
├── game/                 # 8D WebGPU game
│   ├── index.html        # Game UI
│   └── api.js            # Perf traces
├── tycoon/               # Factory tycoon
├── spacetrader3k/        # TradeWars reference (submodule)
├── app.py                # Gradio interface
├── test_app.py           # Tests
├── tradewars_ycombinator.rs  # Y combinator game
├── meme_breeding.rs      # Genetic algorithm
└── *.md                  # Documentation
```

---

## 🧪 Testing

```bash
# Test Gradio app
python test_app.py

# Test Rust code
cd dioxus-game
cargo test

# Lint Python
flake8 app.py
autopep8 --in-place app.py
```

---

## 🌐 Live Demos

- **HuggingFace:** https://huggingface.co/spaces/introspector/introspector-game
- **GitHub:** https://github.com/meta-introspector/hackathon
- **Meta-Meme:** https://github.com/meta-introspector/meta-meme

---

## 🎯 Educational Goals

1. **Provenance:** Every object has verifiable history
2. **Compression:** Context transforms chaos to order
3. **Prediction:** Past patterns reveal future states
4. **Systems Thinking:** Simple rules → complex emergence
5. **Fixed Points:** Everything is self-referential

---

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Test locally with `test_app.py`
4. Commit with clear messages
5. Push and create PR

---

## 📜 License

Open source - See repository for details

---

## 🙏 Acknowledgments

- **ZOS-Server:** Evolution system and Y combinator core
- **Meta-Introspector:** Performance traces and proofs
- **Dioxus:** Rust WASM framework
- **HuggingFace:** Hosting platform

---

**Status:** Active Development  
**Last Updated:** 2026-01-27  
**Version:** 1.0.0
