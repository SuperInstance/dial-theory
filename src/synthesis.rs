//! Synthesis of traditions: merging dial positions.
//!
//! Supports weighted synthesis, averaging, and paradox detection
//! when syntheses violate core commitments of both parents.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dial::Axis;
use crate::tradition::Tradition;

/// Result of synthesizing two traditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisResult {
    pub tradition: Tradition,
    /// Which axes produced paradoxes (both parents strongly disagree and synthesis compromises both).
    pub paradoxes: Vec<Paradox>,
    /// Per-axis agreement between parents (-1 = full opposition, 1 = full agreement).
    pub parental_agreement: HashMap<Axis, f64>,
}

/// A detected paradox in a synthesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paradox {
    pub axis: Axis,
    pub parent_a_position: f64,
    pub parent_b_position: f64,
    pub synthesized_position: f64,
    /// How strongly both parents are committed to their positions (0-1).
    pub conflict_intensity: f64,
    pub description: String,
}

/// Synthesize two traditions by averaging their dial positions.
pub fn synthesize(a: &Tradition, b: &Tradition, name: &str) -> SynthesisResult {
    synthesize_weighted(a, b, name, 0.5)
}

/// Synthesize with a weight bias (0.0 = all A, 1.0 = all B).
pub fn synthesize_weighted(a: &Tradition, b: &Tradition, name: &str, weight_b: f64) -> SynthesisResult {
    let weight_b = weight_b.clamp(0.0, 1.0);
    let weight_a = 1.0 - weight_b;

    let mut paradoxes = Vec::new();
    let mut agreement = HashMap::new();

    let dials: Vec<_> = Axis::all()
        .iter()
        .map(|&axis| {
            let pa = a.position(axis);
            let pb = b.position(axis);
            let pos = pa * weight_a + pb * weight_b;
            let conf = a.confidence(axis) * weight_a + b.confidence(axis) * weight_b;

            // Agreement: product of positions (negative = opposed)
            let agree = if pa == 0.0 && pb == 0.0 {
                1.0
            } else {
                let norm = (pa * pa + pb * pb).sqrt();
                if norm > 0.0 {
                    (pa * pb) / (norm * norm) * 2.0
                } else {
                    1.0
                }
            };
            agreement.insert(axis, agree);

            // Paradox detection: both parents have strong opposing positions
            let threshold = 0.5;
            if pa.abs() > threshold && pb.abs() > threshold && pa * pb < 0.0 {
                let intensity = (pa.abs().min(pb.abs()) - threshold) / (1.0 - threshold);
                paradoxes.push(Paradox {
                    axis,
                    parent_a_position: pa,
                    parent_b_position: pb,
                    synthesized_position: pos,
                    conflict_intensity: intensity.clamp(0.0, 1.0),
                    description: format!(
                        "Parents conflict on {}: {} favors {} ({:.1}), {} favors {} ({:.1})",
                        axis.label(),
                        a.name,
                        if pa < 0.0 { axis.negative_pole() } else { axis.positive_pole() },
                        pa,
                        b.name,
                        if pb < 0.0 { axis.negative_pole() } else { axis.positive_pole() },
                        pb,
                    ),
                });
            }

            crate::dial::Dial::with_confidence(axis, pos, conf)
        })
        .collect();

    let description = format!(
        "Synthesis of {} ({:.0}%) and {} ({:.0}%)",
        a.name,
        weight_a * 100.0,
        b.name,
        weight_b * 100.0,
    );

    let tradition = Tradition::new(name, dials, &description);

    SynthesisResult {
        tradition,
        paradoxes,
        parental_agreement: agreement,
    }
}

/// Check if a synthesis has any paradoxes.
pub fn has_paradoxes(result: &SynthesisResult) -> bool {
    !result.paradoxes.is_empty()
}

/// Get the paradox count.
pub fn paradox_count(result: &SynthesisResult) -> usize {
    result.paradoxes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tradition::prebuilt;
    use approx::assert_relative_eq;

    #[test]
    fn test_synthesis_averages_positions() {
        let a = prebuilt::analytic_philosophy();
        let b = prebuilt::pragmatism();
        let result = synthesize(&a, &b, "Analytic Pragmatism");

        for &axis in &Axis::all() {
            let expected = (a.position(axis) + b.position(axis)) / 2.0;
            assert_relative_eq!(
                result.tradition.position(axis),
                expected,
                epsilon = 1e-10
            );
        }
    }

    #[test]
    fn test_weighted_synthesis() {
        let a = prebuilt::analytic_philosophy();
        let b = prebuilt::pragmatism();
        let result = synthesize_weighted(&a, &b, "Mostly Analytic", 0.0);

        for &axis in &Axis::all() {
            assert_relative_eq!(
                result.tradition.position(axis),
                a.position(axis),
                epsilon = 1e-10
            );
        }
    }

    #[test]
    fn test_paradox_detection() {
        // Marxism (progressive 0.9) vs Confucianism (conservative -0.6) should conflict on Change
        let marx = prebuilt::marxism();
        let confuc = prebuilt::confucianism();
        let result = synthesize(&marx, &confuc, "Marxist-Confucian");

        assert!(has_paradoxes(&result), "Expected paradoxes between Marxism and Confucianism");
        let change_paradox = result.paradoxes.iter().find(|p| p.axis == Axis::Change);
        assert!(change_paradox.is_some(), "Expected Change axis paradox");
    }

    #[test]
    fn test_no_paradox_for_similar_traditions() {
        let a = prebuilt::systems_theory();
        let b = prebuilt::complexity_science();
        let result = synthesize(&a, &b, "Systems-Complexity");

        assert_eq!(paradox_count(&result), 0, "Similar traditions should not produce paradoxes");
    }

    #[test]
    fn test_parental_agreement_range() {
        let result = synthesize(&prebuilt::analytic_philosophy(), &prebuilt::taoism(), "Test");
        for &v in result.parental_agreement.values() {
            assert!(v >= -1.01 && v <= 1.01, "Agreement {} out of range", v);
        }
    }
}
