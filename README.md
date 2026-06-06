# Dial Theory

A Rust library for mapping intellectual traditions across multi-dimensional dial positions.

[![Crates.io](https://img.shields.io/crates/v/dial-theory.svg)](https://crates.io/crates/dial-theory)

## What is Dial Theory?

Dial Theory is a framework for understanding how different intellectual traditions — philosophy, science, religion, political thought — relate to each other through their positions on continuous axes ("dials"). Rather than treating traditions as isolated silos, Dial Theory maps them into a shared multi-dimensional space where distances, clusters, and bridges become visible.

### The Seven Dials

Each tradition is positioned on seven continuous axes, ranging from -1 to +1:

| Axis | ← Negative Pole | Positive Pole → |
|------|-----------------|-----------------|
| **Epistemology** | Rationalist | Empiricist |
| **Social** | Individualist | Collectivist |
| **Methodology** | Reductionist | Holist |
| **Abstraction** | Abstract | Concrete |
| **Change** | Conservative | Progressive |
| **Scope** | Universalist | Relativist |
| **Reasoning** | Formalist | Intuitionist |

### Example: Dial Positions

```
Analytic Philosophy:  Epi:+0.6  Soc:-0.3  Met:-0.7  Abs:-0.8  Chg:+0.2  Sco:-0.6  Rea:-0.9
Taoism:               Epi:-0.6  Soc:+0.1  Met:+0.8  Abs: 0.0  Chg:-0.2  Sco:+0.4  Rea:+0.9
```

Analytic Philosophy leans empiricist, reductionist, abstract, and formalist — while Taoism is rationalist, holistic, and deeply intuitionist. Their distance in dial space quantifies their philosophical divergence.

## Installation

```toml
[dependencies]
dial-theory = "0.1"
```

## Quick Start

```rust
use dial_theory::tradition::prebuilt;
use dial_theory::distance;
use dial_theory::nearest;
use dial_theory::synthesis;

// Get pre-built traditions
let analytic = prebuilt::analytic_philosophy();
let pragmatism = prebuilt::pragmatism();
let taoism = prebuilt::taoism();

// Measure distance
let dist = distance::tradition_distance(&analytic, &taoism);
println!("Euclidean distance: {:.4}", dist.euclidean);
println!("Angular distance: {:.4}", dist.angular);
println!("Tension points: {:?}", dist.tension_points);

// Find nearest neighbors
let all = prebuilt::all();
let nearest = nearest::nearest_neighbors(&analytic, &all, 3);
for n in &nearest {
    println!("  {} (distance: {:.4})", n.tradition.name, n.distance.euclidean);
}

// Find bridge traditions between distant pair
let bridges = nearest::find_bridges(&analytic, &taoism, &all, 3);
for b in &bridges {
    println!("Bridge: {} (score: {:.4})", b.tradition.name, b.bridge_score);
}

// Synthesize two traditions
let result = synthesis::synthesize(&analytic, &pragmatism, "Analytic Pragmatism");
println!("Synthesis: {}", result.tradition.name);
if synthesis::has_paradoxes(&result) {
    for p in &result.paradoxes {
        println!("  Paradox: {}", p.description);
    }
}
```

## 15 Pre-built Traditions

| Tradition | Era | Character |
|-----------|-----|-----------|
| Analytic Philosophy | Early 20th c. – present | Logic, clarity, formal methods |
| Phenomenology | Early 20th c. – present | Conscious experience, intentionality |
| Pragmatism | Late 19th c. – present | Practical consequences, what works |
| Marxism | Mid 19th c. – present | Class struggle, historical materialism |
| Buddhism | 5th c. BCE – present | Suffering, impermanence, mindfulness |
| Taoism | 4th c. BCE – present | Harmony with Tao, naturalness |
| Existentialism | 19th–20th c. | Freedom, authenticity, dread |
| Structuralism | 1950s–1970s | Underlying structures, signs |
| Postmodernism | Late 20th c. – present | Anti-grand-narrative, power critique |
| Process Philosophy | Early 20th c. – present | Becoming over being |
| Confucianism | 6th c. BCE – present | Social harmony, virtue, relationships |
| Feminist Epistemology | 1980s – present | Situated knowledge, power & gender |
| Indigenous Knowledge | Prehistoric – present | Relational ontology, oral tradition |
| Systems Theory | 1940s – present | Interconnected wholes, emergence |
| Complexity Science | 1980s – present | Non-linear dynamics, self-organization |

## Modules

### `dial` — Core Types

The [`Dial`] type represents a position on a single axis with confidence. [`Axis`] enumerates the seven dimensions. [`DialWeights`] allows emphasizing certain axes in comparisons.

```rust
use dial_theory::dial::{Axis, Dial, DialWeights};

let dial = Dial::new(Axis::Epistemology, 0.6);  // Leans empiricist
let confident = Dial::with_confidence(Axis::Epistemology, 0.6, 0.8);

// Weight epistemology more than other axes
let mut weights = DialWeights::uniform();
weights.set(Axis::Epistemology, 2.0);
```

### `distance` — Tradition Distance

Computes Euclidean, angular, and Manhattan distances between traditions. Identifies alignment (which dials agree) and tension points (where traditions clash most).

### `nearest` — Nearest Neighbors & Bridges

K-nearest neighbor search in dial space. Bridge detection finds traditions that are close to both of two distant traditions — intellectual mediators.

### `synthesis` — Tradition Synthesis

Merges two traditions by interpolating dial positions. Supports weighted synthesis (one tradition contributes more). Automatically detects paradoxes where both parents have strong opposing commitments.

```rust
let result = synthesis::synthesize_weighted(&marxism, &confucianism, "Marxist-Confucian", 0.7);
// Detects paradox on Change axis: Marxism is progressive (+0.9), Confucianism conservative (-0.6)
```

### `topology` — Space Topology

Analyzes the structure of the tradition space: clusters of similar traditions, unexplored regions (holes), and boundaries (traditions closest to each pole of each axis).

```rust
use dial_theory::topology;

let topo = topology::analyze(&prebuilt::all(), 1.2);
for cluster in &topo.clusters {
    println!("Cluster: {} (diameter: {:.2})", cluster.label, cluster.diameter);
}
for hole in &topo.holes {
    println!("Gap: {:.2} from nearest tradition", hole.radius);
}
```

### `evolution` — Temporal Dynamics

Models how traditions drift over time. Supports linear interpolation of historical trajectories, projection of future positions, and splitting/merging of traditions.

```rust
use dial_theory::evolution;

// Simulate 100 years of drift
let trajectory = evolution::simulate_evolution(&pragmatism, 1900, 2000, 10, 0.05);

// Get interpolated position at any year
let pos_1950 = trajectory.position_at(Axis::Epistemology, 1950);

// Project forward
let future = trajectory.project(2050);
```

## Visualization

The tradition space can be visualized as a radar/spider chart for individual traditions:

```
         Epistemology
         R ◄─────► E
              |
Social   I ◄──┼──► C    Methodology
              |         R ◄─────► H
   ──────────┼──────────
         Abstraction
         A ◄─────► C
              |
  Change  C ◄──┼──► P    Scope
              |         U ◄─────► R
         Reasoning
         F ◄─────► I
```

Or as a 2D projection (e.g., Epistemology × Methodology):

```
         Holist
           ↑
    Taoism •        Buddhism •
                         Systems Theory •
  Confucianism •               Complexity Science •
                     Phenomenology •
  Marxism •
  Feminist Ep •
                     Pragmatism •
  Postmodernism •    
  Existentialism •   Analytic Philosophy •
           • Structuralism
           ←────────────────────────────────→
         Rationalist                  Empiricist
```

## Distance Metrics

| Metric | Formula | Range | Use Case |
|--------|---------|-------|----------|
| Euclidean | √(Σ(pos_a - pos_b)²) | [0, √28] ≈ [0, 5.29] | Overall similarity |
| Angular | arccos(cos_sim) | [0, π] | Direction matters more than magnitude |
| Manhattan | Σ|pos_a - pos_b| | [0, 14] | Robust to outliers |

## License

MIT
