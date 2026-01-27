//! Combinator Universe - Minimal Dioxus WASM Game

use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    let mut count = use_signal(|| 0);
    let mut memes = use_signal(|| vec!["⚡", "🔥", "🌀"]);

    rsx! {
        div {
            style: "font-family: monospace; background: #0a0a0a; color: #00ff00; padding: 20px;",
            
            h1 { "🌌 Combinator Universe" }
            p { "Navigate 8D hyperspace • Breed memes • Explore math" }
            
            h2 { "Generation: {count}" }
            
            div {
                style: "display: flex; gap: 20px; margin: 20px 0;",
                for emoji in memes.read().iter() {
                    div {
                        style: "border: 2px solid #00ff00; padding: 20px; font-size: 3em; cursor: pointer;",
                        onclick: move |_| {
                            count += 1;
                        },
                        "{emoji}"
                    }
                }
            }
            
            button {
                style: "background: #00ff00; color: #000; padding: 10px 20px; border: none; cursor: pointer; font-weight: bold;",
                onclick: move |_| {
                    count += 1;
                    memes.write().push("🚀");
                },
                "🧬 Breed Meme"
            }
            
            p { "mkycombinator!(Universe) => Universe(Universe)" }
        }
    }
}
