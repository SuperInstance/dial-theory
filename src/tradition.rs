//! Tradition type and pre-built traditions.
//!
//! A tradition is a named collection of dial positions representing an intellectual
//! tradition's stance across all seven axes.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::dial::{Axis, Dial};

/// An intellectual tradition with dial positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tradition {
    pub name: String,
    pub dials: HashMap<Axis, Dial>,
    pub description: String,
    pub era: Option<String>,
}

impl Tradition {
    /// Create a new tradition with the given name and dials.
    pub fn new(name: &str, dials: Vec<Dial>, description: &str) -> Self {
        Self {
            name: name.to_string(),
            dials: dials.into_iter().map(|d| (d.axis, d)).collect(),
            description: description.to_string(),
            era: None,
        }
    }

    /// Create a tradition with a historical era.
    pub fn with_era(mut self, era: &str) -> Self {
        self.era = Some(era.to_string());
        self
    }

    /// Get the dial position for a given axis, defaulting to 0.0.
    pub fn position(&self, axis: Axis) -> f64 {
        self.dials.get(&axis).map(|d| d.position).unwrap_or(0.0)
    }

    /// Get the confidence for a given axis, defaulting to 1.0.
    pub fn confidence(&self, axis: Axis) -> f64 {
        self.dials.get(&axis).map(|d| d.confidence).unwrap_or(1.0)
    }

    /// Get all axis positions as a vector (in canonical order).
    pub fn position_vector(&self) -> Vec<f64> {
        Axis::all().map(|a| self.position(a)).to_vec()
    }

    /// Validate that all axes have dial positions in [-1, 1].
    pub fn validate(&self) -> Result<(), String> {
        for axis in Axis::all() {
            if let Some(d) = self.dials.get(&axis) {
                if !(-1.0..=1.0).contains(&d.position) {
                    return Err(format!(
                        "Invalid position {} for axis {:?} in tradition {}",
                        d.position, axis, self.name
                    ));
                }
                if !(0.0..=1.0).contains(&d.confidence) {
                    return Err(format!(
                        "Invalid confidence {} for axis {:?} in tradition {}",
                        d.confidence, axis, self.name
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Pre-built traditions with carefully calibrated dial positions.
pub mod prebuilt {
    use super::*;

    /// Analytic Philosophy: Emphasizes logic, language analysis, and empirical methods.
    pub fn analytic_philosophy() -> Tradition {
        Tradition::new(
            "Analytic Philosophy",
            vec![
                Dial::new(Axis::Epistemology, 0.6),
                Dial::new(Axis::Social, -0.3),
                Dial::new(Axis::Methodology, -0.7),
                Dial::new(Axis::Abstraction, -0.8),
                Dial::new(Axis::Change, 0.2),
                Dial::new(Axis::Scope, -0.6),
                Dial::new(Axis::Reasoning, -0.9),
            ],
            "Emphasizes clarity, logical rigor, and argumentation. Focuses on language analysis and formal logic.",
        ).with_era("Early 20th century – present")
    }

    /// Phenomenology: Studies structures of conscious experience from the first-person perspective.
    pub fn phenomenology() -> Tradition {
        Tradition::new(
            "Phenomenology",
            vec![
                Dial::new(Axis::Epistemology, -0.4),
                Dial::new(Axis::Social, -0.1),
                Dial::new(Axis::Methodology, 0.3),
                Dial::new(Axis::Abstraction, -0.5),
                Dial::new(Axis::Change, 0.0),
                Dial::new(Axis::Scope, 0.2),
                Dial::new(Axis::Reasoning, 0.7),
            ],
            "Studies structures of conscious experience. Emphasizes intentionality and lived experience.",
        ).with_era("Early 20th century – present")
    }

    /// Pragmatism: Truth is what works in practice.
    pub fn pragmatism() -> Tradition {
        Tradition::new(
            "Pragmatism",
            vec![
                Dial::new(Axis::Epistemology, 0.7),
                Dial::new(Axis::Social, 0.2),
                Dial::new(Axis::Methodology, 0.3),
                Dial::new(Axis::Abstraction, 0.5),
                Dial::new(Axis::Change, 0.6),
                Dial::new(Axis::Scope, 0.3),
                Dial::new(Axis::Reasoning, -0.2),
            ],
            "Judges ideas by their practical consequences. Truth is what works.",
        ).with_era("Late 19th century – present")
    }

    /// Marxism: Historical materialism, class struggle, collective emancipation.
    pub fn marxism() -> Tradition {
        Tradition::new(
            "Marxism",
            vec![
                Dial::new(Axis::Epistemology, 0.3),
                Dial::new(Axis::Social, 0.9),
                Dial::new(Axis::Methodology, 0.5),
                Dial::new(Axis::Abstraction, 0.2),
                Dial::new(Axis::Change, 0.9),
                Dial::new(Axis::Scope, -0.4),
                Dial::new(Axis::Reasoning, -0.3),
            ],
            "Historical materialism analyzing class struggle and modes of production.",
        ).with_era("Mid 19th century – present")
    }

    /// Buddhism: Suffering, impermanence, non-self, mindfulness.
    pub fn buddhism() -> Tradition {
        Tradition::new(
            "Buddhism",
            vec![
                Dial::new(Axis::Epistemology, -0.5),
                Dial::new(Axis::Social, 0.4),
                Dial::new(Axis::Methodology, 0.6),
                Dial::new(Axis::Abstraction, 0.1),
                Dial::new(Axis::Change, 0.3),
                Dial::new(Axis::Scope, -0.3),
                Dial::new(Axis::Reasoning, 0.8),
            ],
            "Focuses on suffering, impermanence, and the path to enlightenment through mindfulness.",
        ).with_era("5th century BCE – present")
    }

    /// Taoism: Naturalness, simplicity, the Tao as fundamental principle.
    pub fn taoism() -> Tradition {
        Tradition::new(
            "Taoism",
            vec![
                Dial::new(Axis::Epistemology, -0.6),
                Dial::new(Axis::Social, 0.1),
                Dial::new(Axis::Methodology, 0.8),
                Dial::new(Axis::Abstraction, 0.0),
                Dial::new(Axis::Change, -0.2),
                Dial::new(Axis::Scope, 0.4),
                Dial::new(Axis::Reasoning, 0.9),
            ],
            "Emphasizes living in harmony with the Tao, naturalness, and simplicity.",
        ).with_era("4th century BCE – present")
    }

    /// Existentialism: Freedom, authenticity, individual existence precedes essence.
    pub fn existentialism() -> Tradition {
        Tradition::new(
            "Existentialism",
            vec![
                Dial::new(Axis::Epistemology, -0.3),
                Dial::new(Axis::Social, -0.6),
                Dial::new(Axis::Methodology, 0.1),
                Dial::new(Axis::Abstraction, 0.3),
                Dial::new(Axis::Change, 0.4),
                Dial::new(Axis::Scope, 0.6),
                Dial::new(Axis::Reasoning, 0.5),
            ],
            "Emphasizes individual freedom, authenticity, and existence preceding essence.",
        ).with_era("19th–20th century")
    }

    /// Structuralism: Understanding through underlying structures and systems of signs.
    pub fn structuralism() -> Tradition {
        Tradition::new(
            "Structuralism",
            vec![
                Dial::new(Axis::Epistemology, 0.1),
                Dial::new(Axis::Social, 0.5),
                Dial::new(Axis::Methodology, -0.5),
                Dial::new(Axis::Abstraction, -0.6),
                Dial::new(Axis::Change, -0.3),
                Dial::new(Axis::Scope, -0.5),
                Dial::new(Axis::Reasoning, -0.6),
            ],
            "Seeks underlying structures governing human culture, language, and thought.",
        ).with_era("1950s–1970s")
    }

    /// Postmodernism: Skeptical of grand narratives, emphasizes fragmentation and power.
    pub fn postmodernism() -> Tradition {
        Tradition::new(
            "Postmodernism",
            vec![
                Dial::new(Axis::Epistemology, 0.2),
                Dial::new(Axis::Social, 0.3),
                Dial::new(Axis::Methodology, 0.4),
                Dial::new(Axis::Abstraction, 0.1),
                Dial::new(Axis::Change, 0.7),
                Dial::new(Axis::Scope, 0.8),
                Dial::new(Axis::Reasoning, 0.3),
            ],
            "Skeptical of grand narratives. Emphasizes fragmentation, power relations, and discourse.",
        ).with_era("Late 20th century – present")
    }

    /// Process Philosophy: Reality is fundamentally process and becoming, not static substance.
    pub fn process_philosophy() -> Tradition {
        Tradition::new(
            "Process Philosophy",
            vec![
                Dial::new(Axis::Epistemology, -0.2),
                Dial::new(Axis::Social, 0.2),
                Dial::new(Axis::Methodology, 0.7),
                Dial::new(Axis::Abstraction, -0.3),
                Dial::new(Axis::Change, 0.8),
                Dial::new(Axis::Scope, -0.2),
                Dial::new(Axis::Reasoning, 0.3),
            ],
            "Reality as fundamentally processual. Becoming over being.",
        ).with_era("Early 20th century – present")
    }

    /// Confucianism: Social harmony, virtue ethics, filial piety, proper relationships.
    pub fn confucianism() -> Tradition {
        Tradition::new(
            "Confucianism",
            vec![
                Dial::new(Axis::Epistemology, -0.3),
                Dial::new(Axis::Social, 0.8),
                Dial::new(Axis::Methodology, 0.2),
                Dial::new(Axis::Abstraction, 0.4),
                Dial::new(Axis::Change, -0.6),
                Dial::new(Axis::Scope, -0.3),
                Dial::new(Axis::Reasoning, 0.2),
            ],
            "Emphasizes social harmony, virtue, filial piety, and proper relationships.",
        ).with_era("6th century BCE – present")
    }

    /// Feminist Epistemology: Knowledge as situated, embodied, and power-inflected.
    pub fn feminist_epistemology() -> Tradition {
        Tradition::new(
            "Feminist Epistemology",
            vec![
                Dial::new(Axis::Epistemology, 0.4),
                Dial::new(Axis::Social, 0.6),
                Dial::new(Axis::Methodology, 0.5),
                Dial::new(Axis::Abstraction, 0.5),
                Dial::new(Axis::Change, 0.7),
                Dial::new(Axis::Scope, 0.6),
                Dial::new(Axis::Reasoning, 0.3),
            ],
            "Examines how gender and power shape knowledge production and validation.",
        ).with_era("1980s – present")
    }

    /// Indigenous Knowledge Systems: Traditional ecological knowledge, oral traditions, relational ontology.
    pub fn indigenous_knowledge_systems() -> Tradition {
        Tradition::new(
            "Indigenous Knowledge Systems",
            vec![
                Dial::new(Axis::Epistemology, -0.7),
                Dial::new(Axis::Social, 0.7),
                Dial::new(Axis::Methodology, 0.7),
                Dial::new(Axis::Abstraction, 0.8),
                Dial::new(Axis::Change, -0.5),
                Dial::new(Axis::Scope, 0.5),
                Dial::new(Axis::Reasoning, 0.7),
            ],
            "Traditional ecological knowledge emphasizing relationality, reciprocity, and oral transmission.",
        ).with_era("Prehistoric – present")
    }

    /// Systems Theory: Interconnected components forming complex wholes.
    pub fn systems_theory() -> Tradition {
        Tradition::new(
            "Systems Theory",
            vec![
                Dial::new(Axis::Epistemology, 0.5),
                Dial::new(Axis::Social, 0.4),
                Dial::new(Axis::Methodology, 0.9),
                Dial::new(Axis::Abstraction, -0.4),
                Dial::new(Axis::Change, 0.3),
                Dial::new(Axis::Scope, -0.5),
                Dial::new(Axis::Reasoning, -0.5),
            ],
            "Studies interconnected components forming complex wholes with emergent properties.",
        ).with_era("1940s – present")
    }

    /// Complexity Science: Emergence, self-organization, non-linear dynamics.
    pub fn complexity_science() -> Tradition {
        Tradition::new(
            "Complexity Science",
            vec![
                Dial::new(Axis::Epistemology, 0.6),
                Dial::new(Axis::Social, 0.3),
                Dial::new(Axis::Methodology, 0.8),
                Dial::new(Axis::Abstraction, -0.2),
                Dial::new(Axis::Change, 0.5),
                Dial::new(Axis::Scope, -0.3),
                Dial::new(Axis::Reasoning, -0.1),
            ],
            "Studies emergence, self-organization, and non-linear dynamics in complex systems.",
        ).with_era("1980s – present")
    }

    /// Returns all 15 pre-built traditions.
    pub fn all() -> Vec<Tradition> {
        vec![
            analytic_philosophy(),
            phenomenology(),
            pragmatism(),
            marxism(),
            buddhism(),
            taoism(),
            existentialism(),
            structuralism(),
            postmodernism(),
            process_philosophy(),
            confucianism(),
            feminist_epistemology(),
            indigenous_knowledge_systems(),
            systems_theory(),
            complexity_science(),
        ]
    }

    /// Find a pre-built tradition by name.
    pub fn by_name(name: &str) -> Option<Tradition> {
        all().into_iter().find(|t| t.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::prebuilt;

    #[test]
    fn test_all_traditions_valid() {
        for t in prebuilt::all() {
            t.validate().unwrap_or_else(|e| panic!("{}: {}", t.name, e));
        }
    }

    #[test]
    fn test_tradition_count() {
        assert_eq!(prebuilt::all().len(), 15);
    }

    #[test]
    fn test_traditions_have_all_axes() {
        for t in prebuilt::all() {
            for axis in crate::dial::Axis::all() {
                assert!(t.dials.contains_key(&axis), "{} missing {:?}", t.name, axis);
            }
        }
    }

    #[test]
    fn test_position_vector_length() {
        let t = prebuilt::analytic_philosophy();
        assert_eq!(t.position_vector().len(), 7);
    }

    #[test]
    fn test_by_name() {
        assert!(prebuilt::by_name("Pragmatism").is_some());
        assert!(prebuilt::by_name("Nonexistent").is_none());
    }

    #[test]
    fn test_unique_names() {
        let all = prebuilt::all();
        let names: std::collections::HashSet<_> = all.iter().map(|t| t.name.clone()).collect();
        assert_eq!(names.len(), all.len(), "Duplicate tradition names found");
    }

    #[test]
    fn test_era_set() {
        let t = prebuilt::analytic_philosophy();
        assert!(t.era.is_some());
    }
}
