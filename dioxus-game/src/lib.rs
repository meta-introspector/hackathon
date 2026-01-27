//! Combinator Universe - Dioxus WASM Game
//! One game: Explore 8D hyperspace using Y combinators

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Meme {
    id: String,
    emoji: String,
    cycles: u64,
    position: [f32; 8],
    fitness: f64,
}

#[derive(Clone, Debug)]
struct GameState {
    memes: Vec<Meme>,
    player_pos: [f32; 8],
    generation: u32,
    selected: Option<usize>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            memes: vec![
                Meme {
                    id: "meme_0".to_string(),
                    emoji: "⚡".to_string(),
                    cycles: 468,
                    position: [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.1],
                    fitness: 0.95,
                },
                Meme {
                    id: "meme_1".to_string(),
                    emoji: "🔥".to_string(),
                    cycles: 5204,
                    position: [1.0, 1.0, 0.0, 1.0, 0.2, 0.0, 1.0, 0.3],
                    fitness: 0.85,
                },
                Meme {
                    id: "meme_2".to_string(),
                    emoji: "🌀".to_string(),
                    cycles: 35187,
                    position: [2.0, 2.0, 1.0, 0.0, 0.5, 0.0, 1.0, 0.8],
                    fitness: 0.45,
                },
            ],
            player_pos: [0.0; 8],
            generation: 0,
            selected: None,
        }
    }
}

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let state = use_state(cx, GameState::default);

    cx.render(rsx! {
        style { include_str!("../style.css") }
        
        div {
            class: "game-container",
            
            // Header
            div {
                class: "header",
                h1 { "🌌 Combinator Universe" }
                p { "Navigate 8D hyperspace • Breed memes • Explore math" }
            }
            
            // Stats
            div {
                class: "stats",
                div { "Generation: {state.generation}" }
                div { "Memes: {state.memes.len()}" }
                div { "Position: [{state.player_pos[0]:.1}, {state.player_pos[1]:.1}, {state.player_pos[2]:.1}, ...]" }
            }
            
            // Game view
            div {
                class: "game-view",
                
                // Meme grid
                div {
                    class: "meme-grid",
                    for (i, meme) in state.memes.iter().enumerate() {
                        div {
                            class: "meme-card",
                            onclick: move |_| {
                                state.modify(|s| s.selected = Some(i));
                            },
                            
                            div { class: "meme-emoji", "{meme.emoji}" }
                            div { class: "meme-id", "{meme.id}" }
                            div { class: "meme-cycles", "{meme.cycles} cycles" }
                            div { class: "meme-fitness", "Fitness: {meme.fitness:.2}" }
                        }
                    }
                }
                
                // Selected meme details
                if let Some(idx) = state.selected {
                    if let Some(meme) = state.memes.get(idx) {
                        rsx! {
                            div {
                                class: "meme-details",
                                h3 { "Selected: {meme.emoji} {meme.id}" }
                                p { "Cycles: {meme.cycles}" }
                                p { "Fitness: {meme.fitness:.3}" }
                                p { "Position: [{meme.position[0]:.2}, {meme.position[1]:.2}, {meme.position[2]:.2}, ...]" }
                                
                                button {
                                    onclick: move |_| {
                                        state.modify(|s| {
                                            // Navigate to meme
                                            if let Some(m) = s.memes.get(idx) {
                                                s.player_pos = m.position;
                                            }
                                        });
                                    },
                                    "Navigate Here"
                                }
                                
                                button {
                                    onclick: move |_| {
                                        state.modify(|s| s.selected = None);
                                    },
                                    "Close"
                                }
                            }
                        }
                    }
                }
            }
            
            // Controls
            div {
                class: "controls",
                h3 { "Actions" }
                
                button {
                    onclick: move |_| {
                        state.modify(|s| {
                            // Breed two fittest memes
                            if s.memes.len() >= 2 {
                                let mut sorted = s.memes.clone();
                                sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
                                
                                let parent1 = &sorted[0];
                                let parent2 = &sorted[1];
                                
                                let child = Meme {
                                    id: format!("meme_gen{}", s.generation),
                                    emoji: if parent1.fitness > parent2.fitness { parent1.emoji.clone() } else { parent2.emoji.clone() },
                                    cycles: (parent1.cycles + parent2.cycles) / 2,
                                    position: [
                                        (parent1.position[0] + parent2.position[0]) / 2.0,
                                        (parent1.position[1] + parent2.position[1]) / 2.0,
                                        (parent1.position[2] + parent2.position[2]) / 2.0,
                                        (parent1.position[3] + parent2.position[3]) / 2.0,
                                        (parent1.position[4] + parent2.position[4]) / 2.0,
                                        (parent1.position[5] + parent2.position[5]) / 2.0,
                                        (parent1.position[6] + parent2.position[6]) / 2.0,
                                        (parent1.position[7] + parent2.position[7]) / 2.0,
                                    ],
                                    fitness: (parent1.fitness + parent2.fitness) / 2.0 * 1.1,
                                };
                                
                                s.memes.push(child);
                                s.generation += 1;
                            }
                        });
                    },
                    "🧬 Breed Memes"
                }
                
                button {
                    onclick: move |_| {
                        state.modify(|s| {
                            s.player_pos[0] += 0.5;
                        });
                    },
                    "→ Move X+"
                }
                
                button {
                    onclick: move |_| {
                        state.modify(|s| {
                            s.player_pos[1] += 0.5;
                        });
                    },
                    "↑ Move Y+"
                }
                
                button {
                    onclick: move |_| {
                        state.modify(|s| {
                            s.player_pos[2] += 0.5;
                        });
                    },
                    "⬆ Move Z+"
                }
            }
            
            // Info
            div {
                class: "info",
                h3 { "Y Combinator Universe" }
                p { "mkycombinator!(Universe) => Universe(Universe)" }
                p { "Each meme is a fixed point in 8D space" }
                p { "Breed memes to evolve better performance" }
            }
        }
    })
}
