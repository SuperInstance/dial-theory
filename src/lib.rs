//! # Dial Theory
//!
//! A framework for understanding how different intellectual traditions relate to each
//! other through multi-dimensional dial positions.
//!
//! Each tradition is positioned on seven continuous axes (dials):
//! - **Epistemology**: Rationalist ↔ Empiricist
//! - **Social**: Individualist ↔ Collectivist
//! - **Methodology**: Reductionist ↔ Holist
//! - **Abstraction**: Abstract ↔ Concrete
//! - **Change**: Conservative ↔ Progressive
//! - **Scope**: Universalist ↔ Relativist
//! - **Reasoning**: Formalist ↔ Intuitionist
//!
//! # Example
//!
//! ```
//! use dial_theory::tradition::prebuilt;
//! use dial_theory::distance;
//! use dial_theory::nearest;
//!
//! let analytic = prebuilt::analytic_philosophy();
//! let prag = prebuilt::pragmatism();
//!
//! let dist = distance::tradition_distance(&analytic, &prag);
//! println!("Euclidean distance: {:.4}", dist.euclidean);
//!
//! let nearest = nearest::nearest_neighbors(&analytic, &prebuilt::all(), 3);
//! for n in &nearest {
//!     println!("{}: {:.4}", n.tradition.name, n.distance.euclidean);
//! }
//! ```

pub mod dial;
pub mod distance;
pub mod evolution;
pub mod nearest;
pub mod synthesis;
pub mod topology;
pub mod tradition;
