//! Nearest-tradition search and bridge detection.
//!
//! Find k-nearest neighbors in dial space and identify traditions
//! that bridge between two otherwise distant traditions.

use serde::{Deserialize, Serialize};

use crate::dial::DialWeights;
use crate::distance::{self, TraditionDistance};
use crate::tradition::Tradition;

/// A tradition paired with its distance from a query point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighborResult {
    pub tradition: Tradition,
    pub distance: TraditionDistance,
}

/// Find the k nearest traditions to a given tradition.
pub fn nearest_neighbors(query: &Tradition, candidates: &[Tradition], k: usize) -> Vec<NeighborResult> {
    nearest_neighbors_weighted(query, candidates, k, &DialWeights::uniform())
}

/// Find the k nearest traditions with custom axis weights.
pub fn nearest_neighbors_weighted(
    query: &Tradition,
    candidates: &[Tradition],
    k: usize,
    weights: &DialWeights,
) -> Vec<NeighborResult> {
    let mut results: Vec<NeighborResult> = candidates
        .iter()
        .filter(|c| c.name != query.name)
        .map(|c| NeighborResult {
            distance: distance::tradition_distance_weighted(query, c, weights),
            tradition: c.clone(),
        })
        .collect();

    results.sort_by(|a, b| {
        a.distance
            .euclidean
            .partial_cmp(&b.distance.euclidean)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    results.truncate(k);
    results
}

/// Find bridge traditions: traditions close to both of two given traditions.
///
/// A bridge is a tradition that minimizes the maximum distance to both endpoints.
/// Useful for finding intellectual mediators between distant traditions.
pub fn find_bridges(
    a: &Tradition,
    b: &Tradition,
    candidates: &[Tradition],
    k: usize,
) -> Vec<BridgeResult> {
    find_bridges_weighted(a, b, candidates, k, &DialWeights::uniform())
}

/// Find bridge traditions with custom weights.
pub fn find_bridges_weighted(
    a: &Tradition,
    b: &Tradition,
    candidates: &[Tradition],
    k: usize,
    weights: &DialWeights,
) -> Vec<BridgeResult> {
    let mut results: Vec<BridgeResult> = candidates
        .iter()
        .filter(|c| c.name != a.name && c.name != b.name)
        .map(|c| {
            let da = distance::tradition_distance_weighted(a, c, weights);
            let db = distance::tradition_distance_weighted(b, c, weights);
            let score = (da.euclidean + db.euclidean) / 2.0;
            let max_d = da.euclidean.max(db.euclidean);
            BridgeResult {
                tradition: c.clone(),
                distance_to_a: da,
                distance_to_b: db,
                bridge_score: score,
                max_distance: max_d,
            }
        })
        .collect();

    // Sort by bridge score (lower = better bridge)
    results.sort_by(|a, b| {
        a.bridge_score
            .partial_cmp(&b.bridge_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    results.truncate(k);
    results
}

/// Result of a bridge search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeResult {
    pub tradition: Tradition,
    pub distance_to_a: TraditionDistance,
    pub distance_to_b: TraditionDistance,
    /// Average distance to both endpoints (lower = better bridge).
    pub bridge_score: f64,
    /// Maximum distance to either endpoint.
    pub max_distance: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tradition::prebuilt;

    #[test]
    fn test_nearest_returns_k_results() {
        let query = prebuilt::pragmatism();
        let all = prebuilt::all();
        let nn = nearest_neighbors(&query, &all, 3);
        assert_eq!(nn.len(), 3);
    }

    #[test]
    fn test_nearest_sorted_by_distance() {
        let query = prebuilt::pragmatism();
        let all = prebuilt::all();
        let nn = nearest_neighbors(&query, &all, 5);
        for i in 1..nn.len() {
            assert!(
                nn[i - 1].distance.euclidean <= nn[i].distance.euclidean + 1e-10,
                "Results not sorted: {:.4} > {:.4}",
                nn[i - 1].distance.euclidean,
                nn[i].distance.euclidean
            );
        }
    }

    #[test]
    fn test_nearest_excludes_self() {
        let query = prebuilt::pragmatism();
        let all = prebuilt::all();
        let nn = nearest_neighbors(&query, &all, 15);
        assert!(nn.iter().all(|n| n.tradition.name != "Pragmatism"));
    }

    #[test]
    fn test_bridge_traditions() {
        let a = prebuilt::analytic_philosophy();
        let b = prebuilt::taoism();
        let all = prebuilt::all();
        let bridges = find_bridges(&a, &b, &all, 3);
        assert_eq!(bridges.len(), 3);
        // Bridges should exist and have finite scores
        for bridge in &bridges {
            assert!(bridge.bridge_score.is_finite());
            assert!(bridge.bridge_score > 0.0);
        }
    }

    #[test]
    fn test_bridge_excludes_endpoints() {
        let a = prebuilt::analytic_philosophy();
        let b = prebuilt::taoism();
        let all = prebuilt::all();
        let bridges = find_bridges(&a, &b, &all, 15);
        let names: Vec<_> = bridges.iter().map(|b| b.tradition.name.as_str()).collect();
        assert!(!names.contains(&"Analytic Philosophy"));
        assert!(!names.contains(&"Taoism"));
    }

    #[test]
    fn test_systems_theory_close_to_complexity() {
        let systems = prebuilt::systems_theory();
        let all = prebuilt::all();
        let nn = nearest_neighbors(&systems, &all, 1);
        assert_eq!(nn[0].tradition.name, "Complexity Science");
    }
}
