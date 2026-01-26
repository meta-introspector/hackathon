# Project Provenance - Game Design Document

**A Rust+WASM Adventure in Data Provenance**

---

## Quick Start

```bash
# Enter Nix development environment
nix develop

# Run locally
wrangler dev

# Deploy to Cloudflare
wrangler deploy
```

---

## Core Mechanics (Y Combinator Based)

### 1. Creature Genealogy (Dynamic Provenance)
```rust
// Each creature is a fixed point of its history
mkycombinator!("DataCreature") => Creature(Creature)

// Trace provenance chain
trace_lineage!(creature) => Assembly -> C -> Scheme -> Seed
```

### 2. Essence Crafting (Contextual Compression)
```rust
// Compress syntactic matter into semantic essence
craft!(SyntacticMatter, CosmicConstant) => SemanticEssence

// Fixed point crafting
mkycombinator!("Craft") => Craft(Craft)
```

### 3. Chrono-Vision (Canonical State)
```rust
// Time manipulation as fixed point
mkycombinator!("ChronoVision") => Past(Future(Present))

// Predict outcomes
predict!(action) => ghost_outcome
```

---

## Integration with Existing Systems

### From TradeWars 3033
- **Perf traces** → Data Creature origins
- **Rust blocks** → Semantic Essences
- **8D navigation** → Provenance chain exploration
- **Factories** → Crafting stations

### From ZOS-Server
- **Y combinator core** → All game mechanics
- **Complexity guards** → Puzzle difficulty
- **Security lattice** → LMFDB convergence puzzles

---

## File Structure

```
hackathon/
├── game/
│   ├── index.html          # 8D visualization
│   └── api.js              # Perf traces API
├── provenance/
│   ├── creatures.rs        # Data Creature system
│   ├── crafting.rs         # Essence crafting
│   ├── chrono.rs           # Time manipulation
│   └── genealogy.rs        # Provenance tracing
├── tradewars_ycombinator.rs # Y combinator core
├── wrangler.toml           # Cloudflare config
└── flake.nix               # Nix environment
```

---

## Gameplay Loop

```
1. Discover Data Creature (perf trace)
   ↓
2. Trace Genealogy (provenance chain)
   ↓
3. Unlock Cosmic Constant (recipe)
   ↓
4. Gather Syntactic Matter (resources)
   ↓
5. Craft Semantic Essence (compression)
   ↓
6. Power Chrono-Vision (prediction)
   ↓
7. Solve Convergence Puzzle (LMFDB)
   ↓
8. Purify Glitched Creature
   ↓
9. Reconstruct First Witness
```

---

## Technical Foundation

### Rust + WASM
- **Performance**: Native speed in browser
- **Safety**: Memory-safe game logic
- **Accessibility**: No installation required

### Y Combinator Core
```rust
// All mechanics are fixed points
Game(Game) = Game
Creature(Creature) = Creature
Craft(Craft) = Craft
Time(Time) = Time
```

### Cloudflare Workers
- **Global deployment**: Low latency worldwide
- **Serverless**: No infrastructure management
- **KV storage**: Persistent game state

---

## Educational Goals

1. **Provenance**: Every object has verifiable history
2. **Compression**: Context transforms chaos to order
3. **Prediction**: Past patterns reveal future states
4. **Systems Thinking**: Simple rules → complex emergence

---

## Next Steps

### Phase 1: Core Prototype (Week 1)
- [ ] Implement Creature Genealogy puzzle
- [ ] Basic provenance chain visualization
- [ ] Y combinator integration

### Phase 2: Crafting System (Week 2)
- [ ] Syntactic Matter gathering
- [ ] Cosmic Constant discovery
- [ ] Essence compression mechanic

### Phase 3: Chrono-Vision (Week 3)
- [ ] Time rewind functionality
- [ ] Outcome prediction
- [ ] Ghost visualization

### Phase 4: World System (Week 4)
- [ ] Adaptive environment
- [ ] Symbolic pointers
- [ ] Heat-based materialization

### Phase 5: Validation (Week 5)
- [ ] LMFDB convergence puzzles
- [ ] Glitched creature purification
- [ ] First Witness reconstruction

---

## Deployment

```bash
# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Deploy to Cloudflare
wrangler deploy

# Access at
https://provenance.jmikedupont2.workers.dev
```

---

## References

- **Source Architecture**: meta-introspector/HOMOMORPHIC_HOMOTOPY.md
- **Y Combinator**: nix-controller/ycombinator_macro.rs
- **TradeWars**: hackathon/tradewars_ycombinator.rs
- **ZOS Core**: zos-server/src/core.rs

---

**Status**: Design Complete → Ready for Implementation  
**Target**: Children aged 10-14  
**Platform**: Browser (Rust + WASM + Cloudflare)
