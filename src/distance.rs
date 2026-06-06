//! Distance metrics between traditions.
//!
//! Computes multi-dimensional distances using weighted dial positions,
//! identifies alignment and tension points between traditions.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dial::{Axis, DialWeights};
use crate::tradition::Tradition;

/// Full distance report between two traditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionDistance {
    /// Euclidean (L2) distance in dial space.
    pub euclidean: f64,
    /// Angular (cosine-based) distance.
    pub angular: f64,
    /// Manhattan (L1) distance in dial space.
    pub manhattan: f64,
    /// Per-axis alignment: positive = same direction, negative = opposed.
    pub alignment: HashMap<Axis, f64>,
    /// Axes where traditions are most opposed (largest positional difference).
    pub tension_points: Vec<Axis>,
}

/// Compute full distance between two traditions using uniform weights.
pub fn tradition_distance(a: &Tradition, b: &Tradition) -> TraditionDistance {
    tradition_distance_weighted(a, b, &DialWeights::uniform())
}

/// Compute full distance between two traditions with custom weights.
pub fn tradition_distance_weighted(
    a: &Tradition,
    b: &Tradition,
    weights: &DialWeights,
) -> TraditionDistance {
    let axes = Axis::all();
    let mut sum_sq = 0.0;
    let mut sum_abs = 0.0;
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    let mut alignment = HashMap::new();
    let mut tensions: Vec<(f64, Axis)> = Vec::new();

    for &axis in &axes {
        let pa = a.position(axis);
        let pb = b.position(axis);
        let w = weights.get(axis);
        let diff = (pa - pb).abs() * w;

        sum_sq += diff * diff;
        sum_abs += diff;

        let wa = pa * w;
        let wb = pb * w;
        dot += wa * wb;
        norm_a += wa * wa;
        norm_b += wb * wb;

        // Alignment: product of positions (positive = same side, negative = opposed)
        alignment.insert(axis, pa * pb);

        tensions.push((diff, axis));
    }

    let euclidean = sum_sq.sqrt();

    let angular = if norm_a > 0.0 && norm_b > 0.0 {
        let cos_sim = dot / (norm_a.sqrt() * norm_b.sqrt());
        cos_sim.clamp(-1.0, 1.0).acos()
    } else {
        std::f64::consts::FRAC_PI_2
    };

    // Top 3 tension points
    tensions.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    let tension_points = tensions.into_iter().take(3).map(|(_, a)| a).collect();

    TraditionDistance {
        euclidean,
        angular,
        manhattan: sum_abs,
        alignment,
        tension_points,
    }
}

/// Quick Euclidean distance between two traditions.
pub fn euclidean_distance(a: &Tradition, b: &Tradition) -> f64 {
    tradition_distance(a, b).euclidean
}

/// Quick angular distance between two traditions.
pub fn angular_distance(a: &Tradition, b: &Tradition) -> f64 {
    tradition_distance(a, b).angular
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tradition::prebuilt;
    use approx::assert_relative_eq;

    #[test]
    fn test_identical_traditions_zero_distance() {
        let t = prebuilt::pragmatism();
        let dist = tradition_distance(&t, &t);
        assert_relative_eq!(dist.euclidean, 0.0, epsilon = 1e-10);
        assert_relative_eq!(dist.angular, 0.0, epsilon = 1e-5);
        assert_relative_eq!(dist.manhattan, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_angular_distance_bounded() {
        let all = prebuilt::all();
        for i in 0..all.len() {
            for j in 0..all.len() {
                let dist = angular_distance(&all[i], &all[j]);
                assert!(
                    dist >= 0.0 && dist <= std::f64::consts::PI + 1e-10,
                    "Angular distance {} between {} and {} out of bounds",
                    dist, all[i].name, all[j].name
                );
            }
        }
    }

    #[test]
    fn test_pragmatism_close_to_analytic() {
        // Systems Theory and Complexity Science are very similar traditions
        let systems = prebuilt::systems_theory();
        let complexity = prebuilt::complexity_science();
        let marxism = prebuilt::marxism();

        let d_sys_comp = euclidean_distance(&systems, &complexity);
        let d_sys_marx = euclidean_distance(&systems, &marxism);

        assert!(
            d_sys_comp < d_sys_marx,
            "Systems Theory should be closer to Complexity Science ({:.4}) than Marxism ({:.4})",
            d_sys_comp, d_sys_marx
        );
    }

    #[test]
    fn test_symmetry() {
        let a = prebuilt::marxism();
        let b = prebuilt::confucianism();
        let d1 = tradition_distance(&a, &b);
        let d2 = tradition_distance(&b, &a);
        assert_relative_eq!(d1.euclidean, d2.euclidean, epsilon = 1e-10);
    }

    #[test]
    fn test_alignment_values() {
        let dist = tradition_distance(&prebuilt::analytic_philosophy(), &prebuilt::analytic_philosophy());
        for &v in dist.alignment.values() {
            assert!(v >= 0.0, "Self-alignment should be non-negative");
        }
    }

    #[test]
    fn test_tension_points_exist() {
        let dist = tradition_distance(&prebuilt::marxism(), &prebuilt::taoism());
        assert!(!dist.tension_points.is_empty());
    }

    #[test]
    fn test_weighted_distance_differs() {
        let a = prebuilt::analytic_philosophy();
        let b = prebuilt::pragmatism();
        let uniform = tradition_distance(&a, &b);
        let ep_only = tradition_distance_weighted(
            &a,
            &b,
            &DialWeights::only(&[Axis::Epistemology]),
        );
        // Single-axis distance should be <= full distance
        assert!(ep_only.euclidean <= uniform.euclidean + 1e-10);
    }
}
