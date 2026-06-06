//! Topology of the tradition space.
//!
//! Identifies clusters, unexplored regions, and boundaries
//! in the multi-dimensional dial space.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dial::Axis;
use crate::distance;
use crate::tradition::Tradition;

/// A cluster of nearby traditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub members: Vec<String>,
    pub centroid: Vec<f64>,
    pub diameter: f64,
    pub label: String,
}

/// An unexplored region of the tradition space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hole {
    pub center: Vec<f64>,
    pub radius: f64,
    pub nearest_traditions: Vec<String>,
    pub description: String,
}

/// The boundary of the tradition space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boundary {
    pub axis: Axis,
    pub pole: Pole,
    pub nearest_tradition: String,
    pub distance_to_edge: f64,
}

/// Which pole of an axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pole {
    Negative,
    Positive,
}

/// Complete topology analysis of the tradition space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topology {
    pub clusters: Vec<Cluster>,
    pub holes: Vec<Hole>,
    pub boundaries: Vec<Boundary>,
}

/// Analyze the topology of a set of traditions.
pub fn analyze(traditions: &[Tradition], cluster_threshold: f64) -> Topology {
    let clusters = find_clusters(traditions, cluster_threshold);
    let holes = find_holes(traditions);
    let boundaries = find_boundaries(traditions);

    Topology {
        clusters,
        holes,
        boundaries,
    }
}

/// Find connected clusters using single-linkage agglomerative clustering.
pub fn find_clusters(traditions: &[Tradition], threshold: f64) -> Vec<Cluster> {
    let n = traditions.len();
    if n == 0 {
        return vec![];
    }

    // Union-Find for single-linkage clustering
    let mut parent: Vec<usize> = (0..n).collect();

    fn find(parent: &mut Vec<usize>, i: usize) -> usize {
        if parent[i] != i {
            parent[i] = find(parent, parent[i]);
        }
        parent[i]
    }

    for i in 0..n {
        for j in (i + 1)..n {
            let d = distance::euclidean_distance(&traditions[i], &traditions[j]);
            if d <= threshold {
                let ri = find(&mut parent, i);
                let rj = find(&mut parent, j);
                if ri != rj {
                    parent[ri] = rj;
                }
            }
        }
    }

    // Group by root
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let root = find(&mut parent, i);
        groups.entry(root).or_default().push(i);
    }

    groups
        .into_values()
        .map(|indices| {
            let members: Vec<String> = indices.iter().map(|&i| traditions[i].name.clone()).collect();
            let centroid = compute_centroid(&indices.iter().map(|&i| &traditions[i]).collect::<Vec<_>>());
            let diameter = if indices.len() > 1 {
                let mut max_d: f64 = 0.0;
                for i in 0..indices.len() {
                    for j in (i + 1)..indices.len() {
                        let d = distance::euclidean_distance(
                            &traditions[indices[i]],
                            &traditions[indices[j]],
                        );
                        max_d = max_d.max(d);
                    }
                }
                max_d
            } else {
                0.0
            };

            Cluster {
                label: members.join(" · "),
                members,
                centroid,
                diameter,
            }
        })
        .collect()
}

/// Find holes (sparse regions) in the tradition space by sampling random points
/// and checking distance to nearest tradition.
pub fn find_holes(traditions: &[Tradition]) -> Vec<Hole> {
    if traditions.is_empty() {
        return vec![];
    }

    let mut holes = Vec::new();
    let mut rng_state: u64 = 42;

    // Simple LCG RNG
    let next_rand = |state: &mut u64| -> f64 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((*state >> 33) as f64) / (1u64 << 31) as f64 * 2.0 - 1.0
    };

    // Sample random points and find the sparsest ones
    let samples = 200;
    let mut candidates: Vec<(Vec<f64>, f64, Vec<String>)> = Vec::new();

    for _ in 0..samples {
        let point: Vec<f64> = Axis::all().iter().map(|_| next_rand(&mut rng_state)).collect();
        let point_trad = point_to_tradition(&point);

        let mut min_dist = f64::INFINITY;
        let mut nearest = Vec::new();

        for t in traditions {
            let d = distance::euclidean_distance(&point_trad, t);
            if d < min_dist {
                min_dist = d;
                nearest = vec![t.name.clone()];
            } else if (d - min_dist).abs() < 0.3 {
                nearest.push(t.name.clone());
            }
        }

        candidates.push((point, min_dist, nearest));
    }

    // Sort by distance to nearest tradition (largest = biggest hole)
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    candidates.truncate(3);

    for (center, radius, nearest) in candidates {
        holes.push(Hole {
            center,
            radius,
            nearest_traditions: nearest,
            description: format!("Sparse region {:.2} from nearest tradition", radius),
        });
    }

    holes
}

/// Find boundaries: traditions closest to each pole of each axis.
pub fn find_boundaries(traditions: &[Tradition]) -> Vec<Boundary> {
    let mut boundaries = Vec::new();

    for &axis in &Axis::all() {
        // Negative pole: tradition with most negative position
        let neg = traditions
            .iter()
            .min_by(|a, b| a.position(axis).partial_cmp(&b.position(axis)).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();
        // Positive pole: tradition with most positive position
        let pos = traditions
            .iter()
            .max_by(|a, b| a.position(axis).partial_cmp(&b.position(axis)).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();

        boundaries.push(Boundary {
            axis,
            pole: Pole::Negative,
            nearest_tradition: neg.name.clone(),
            distance_to_edge: neg.position(axis).abs() - 1.0,
        });
        boundaries.push(Boundary {
            axis,
            pole: Pole::Positive,
            nearest_tradition: pos.name.clone(),
            distance_to_edge: pos.position(axis).abs() - 1.0,
        });
    }

    boundaries
}

fn compute_centroid(traditions: &[&Tradition]) -> Vec<f64> {
    let n = traditions.len() as f64;
    Axis::all()
        .iter()
        .map(|&axis| {
            traditions.iter().map(|t| t.position(axis)).sum::<f64>() / n
        })
        .collect()
}

fn point_to_tradition(point: &[f64]) -> Tradition {
    let axes = Axis::all();
    let dials: Vec<_> = axes
        .iter()
        .zip(point.iter())
        .map(|(&axis, &pos)| crate::dial::Dial::new(axis, pos))
        .collect();
    Tradition::new("_sample", dials, "sample point")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tradition::prebuilt;

    #[test]
    fn test_topology_analysis_runs() {
        let all = prebuilt::all();
        let topo = analyze(&all, 1.5);
        assert!(!topo.clusters.is_empty());
        assert!(!topo.holes.is_empty());
        assert!(!topo.boundaries.is_empty());
    }

    #[test]
    fn test_boundaries_cover_all_axes() {
        let all = prebuilt::all();
        let boundaries = find_boundaries(&all);
        assert_eq!(boundaries.len(), 14); // 7 axes × 2 poles
    }

    #[test]
    fn test_clusters_at_high_threshold() {
        let all = prebuilt::all();
        // Very high threshold: everything in one cluster
        let clusters = find_clusters(&all, 100.0);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].members.len(), 15);
    }

    #[test]
    fn test_clusters_at_low_threshold() {
        let all = prebuilt::all();
        // Very low threshold: each tradition is its own cluster
        let clusters = find_clusters(&all, 0.0);
        assert_eq!(clusters.len(), 15);
    }

    #[test]
    fn test_systems_complexity_cluster() {
        let all = prebuilt::all();
        // These two should cluster together
        let clusters = find_clusters(&all, 1.0);
        let sys_comp_cluster = clusters.iter().find(|c| {
            c.members.contains(&"Systems Theory".to_string())
                && c.members.contains(&"Complexity Science".to_string())
        });
        assert!(sys_comp_cluster.is_some(), "Systems Theory and Complexity Science should cluster together");
    }
}
