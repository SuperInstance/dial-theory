//! Core dial type and distance metrics.
//!
//! A dial represents a tradition's position on a single axis, with an optional
//! confidence value indicating how well-characterized that position is.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// The seven axes of dial theory.
///
/// Negative values (-1) correspond to the left pole, positive values (+1) to the right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Axis {
    /// Rationalist (-1) ↔ Empiricist (+1)
    Epistemology,
    /// Individualist (-1) ↔ Collectivist (+1)
    Social,
    /// Reductionist (-1) ↔ Holist (+1)
    Methodology,
    /// Abstract (-1) ↔ Concrete (+1)
    Abstraction,
    /// Conservative (-1) ↔ Progressive (+1)
    Change,
    /// Universalist (-1) ↔ Relativist (+1)
    Scope,
    /// Formalist (-1) ↔ Intuitionist (+1)
    Reasoning,
}

impl Axis {
    /// Returns all seven axes.
    pub fn all() -> [Axis; 7] {
        [
            Axis::Epistemology,
            Axis::Social,
            Axis::Methodology,
            Axis::Abstraction,
            Axis::Change,
            Axis::Scope,
            Axis::Reasoning,
        ]
    }

    /// Human-readable label for the negative pole.
    pub fn negative_pole(&self) -> &'static str {
        match self {
            Axis::Epistemology => "Rationalist",
            Axis::Social => "Individualist",
            Axis::Methodology => "Reductionist",
            Axis::Abstraction => "Abstract",
            Axis::Change => "Conservative",
            Axis::Scope => "Universalist",
            Axis::Reasoning => "Formalist",
        }
    }

    /// Human-readable label for the positive pole.
    pub fn positive_pole(&self) -> &'static str {
        match self {
            Axis::Epistemology => "Empiricist",
            Axis::Social => "Collectivist",
            Axis::Methodology => "Holist",
            Axis::Abstraction => "Concrete",
            Axis::Change => "Progressive",
            Axis::Scope => "Relativist",
            Axis::Reasoning => "Intuitionist",
        }
    }

    /// Human-readable name for the axis itself.
    pub fn label(&self) -> &'static str {
        match self {
            Axis::Epistemology => "Epistemology",
            Axis::Social => "Social",
            Axis::Methodology => "Methodology",
            Axis::Abstraction => "Abstraction",
            Axis::Change => "Change",
            Axis::Scope => "Scope",
            Axis::Reasoning => "Reasoning",
        }
    }
}

/// A dial: a position on an axis with an associated confidence.
///
/// Position ranges from -1.0 to 1.0. Confidence ranges from 0.0 to 1.0
/// and indicates how well-established the position is.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dial {
    pub axis: Axis,
    pub position: f64,
    pub confidence: f64,
}

impl Dial {
    /// Create a new dial with the given position and full confidence.
    pub fn new(axis: Axis, position: f64) -> Self {
        Self {
            axis,
            position: position.clamp(-1.0, 1.0),
            confidence: 1.0,
        }
    }

    /// Create a dial with explicit confidence.
    pub fn with_confidence(axis: Axis, position: f64, confidence: f64) -> Self {
        Self {
            axis,
            position: position.clamp(-1.0, 1.0),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }

    /// Euclidean distance between two dials on the same axis.
    pub fn euclidean_distance(&self, other: &Dial) -> f64 {
        (self.position - other.position).abs()
    }

    /// Angular distance between two dials, treating positions as points on a circle.
    ///
    /// Returns a value in [0, π].
    pub fn angular_distance(&self, other: &Dial) -> f64 {
        let theta1 = (self.position + 1.0) * PI / 2.0;
        let theta2 = (other.position + 1.0) * PI / 2.0;
        (theta1 - theta2).abs().min(PI)
    }

    /// Manhattan (L1) distance between two dials.
    pub fn manhattan_distance(&self, other: &Dial) -> f64 {
        (self.position - other.position).abs()
    }
}

/// Weights for each axis, used to emphasize certain dimensions in distance calculations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialWeights {
    pub weights: std::collections::HashMap<Axis, f64>,
}

impl DialWeights {
    /// Create uniform weights (all 1.0).
    pub fn uniform() -> Self {
        Self {
            weights: Axis::all().into_iter().map(|a| (a, 1.0)).collect(),
        }
    }

    /// Create weights where only the specified axes are active.
    pub fn only(axes: &[Axis]) -> Self {
        let mut w = Self {
            weights: Axis::all().into_iter().map(|a| (a, 0.0)).collect(),
        };
        for &a in axes {
            w.weights.insert(a, 1.0);
        }
        w
    }

    /// Get weight for an axis, defaulting to 1.0.
    pub fn get(&self, axis: Axis) -> f64 {
        *self.weights.get(&axis).unwrap_or(&1.0)
    }

    /// Set weight for an axis.
    pub fn set(&mut self, axis: Axis, weight: f64) {
        self.weights.insert(axis, weight);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_dial_position_clamped() {
        let d = Dial::new(Axis::Epistemology, 5.0);
        assert_relative_eq!(d.position, 1.0);
        let d = Dial::new(Axis::Epistemology, -5.0);
        assert_relative_eq!(d.position, -1.0);
    }

    #[test]
    fn test_dial_confidence_clamped() {
        let d = Dial::with_confidence(Axis::Epistemology, 0.5, 2.0);
        assert_relative_eq!(d.confidence, 1.0);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = Dial::new(Axis::Epistemology, -1.0);
        let b = Dial::new(Axis::Epistemology, 1.0);
        assert_relative_eq!(a.euclidean_distance(&b), 2.0);
    }

    #[test]
    fn test_angular_distance_max() {
        let a = Dial::new(Axis::Epistemology, -1.0);
        let b = Dial::new(Axis::Epistemology, 1.0);
        assert_relative_eq!(a.angular_distance(&b), PI);
    }

    #[test]
    fn test_angular_distance_zero() {
        let a = Dial::new(Axis::Epistemology, 0.5);
        assert_relative_eq!(a.angular_distance(&a), 0.0);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = Dial::new(Axis::Epistemology, -0.5);
        let b = Dial::new(Axis::Epistemology, 0.3);
        assert_relative_eq!(a.manhattan_distance(&b), 0.8);
    }

    #[test]
    fn test_all_axes_count() {
        assert_eq!(Axis::all().len(), 7);
    }

    #[test]
    fn test_axis_labels_exist() {
        for axis in Axis::all() {
            assert!(!axis.label().is_empty());
            assert!(!axis.negative_pole().is_empty());
            assert!(!axis.positive_pole().is_empty());
        }
    }

    #[test]
    fn test_uniform_weights() {
        let w = DialWeights::uniform();
        for &axis in &Axis::all() {
            assert_relative_eq!(w.get(axis), 1.0);
        }
    }
}
