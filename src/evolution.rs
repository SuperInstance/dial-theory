//! Tradition evolution: drift, splitting, and merging over time.
//!
//! Models how traditions change their dial positions over historical time,
//! including splitting into sub-traditions and merging with others.

use serde::{Deserialize, Serialize};

use crate::dial::Axis;
use crate::tradition::Tradition;

/// A snapshot of a tradition's position at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionSnapshot {
    pub tradition: Tradition,
    pub year: i32,
}

/// An evolution trajectory: a sequence of snapshots over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trajectory {
    pub snapshots: Vec<TraditionSnapshot>,
}

impl Trajectory {
    /// Create a new empty trajectory.
    pub fn new() -> Self {
        Self { snapshots: vec![] }
    }

    /// Add a snapshot.
    pub fn add(&mut self, tradition: Tradition, year: i32) {
        self.snapshots.push(TraditionSnapshot { tradition, year });
    }

    /// Get interpolated position at a given year.
    pub fn position_at(&self, axis: Axis, year: i32) -> Option<f64> {
        if self.snapshots.is_empty() {
            return None;
        }

        let sorted: Vec<_> = {
            let mut s = self.snapshots.clone();
            s.sort_by_key(|s| s.year);
            s
        };

        if year <= sorted[0].year {
            return Some(sorted[0].tradition.position(axis));
        }
        if year >= sorted.last().unwrap().year {
            return Some(sorted.last().unwrap().tradition.position(axis));
        }

        // Linear interpolation between bracketing snapshots
        for i in 0..sorted.len() - 1 {
            if year >= sorted[i].year && year <= sorted[i + 1].year {
                let t = (year - sorted[i].year) as f64
                    / (sorted[i + 1].year - sorted[i].year) as f64;
                let p0 = sorted[i].tradition.position(axis);
                let p1 = sorted[i + 1].tradition.position(axis);
                return Some(p0 + t * (p1 - p0));
            }
        }

        None
    }

    /// Project forward by assuming constant drift rate.
    pub fn project(&self, future_year: i32) -> Option<Tradition> {
        if self.snapshots.len() < 2 {
            return None;
        }

        let mut sorted = self.snapshots.clone();
        sorted.sort_by_key(|s| s.year);

        let last = sorted.last()?;
        let prev = &sorted[sorted.len() - 2];
        let dt = (last.year - prev.year) as f64;
        if dt == 0.0 {
            return None;
        }

        let forward_dt = (future_year - last.year) as f64;

        let dials: Vec<_> = Axis::all()
            .iter()
            .map(|&axis| {
                let drift_rate = (last.tradition.position(axis) - prev.tradition.position(axis)) / dt;
                let projected = last.tradition.position(axis) + drift_rate * forward_dt;
                crate::dial::Dial::new(axis, projected)
            })
            .collect();

        Some(Tradition::new(
            &format!("{} (projected {})", last.tradition.name, future_year),
            dials,
            &format!("Projected trajectory from {} to {}", last.year, future_year),
        ))
    }
}

/// Apply random drift to a tradition's dial positions.
///
/// `drift_rate` controls how much each axis can change per step.
/// Uses a deterministic seed for reproducibility.
pub fn apply_drift(tradition: &Tradition, drift_rate: f64, seed: u64) -> Tradition {
    let mut state = seed;
    let mut dials = Vec::new();

    for &axis in &Axis::all() {
        // Simple LCG for deterministic drift
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let noise = ((state >> 33) as f64 / (1u64 << 31) as f64) * 2.0 - 1.0;
        let new_pos = tradition.position(axis) + noise * drift_rate;
        dials.push(crate::dial::Dial::with_confidence(
            axis,
            new_pos,
            tradition.confidence(axis),
        ));
    }

    Tradition::new(
        &tradition.name,
        dials,
        &tradition.description,
    )
}

/// Simulate evolution over a sequence of years.
pub fn simulate_evolution(
    start: &Tradition,
    start_year: i32,
    end_year: i32,
    step_years: i32,
    drift_rate: f64,
) -> Trajectory {
    let mut traj = Trajectory::new();
    traj.add(start.clone(), start_year);

    let mut current = start.clone();
    let mut year = start_year + step_years;
    let mut seed = start_year as u64;

    while year <= end_year {
        seed = seed.wrapping_add(1);
        current = apply_drift(&current, drift_rate, seed);
        traj.add(current.clone(), year);
        year += step_years;
    }

    traj
}

/// Split a tradition into two sub-traditions with divergent drift.
pub fn split(tradition: &Tradition, divergence: f64) -> (Tradition, Tradition) {
    let mut dials_a = Vec::new();
    let mut dials_b = Vec::new();

    for &axis in &Axis::all() {
        let pos = tradition.position(axis);
        dials_a.push(crate::dial::Dial::new(axis, pos + divergence));
        dials_b.push(crate::dial::Dial::new(axis, pos - divergence));
    }

    let a = Tradition::new(
        &format!("{} (Branch A)", tradition.name),
        dials_a,
        &format!("Divergent branch of {}", tradition.name),
    );
    let b = Tradition::new(
        &format!("{} (Branch B)", tradition.name),
        dials_b,
        &format!("Divergent branch of {}", tradition.name),
    );

    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tradition::prebuilt;
    use approx::assert_relative_eq;

    #[test]
    fn test_trajectory_interpolation() {
        let mut traj = Trajectory::new();
        let mut t1 = prebuilt::pragmatism();
        t1.name = "Test".to_string();
        traj.add(t1.clone(), 1900);

        // Modify position for 2000
        let mut t2 = t1.clone();
        let dials: Vec<_> = Axis::all()
            .iter()
            .map(|&axis| {
                crate::dial::Dial::new(axis, t2.position(axis) + 0.5)
            })
            .collect();
        t2 = Tradition::new("Test", dials, "modified");
        traj.add(t2, 2000);

        let pos = traj.position_at(Axis::Epistemology, 1950).unwrap();
        let base = prebuilt::pragmatism().position(Axis::Epistemology);
        let expected = (base + (base + 0.5).min(1.0)) / 2.0;
        assert_relative_eq!(pos, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_trajectory_extrapolation_before() {
        let mut traj = Trajectory::new();
        traj.add(prebuilt::pragmatism(), 1900);
        let pos = traj.position_at(Axis::Epistemology, 1800).unwrap();
        assert_relative_eq!(pos, prebuilt::pragmatism().position(Axis::Epistemology));
    }

    #[test]
    fn test_drift_changes_positions() {
        let t = prebuilt::pragmatism();
        let drifted = apply_drift(&t, 0.1, 42);
        // At least one position should differ
        let any_changed = Axis::all().iter().any(|&axis| {
            (t.position(axis) - drifted.position(axis)).abs() > 1e-10
        });
        assert!(any_changed, "Drift should change at least one position");
    }

    #[test]
    fn test_evolution_produces_smooth_trajectory() {
        let t = prebuilt::pragmatism();
        let traj = simulate_evolution(&t, 1900, 2000, 10, 0.05);

        // Should have ~11 snapshots
        assert!(traj.snapshots.len() >= 10);

        // Check positions stay in [-1, 1] range (or close, small overshoot possible)
        for snap in &traj.snapshots {
            for &axis in &Axis::all() {
                let pos = snap.tradition.position(axis);
                assert!(
                    pos >= -1.5 && pos <= 1.5,
                    "Position {:.3} for {:?} in {} out of reasonable range",
                    pos, axis, snap.year
                );
            }
        }
    }

    #[test]
    fn test_split_produces_divergent_traditions() {
        let t = prebuilt::pragmatism();
        let (a, b) = split(&t, 0.2);
        assert_ne!(a.name, b.name);

        for &axis in &Axis::all() {
            assert!(a.position(axis) > b.position(axis));
        }
    }

    #[test]
    fn test_projection() {
        let mut traj = Trajectory::new();
        traj.add(prebuilt::pragmatism(), 1900);

        let mut modified = prebuilt::pragmatism();
        let dials: Vec<_> = Axis::all()
            .iter()
            .map(|&axis| crate::dial::Dial::new(axis, modified.position(axis) + 0.3))
            .collect();
        modified = Tradition::new("Pragmatism", dials, "");
        traj.add(modified, 2000);

        let projected = traj.project(2025);
        assert!(projected.is_some());
        let p = projected.unwrap();
        // Should be beyond the 2000 position
        for &axis in &Axis::all() {
            assert!(p.position(axis) >= prebuilt::pragmatism().position(axis) + 0.3 - 0.01);
        }
    }
}
