//! Meme Breeding System - Genetic Evolution of Data Creatures
//! Based on evolution_server.rs + Y Combinator genetics

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// A Meme is a self-replicating data creature
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Meme {
    id: String,
    genome: Genome,
    fitness: f64,
    generation: u32,
    parents: Vec<String>,
    traits: Traits,
}

/// Genome encodes the meme's behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Genome {
    combinator: String,      // Y, S, K, I combinators
    perf_trace: PerfTrace,   // Performance DNA
    rust_blocks: Vec<String>, // Code genes
    emoji: String,           // Visual phenotype
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerfTrace {
    cycles: u64,
    weight: u64,
    resonates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Traits {
    speed: f64,      // Low cycles = high speed
    stability: f64,  // Resonance = stability
    complexity: f64, // Weight = complexity
}

/// Breeding pool manages meme evolution
struct BreedingPool {
    memes: HashMap<String, Meme>,
    generation: u32,
    fitness_threshold: f64,
}

impl BreedingPool {
    fn new() -> Self {
        Self {
            memes: HashMap::new(),
            generation: 0,
            fitness_threshold: 0.5,
        }
    }
    
    /// Create initial population from perf traces
    fn seed(&mut self, traces: Vec<PerfTrace>) {
        for (i, trace) in traces.iter().enumerate() {
            let meme = Meme {
                id: format!("meme_{}", i),
                genome: Genome {
                    combinator: "Y".to_string(),
                    perf_trace: trace.clone(),
                    rust_blocks: vec!["Memory".to_string()],
                    emoji: classify_emoji(trace.cycles),
                },
                fitness: calculate_fitness(trace),
                generation: 0,
                parents: vec![],
                traits: Traits {
                    speed: 1.0 / (trace.cycles as f64),
                    stability: if trace.resonates { 1.0 } else { 0.0 },
                    complexity: trace.weight as f64 / 196883.0,
                },
            };
            self.memes.insert(meme.id.clone(), meme);
        }
    }
    
    /// Select parents based on fitness
    fn select_parents(&self) -> Vec<&Meme> {
        let mut sorted: Vec<&Meme> = self.memes.values()
            .filter(|m| m.fitness > self.fitness_threshold)
            .collect();
        sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        sorted.into_iter().take(2).collect()
    }
    
    /// Breed two memes using Y combinator genetics
    fn breed(&mut self, parent1: &Meme, parent2: &Meme) -> Meme {
        self.generation += 1;
        
        // Crossover: combine genomes
        let child_genome = Genome {
            combinator: format!("{}({})", parent1.genome.combinator, parent2.genome.combinator),
            perf_trace: PerfTrace {
                cycles: (parent1.genome.perf_trace.cycles + parent2.genome.perf_trace.cycles) / 2,
                weight: (parent1.genome.perf_trace.weight + parent2.genome.perf_trace.weight) / 2,
                resonates: parent1.genome.perf_trace.resonates && parent2.genome.perf_trace.resonates,
            },
            rust_blocks: [
                parent1.genome.rust_blocks.clone(),
                parent2.genome.rust_blocks.clone(),
            ].concat(),
            emoji: if parent1.fitness > parent2.fitness {
                parent1.genome.emoji.clone()
            } else {
                parent2.genome.emoji.clone()
            },
        };
        
        // Mutation: random variation
        let mutated_genome = self.mutate(child_genome);
        
        let child_fitness = (parent1.fitness + parent2.fitness) / 2.0 * 1.1; // Hybrid vigor
        
        Meme {
            id: format!("meme_gen{}_child", self.generation),
            genome: mutated_genome.clone(),
            fitness: child_fitness,
            generation: self.generation,
            parents: vec![parent1.id.clone(), parent2.id.clone()],
            traits: Traits {
                speed: 1.0 / (mutated_genome.perf_trace.cycles as f64),
                stability: if mutated_genome.perf_trace.resonates { 1.0 } else { 0.5 },
                complexity: mutated_genome.perf_trace.weight as f64 / 196883.0,
            },
        }
    }
    
    /// Mutate genome with small random changes
    fn mutate(&self, mut genome: Genome) -> Genome {
        // 10% chance to mutate cycles
        if rand::random::<f64>() < 0.1 {
            genome.perf_trace.cycles = (genome.perf_trace.cycles as f64 * 0.9) as u64;
        }
        
        // 5% chance to gain resonance
        if rand::random::<f64>() < 0.05 {
            genome.perf_trace.resonates = true;
        }
        
        // Update emoji based on new cycles
        genome.emoji = classify_emoji(genome.perf_trace.cycles);
        
        genome
    }
    
    /// Evolve population for one generation
    fn evolve(&mut self) {
        let parents = self.select_parents();
        if parents.len() < 2 {
            return;
        }
        
        let child = self.breed(parents[0], parents[1]);
        println!("🧬 Generation {}: Bred {} from {} + {}", 
                 self.generation,
                 child.id,
                 parents[0].id,
                 parents[1].id);
        println!("   Fitness: {:.3} → {:.3}", 
                 (parents[0].fitness + parents[1].fitness) / 2.0,
                 child.fitness);
        println!("   Emoji: {} + {} → {}", 
                 parents[0].genome.emoji,
                 parents[1].genome.emoji,
                 child.genome.emoji);
        
        self.memes.insert(child.id.clone(), child);
    }
    
    /// Run evolution until convergence
    fn run(&mut self, max_generations: u32) {
        println!("🧬 Meme Breeding System Starting");
        println!("   Initial population: {}", self.memes.len());
        println!("   Max generations: {}", max_generations);
        println!();
        
        for _ in 0..max_generations {
            self.evolve();
            
            // Check for super-meme (fitness > 0.95)
            if let Some(best) = self.memes.values().max_by(|a, b| 
                a.fitness.partial_cmp(&b.fitness).unwrap()
            ) {
                if best.fitness > 0.95 {
                    println!("\n🎯 SUPER-MEME EVOLVED!");
                    println!("   ID: {}", best.id);
                    println!("   Fitness: {:.3}", best.fitness);
                    println!("   Emoji: {}", best.genome.emoji);
                    println!("   Generation: {}", best.generation);
                    break;
                }
            }
        }
        
        println!("\n🏁 Evolution Complete!");
        println!("   Final population: {}", self.memes.len());
        println!("   Generations: {}", self.generation);
    }
}

/// Calculate fitness from performance trace
fn calculate_fitness(trace: &PerfTrace) -> f64 {
    let speed_score = 1.0 / (trace.cycles as f64 / 1000.0);
    let stability_score = if trace.resonates { 1.0 } else { 0.5 };
    let complexity_score = 1.0 - (trace.weight as f64 / 196883.0);
    
    (speed_score + stability_score + complexity_score) / 3.0
}

/// Classify emoji based on cycles
fn classify_emoji(cycles: u64) -> String {
    match cycles {
        0..=3000 => "⚡".to_string(),
        3001..=5000 => "🚀".to_string(),
        5001..=7000 => "🔥".to_string(),
        7001..=10000 => "💎".to_string(),
        10001..=50000 => "🌊".to_string(),
        _ => "🌀".to_string(),
    }
}

/// Y Combinator breeding: Meme(Meme) = Meme
#[macro_export]
macro_rules! breed_meme {
    ($parent1:expr, $parent2:expr) => {{
        mkycombinator!(format!("Breed({}, {})", $parent1, $parent2))
    }};
}

fn main() {
    println!("🧬 Meme Breeding System - Y Combinator Genetics\n");
    
    // Create initial population from perf traces
    let traces = vec![
        PerfTrace { cycles: 35187, weight: 35187, resonates: false },
        PerfTrace { cycles: 5204, weight: 5204, resonates: true },
        PerfTrace { cycles: 6685, weight: 6685, resonates: true },
        PerfTrace { cycles: 468, weight: 468, resonates: true },
    ];
    
    let mut pool = BreedingPool::new();
    pool.seed(traces);
    
    // Evolve for 100 generations
    pool.run(100);
    
    println!("\n🎯 Breeding as Fixed Point:");
    println!("   Breed(Meme, Meme) = Meme");
    println!("   Y(Evolution) = Evolution(Evolution)");
    println!("   The memes breed themselves!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fitness_calculation() {
        let trace = PerfTrace {
            cycles: 5000,
            weight: 5000,
            resonates: true,
        };
        let fitness = calculate_fitness(&trace);
        assert!(fitness > 0.0 && fitness <= 1.0);
    }
    
    #[test]
    fn test_emoji_classification() {
        assert_eq!(classify_emoji(468), "⚡");
        assert_eq!(classify_emoji(5204), "🔥");
        assert_eq!(classify_emoji(35187), "🌀");
    }
}
