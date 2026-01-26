"""
Project Provenance - Hackathon Games
Educational games for HuggingFace Spaces
"""
import gradio as gr

def create_game_interface():
    with gr.Blocks(title="Project Provenance - Hackathon", theme=gr.themes.Soft()) as demo:
        gr.Markdown("""
        # 🎮 Project Provenance - Hackathon Games
        **Educational games teaching data provenance and systems thinking**
        """)
        
        with gr.Tabs():
            with gr.Tab("🚀 8D Perf Game"):
                gr.Markdown("""
                ### Navigate Performance Traces in 8D Space
                
                **Controls:**
                - WASD: X/Y dimensions
                - QE: Z dimension
                - RF, TG, YH, UJ, IK: Dimensions 4-8
                
                Each emoji particle represents a real performance trace.
                """)
                
                gr.HTML("""
                <iframe src="game/index.html" width="100%" height="600px" frameborder="0"></iframe>
                """)
            
            with gr.Tab("🌌 TradeWars 3033"):
                gr.Markdown("""
                ### Space Trading with Y Combinator Mechanics
                
                **Concept:** Each game element is a fixed point
                - Game(Game) = Game
                - Trade(Trade) = Trade
                - Navigate(Navigate) = Navigate
                
                Trade performance optimizations between star systems!
                """)
                
                gr.Code("""
// Y Combinator game loop
mkycombinator!("TradeWars3033")

// Trade as fixed point
trade!("RustBlock", 2)

// Navigate between sectors
navigate!(1, 2)
                """, language="rust")
            
            with gr.Tab("🧬 Project Provenance"):
                gr.Markdown("""
                ### Educational Adventure in Data Genealogy
                
                **Learn through play:**
                1. **Creature Genealogy** - Trace data provenance
                2. **Essence Crafting** - Compress chaos to order
                3. **Chrono-Vision** - Predict and rewind time
                
                Discover the First Witness and unlock the universe's source code!
                """)
                
                with gr.Row():
                    gr.Markdown("""
                    #### Creature Genealogy
                    Trace the lineage of Data Creatures:
                    ```
                    Assembly → C → Scheme → Seed
                    ```
                    """)
                    
                    gr.Markdown("""
                    #### Essence Crafting
                    Transform resources:
                    ```
                    Syntactic Matter + Cosmic Constant
                    → Semantic Essence
                    ```
                    """)
                    
                    gr.Markdown("""
                    #### Chrono-Vision
                    Manipulate time:
                    ```
                    Rewind | Predict | Insight
                    ```
                    """)
            
            with gr.Tab("📚 Documentation"):
                gr.Markdown("""
                ### Game Documentation
                
                **Design Documents:**
                - [Project Provenance](https://github.com/meta-introspector/hackathon/blob/main/PROJECT_PROVENANCE.md)
                - [TradeWars Ecosystem](https://github.com/meta-introspector/meta-meme/blob/unified-memes/TRADEWARS_ECOSYSTEM.md)
                - [Games Integration](https://github.com/meta-introspector/hackathon/blob/main/GAMES_INTEGRATION.md)
                
                **Technical:**
                - [Cloudflare SOP](https://github.com/meta-introspector/meta-meme/blob/unified-memes/CLOUDFLARE_WORKERS_BUILD_SOP.md)
                - [HuggingFace SOP](https://github.com/meta-introspector/hackathon/blob/main/HUGGINGFACE_SPACES_SOP.md)
                
                **Source Code:**
                - [GitHub Repository](https://github.com/meta-introspector/hackathon)
                - [Y Combinator Core](https://github.com/meta-introspector/hackathon/blob/main/tradewars_ycombinator.rs)
                """)
        
        gr.Markdown("""
        ---
        ### 🎯 Educational Goals
        
        - **Provenance**: Every object has verifiable history
        - **Compression**: Context transforms chaos to order
        - **Prediction**: Past patterns reveal future states
        - **Systems Thinking**: Simple rules → complex emergence
        
        ### 🚀 Deployment
        
        - **HuggingFace Spaces**: https://huggingface.co/spaces/introspector/hackathon
        - **Cloudflare Workers**: https://provenance.jmikedupont2.workers.dev
        - **GitHub**: https://github.com/meta-introspector/hackathon
        """)
    
    return demo

if __name__ == "__main__":
    demo = create_game_interface()
    demo.launch()
