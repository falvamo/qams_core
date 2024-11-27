//! # qams_core
//! QAMS (Quality Assurance Management System) is a purpose-built application for managing Quality
//! Assurance reviews and scorecards. The core package provides structs and functionality in Rust
//! that can be shared across both the CLI (Command Line Interface) and GUI (Graphical User
//! Interface) versions of the program.

/// String representation of the `Fatal` option in the `OptionScore` enum
const FATAL_STR: &str = "FATAL";

/// Represents a scoring scheme for a `CriterionOption`.
#[derive(Debug)]
pub enum OptionScore {
    /// Represents a fatal option. A fatal option will result in any review with the option selected
    /// receiving a total point value of 0 points, and thus a score of 0%.
    Fatal,
    /// Represents an option with a point value. The point value will be added to the total point
    /// value of the review when calculated. The point value may be either positive or negative, but
    /// must be an integer.
    Points(i32)
}

impl OptionScore {

    /// Parse an option score from a string. Assumes fatal options are represented by the string
    /// "FATAL" (case-insensitive) and point-value options are represented by strings containing
    /// only numeric characters (that can be parsed to integers). Returns None if the string doesn't
    /// match either pattern.
    pub fn from_str(s: &str) -> Option<OptionScore> {
        if s.to_uppercase() == FATAL_STR {
            Some(OptionScore::Fatal)
        } else {
            match s.parse::<i32>() {
                Ok(points) => Some(OptionScore::Points(points)),
                Err(_) => None
            }
        }
    }

}

/// Represents an option within a criterion with a label and `OptionScore` value.
#[derive(Debug)]
pub struct CriterionOption {
    /// The user-facing label or name of this `CriterionOption`, e.g., "YES" or "NO".
    label: String,
    /// The `OptionScore` instance that defines how this `Criterion` affects the `Review`'s total
    /// point score.
    score: OptionScore,
}

impl CriterionOption {

    /// Create a new `CriterionOption` instance.
    pub fn new(label: &str, score: OptionScore) -> Self {
        Self { label: label.to_string(), score }
    }

    /// Get the label or name of this `CriterionOption`.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the score (an `OptionScore` instance) associated with this `CriterionOption`.
    pub fn score(&self) -> &OptionScore {
        &self.score
    }
}

/// Represents a criterion in a review in the application.
#[derive(Debug)]
pub struct Criterion {
    /// The user-facing label or name of this `Criterion`, e.g., "Did the product ... ?" or "Did the
    /// agent ...?"
    label: String,
    /// The `CriterionOption`s associated with this `Criterion`. In a review, one of these options
    /// will become the `selection`.
    options: Vec<CriterionOption>,
    /// The index of the currently selected `CriterionOption` if a selection has been made.
    selection_index: Option<usize>
}

impl Criterion {
    /// Create a new `Criterion` instance.
    pub fn new(label: &str, options: Vec<CriterionOption>) -> Self {
        Self { label: label.to_string(), options, selection_index: None }
    }

    /// Get the label or name of this `Criterion`.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the options associated with this `Criterion`.
    pub fn options(&self) -> &Vec<CriterionOption> {
        &self.options
    }

    /// Get the highest point-value associated with the options available within this criterion.
    pub fn max_points(&self) -> i32 {
        let mut max_points = 0;
        for option in &self.options {
            let score = option.score();
            match score {
                OptionScore::Points(points) => {
                    if points > &max_points {
                        max_points = points.clone();
                    }
                }
                OptionScore::Fatal => {}
            }
        }
        max_points
    }

    /// Select a criterion option or change the option selected.
    pub fn set_selection(&mut self, index: usize) {
        assert!(index < self.options.len(), "Tried to select an option that doesn't exist!");
        self.selection_index = Some(index);
    }

    /// Get the currently selected `CriterionOption`.
    pub fn selection(&self) -> &CriterionOption {
        self.options.get(self.selection_index.unwrap()).unwrap()
    }

    /// Get `OptionScore` associated with the current selection.
    pub fn selection_score(&self) -> Option<&OptionScore> {
        match self.selection_index {
            None => None,
            Some(index) => Some(self.options[index].score())
        }
    }
}

/// Represents a QA review in the application.
#[derive(Debug)]
pub struct Review {
    /// The criteria on which the review is conducted.
    criteria: Vec<Criterion>
}

impl Review {
    /// Returns the total of the maximum number of points available for each `Criterion` in the
    /// `Review`.
    pub fn max_points(&self) -> i32 {
        self.criteria.iter().map(|c| c.max_points()).sum()
    }
}

/// Tests for qams_core
#[cfg(test)]
mod tests {
    use crate::{Criterion, CriterionOption, OptionScore, Review};

    /// Test the `max_score` method on the `Criterion` struct.
    #[test]
    fn criterion_max_points_test() {
        let options = vec![
            CriterionOption::new("YES", OptionScore::Points(1)),
            CriterionOption::new("PARTIALLY", OptionScore::Points(0)),
            CriterionOption::new("NO", OptionScore::Fatal)
        ];
        let criterion = Criterion::new("Test criterion", options);

        assert_eq!(criterion.max_points(), 1);
    }

    /// Test the `max_score` method on the `Review` struct.
    #[test]
    fn review_max_points_test() {
        let review = Review {
            criteria: vec![
                Criterion::new("Criterion 1", vec![
                    CriterionOption::new("YES", OptionScore::Points(1)),
                    CriterionOption::new("NO", OptionScore::Points(0)),
                    CriterionOption::new("N/A", OptionScore::Points(1))
                ]),
                Criterion::new("Criterion 2", vec![
                    CriterionOption::new("YES", OptionScore::Points(2)),
                    CriterionOption::new("NO", OptionScore::Points(0))
                ]),
                Criterion::new("Criterion 3", vec![
                    CriterionOption::new("EXCEEDS", OptionScore::Points(3)),
                    CriterionOption::new("MEETS", OptionScore::Points(2)),
                    CriterionOption::new("BELOW", OptionScore::Points(0))
                ])
            ]
        };

        assert_eq!(review.max_points(), 6); // 1 + 2 + 3 = 6
    }

    #[test]
    fn criterion_selection_score_test() {
        let mut criterion = Criterion::new("Criterion", vec![
            CriterionOption::new("EXCEEDS", OptionScore::Points(3)),
            CriterionOption::new("MEETS", OptionScore::Points(2)),
            CriterionOption::new("BELOW", OptionScore::Points(0))
        ]);
        criterion.set_selection(0);

        let score = criterion.selection_score().unwrap();
        match score {
            OptionScore::Points(points) => {
                assert_eq!(points.clone(), 3);
            },
            OptionScore::Fatal => {
                panic!("This shouldn't have happened... A test failed for an unexpected reason.")
            }
        }
    }
}