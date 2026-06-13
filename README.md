# Dial Theory

**A framework for mapping intellectual traditions onto a seven-dimensional dial space** — quantifying philosophical positions as coordinates and computing distances, clusters, and synthesis potential between traditions. It turns qualitative "schools of thought" into geometric data that can be measured, clustered, and visualized.

## Why It Matters

The humanities struggle with a fundamental problem: how do you compare intellectual traditions objectively? "How close is Pragmatism to Analytic Philosophy?" is usually answered with prose, not numbers. Dial Theory provides a quantitative alternative: position each tradition on seven continuous axes (epistemology, social organization, methodology, etc.) and use geometric distance to measure similarity.

This approach has real applications in **computational humanities**, **education** (visualizing how ideas relate), **AI alignment** (mapping different safety frameworks), and **interdisciplinary research** (finding traditions that bridge disciplines). The framework supports:

- **Distance computation** — Euclidean, angular, and Manhattan distances in 7D dial space
- **Nearest-neighbor search** — Find the k most similar traditions to any query
- **Bridge detection** — Find traditions that mediate between distant schools of thought
- **Synthesis with paradox detection** — Merge two traditions and identify where their core commitments conflict
- **Evolution simulation** — Model how traditions drift, split, and merge over historical time
- **Topology analysis** — Find clusters, sparse regions ("holes"), and boundary traditions

## How It Works

**Seven Axes:** Each tradition is positioned on seven orthogonal dimensions:
1. **Epistemology** — Rationalist (−1) ↔ Empiricist (+1)
2. **Social** — Individualist (−1) ↔ Collectivist (+1)
3. **Methodology** — Reductionist (−1) ↔ Holist (+1)
4. **Abstraction** — Abstract (−1) ↔ Concrete (+1)
5. **Change** — Conservative (−1) ↔ Progressive (+1)
6. **Scope** — Universalist (−1) ↔ Relativist (+1)
7. **Reasoning** — Formalist (−1) ↔ Intuitionist (+1)

A `Dial` is a single axis-position pair with an optional confidence value (0.0–1.0). A `Tradition` is a collection of seven dials, forming a point in 7D space.

**Distance metrics:** The primary distance is Euclidean (L2) across all seven axes, with optional per-axis weights. Angular distance (cosine similarity) captures directional alignment regardless of magnitude. The tension points analysis identifies which specific axes contribute most to the distance between two traditions.

**Synthesis and paradox detection:** When synthesizing two traditions (weighted average of positions), the framework detects paradoxes — axes where both parents have strong (>0.5) opposing positions. For example, synthesizing Marxism (Change=+0.9) with Confucianism (Change=−0.6) produces a paradox on the Change axis because both traditions have strong, opposite commitments.

**Topology analysis:** Single-linkage clustering with union-find groups traditions within a distance threshold. Random sampling in the 7D hypercube identifies "holes" — regions of the tradition space with no existing tradition nearby, suggesting unexplored intellectual territory.

## Quick Start

```rust
use dial_theory::tradition::prebuilt;
use dial_theory::distance;
use dial_theory::nearest;
use dial_theory::synthesis;

// Compare two traditions
let analytic = prebuilt::analytic_philosophy();
let pragmatism = prebuilt::pragmatism();
let dist = distance::tradition_distance(&analytic, &pragmatism);
println!("Euclidean distance: {:.4}", dist.euclidean);
println!("Tension axes: {:?}", dist.tension_points);

// Find nearest neighbors
let all = prebuilt::all();
let nn = nearest::nearest_neighbors(&analytic, &all, 3);
for n in &nn {
    println!("{}: {:.4}", n.tradition.name, n.distance.euclidean);
}

// Synthesize two traditions
let result = synthesis::synthesize(&analytic, &pragmatism, "Analytic Pragmatism");
println!("Paradoxes: {}", result.paradoxes.len());
```

## API

### Core Types
- `Axis` — Enum of the seven dimensions (Epistemology, Social, Methodology, Abstraction, Change, Scope, Reasoning)
- `Dial` — Position (−1..1) and confidence (0..1) on a single axis
- `Tradition` — Named collection of 7 dials with description and era
- `DialWeights` — Per-axis weights for emphasizing dimensions in distance calculations

### Modules
- `distance` — `tradition_distance()`, `euclidean_distance()`, `angular_distance()` with tension point analysis
- `nearest` — `nearest_neighbors()` (k-NN search), `find_bridges()` (mediating traditions)
- `synthesis` — `synthesize()`, `synthesize_weighted()`, paradox detection
- `evolution` — `apply_drift()`, `simulate_evolution()`, `split()`, trajectory projection
- `topology` — `analyze()`, `find_clusters()`, `find_holes()`, `find_boundaries()`

### Pre-built Traditions
15 calibrated traditions: Analytic Philosophy, Phenomenology, Pragmatism, Marxism, Buddhism, Taoism, Existentialism, Structuralism, Postmodernism, Process Philosophy, Confucianism, Feminist Epistemology, Indigenous Knowledge Systems, Systems Theory, Complexity Science

## Architecture Notes

Dial Theory is a research and visualization tool within SuperInstance's knowledge graph. It models intellectual relationships as geometric data, enabling quantitative analysis of philosophical landscapes and educational tools for exploring how ideas connect.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## License

MIT
