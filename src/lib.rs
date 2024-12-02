//! #qams_core
//! QAMS (Quality Assurance Management System) is a purpose-built application for managing Quality
//! Assurance reviews and scorecards. The core package provides structs and functionality in Rust
//! that can be shared across both the CLI (Command Line Interface) and GUI (Graphical User
//! Interface) versions of the program.

/// Represents the scoring schema associated with a `CriterionOption`.
#[derive(Debug)]
pub enum CriterionOptionScore {
    /// Represents a point value criterion option. When selected, this option's point value is added
    /// to the review's total point value to calculate the score (unless the review contains a
    /// Fatal selection elsewhere).
    Points(i32),
    /// Represents a fatal criterion option. When selected, this review's total point value will
    /// be 0 regardless of the other selections in the review.
    Fatal,
}

/// Represents an option within a `Criterion`, one of which becomes the `selection` during a review.
#[derive(Debug)]
pub struct CriterionOption {
    /// Label for this criterion option.
    label: String,
    /// Scoring schema for this criterion option.
    score: CriterionOptionScore,
}

impl CriterionOption {
    /// Create a criterion option.
    pub fn new(label: &str, score: CriterionOptionScore) -> Self {
        Self {
            label: label.to_string(),
            score,
        }
    }

    /// Get the label for this criterion option.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the score for this criterion option.
    pub fn score(&self) -> &CriterionOptionScore {
        &self.score
    }
}

/// Represents a criterion on which a review is conducted.
#[derive(Debug)]
pub struct Criterion {
    /// Label for this criterion
    label: String,
    /// Options available for selection in this criterion.
    options: Vec<CriterionOption>,
    /// The index of the currently selected criterion option if a selection has been made.
    selection_index: Option<usize>,
}

impl Criterion {
    /// Create a new criterion.
    pub fn new(label: &str, options: Vec<CriterionOption>) -> Self {
        Self {
            label: label.to_string(),
            options,
            selection_index: None,
        }
    }

    /// Set the selection using the index of the option to select.
    pub fn set_selection_index(&mut self, selection_index: usize) {
        assert!(
            selection_index < self.options.len(),
            "Tried to select a nonexistent option!"
        );
        self.selection_index = Some(selection_index);
    }

    /// Get the current selection if one has been made.
    pub fn selection(&self) -> Option<&CriterionOption> {
        match self.selection_index {
            Some(index) => match self.options.get(index) {
                Some(option) => Some(&option),
                None => None,
            },
            None => None,
        }
    }

    /// Compute the maximum number of points available for this criterion.
    pub fn max_points(&self) -> i32 {
        let mut max_points = 0;

        for option in &self.options {
            match option.score {
                CriterionOptionScore::Fatal => {}
                CriterionOptionScore::Points(points) => {
                    if points > max_points {
                        max_points = points;
                    }
                }
            }
        }

        max_points
    }

    /// Get the score associated with the current selection if one has been made.
    pub fn selection_score(&self) -> Option<&CriterionOptionScore> {
        match self.selection() {
            Some(selection) => Some(&selection.score),
            None => None,
        }
    }

    /// Get the label for this criterion.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the options available for this criterion.
    pub fn options(&self) -> &Vec<CriterionOption> {
        &self.options
    }
}

/// Represents a QA review in the application
#[derive(Debug)]
pub struct Review {
    /// The criteria on which this review is conducted
    criteria: Vec<Criterion>,
}

impl Review {
    /// Create a new review
    pub fn new(criteria: Vec<Criterion>) -> Self {
        Self { criteria }
    }

    /// Get the review's criteria as a mutable reference
    pub fn criteria_mut(&mut self) -> &mut Vec<Criterion> {
        &mut self.criteria
    }

    /// Compute the maximum number of points available for this whole review
    pub fn max_points(&self) -> i32 {
        let x = self
            .criteria
            .iter()
            .map(|criterion| criterion.max_points())
            .sum::<i32>();
        x
    }

    /// Compute the total number of points rewarded to this review
    pub fn total_points(&self) -> i32 {
        let mut total_points = 0;

        for criterion in &self.criteria {
            match criterion.selection_score() {
                Some(CriterionOptionScore::Fatal) => {
                    // fatal option selected. total points is 0
                    return 0;
                }
                Some(CriterionOptionScore::Points(points)) => {
                    // point-value option selected. add to total
                    total_points += points;
                }
                None => {
                    // no selection. do nothing
                }
            }
        }

        total_points
    }
}

// Tests for the qams_core crate
#[cfg(test)]
mod tests {
    use crate::{Criterion, CriterionOption, CriterionOptionScore, Review};

    // Test the nax_points and total_points methods on the review are working correctly.
    #[test]
    fn review_test() {
        let criterion = Criterion::new(
            "Criterion 1",
            vec![
                CriterionOption::new("YES", CriterionOptionScore::Points(3)),
                CriterionOption::new("MOSTLY", CriterionOptionScore::Points(2)),
                CriterionOption::new("PARTLY", CriterionOptionScore::Points(1)),
                CriterionOption::new("NO", CriterionOptionScore::Points(0)),
            ],
        );
        let mut review = Review::new(vec![criterion]);

        review.criteria_mut()[0].set_selection_index(2);

        assert_eq!(review.max_points(), 3);
        assert_eq!(review.total_points(), 1);
    }
}
