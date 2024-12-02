//! #qams_core
//! QAMS (Quality Assurance Management System) is a purpose-built application for managing Quality
//! Assurance reviews and scorecards. The core package provides structs and functionality in Rust
//! that can be shared across both the CLI (Command Line Interface) and GUI (Graphical User
//! Interface) versions of the program.

// constants used to parse scorecard from CSV
const CSV_ROW_DELIMITER: &str = "\n";
const CSV_COL_DELIMITER: &str = ",";
const FATAL_STR: &str = "FATAL";

// constants used to export scorecard to CSV
const CRITERION_STR: &str = "Criterion";
const SELECTION_STR: &str = "Selection";
const COMMENTS_STR: &str = "Comments";
const SCORE_STR: &str = "Percent Score";

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

impl CriterionOptionScore {
    /// Parse a scoring schema from a string slice. Returns `Some(Fatal)` if `s` equals "FATAL" (case insensitive),
    /// `Some(Points(i32))` if `s` can be parsed as an integer, otherwise `None`.
    pub fn from_str(s: &str) -> Option<CriterionOptionScore> {
        if s.to_uppercase() == FATAL_STR {
            return Some(Self::Fatal);
        }
        match s.parse::<i32>() {
            Err(_) => None,
            Ok(points) => Some(Self::Points(points)),
        }
    }
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
    /// Label for this criterion.
    label: String,
    /// Options available for selection in this criterion.
    options: Vec<CriterionOption>,
    /// The index of the currently selected criterion option if a selection has been made.
    selection_index: Option<usize>,
    /// Optional comment attached to this criterion in the review.
    comment: String,
}

impl Criterion {
    /// Create a new criterion.
    pub fn new(label: &str, options: Vec<CriterionOption>) -> Self {
        Self {
            label: label.to_string(),
            options,
            selection_index: None,
            comment: String::new(),
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

    /// Get the optional comment attached to this criterion in the review.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Set the optional comment attached to this criterion in the review.
    pub fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }
}

/// Represents a QA review in the application
#[derive(Debug)]
pub struct Review {
    /// The criteria on which this review is conducted.
    criteria: Vec<Criterion>,
}

impl Review {
    /// Create a new review.
    pub fn new(criteria: Vec<Criterion>) -> Self {
        Self { criteria }
    }

    /// Get the review's criteria as a mutable reference.
    pub fn criteria_mut(&mut self) -> &mut Vec<Criterion> {
        &mut self.criteria
    }

    /// Compute the maximum number of points available for this whole review.
    pub fn max_points(&self) -> i32 {
        let x = self
            .criteria
            .iter()
            .map(|criterion| criterion.max_points())
            .sum::<i32>();
        x
    }

    /// Compute the total number of points rewarded to this review.
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

    /// Compute the percent score for this review
    pub fn percent_score(&self) -> f32 {
        100 as f32 * self.total_points() as f32 / self.max_points() as f32
    }

    pub fn percent_score_string(&self) -> String {
        format!("{:.2}%", self.percent_score())
    }

    /// Create a review from a CSV string.
    pub fn from_csv(csv: &str) -> Review {
        // remove trailing newline characters from the CSV.
        let csv = csv.trim();

        // split the csv input into lines
        let lines: Vec<&str> = csv.split(CSV_ROW_DELIMITER).collect();

        // get the header row
        let header: Vec<&str> = lines
            .get(0)
            .expect("Couldn't get header row of scorecard!")
            .split(CSV_COL_DELIMITER)
            .collect();

        // create an empty vector to store criteria
        let mut criteria: Vec<Criterion> = Vec::new();

        // iterate through lines in the csv
        for line in lines
            .get(1..lines.len())
            .expect("Couldn't get rows of scorecard!")
        {
            // split the line into a row
            let row: Vec<&str> = line.split(CSV_COL_DELIMITER).collect();

            // ensure row is correct size
            assert_eq!(
                row.len(),
                header.len(),
                "Row has the wrong number of columns!"
            );

            // get the criterion label
            let criterion_label = row.get(0).expect("Couldn't get criterion label!");

            // create an empty vec to store criterion options
            let mut criterion_options: Vec<CriterionOption> = Vec::new();

            // iterate through cells in the column
            for i in 1..row.len() {
                // get the score for the criterion option
                let option_score = row.get(i).expect("Couldn't get option score!");
                match CriterionOptionScore::from_str(&option_score) {
                    Some(option_score) => {
                        // add the criterion option to the options vector
                        let option_label = header.get(i).expect("Couldn't get option label!");
                        let option = CriterionOption::new(&option_label, option_score);
                        criterion_options.push(option);
                    }
                    None => {}
                }
            }
            // create the criterion and push it to the criteria vector
            let criterion = Criterion::new(&criterion_label, criterion_options);
            criteria.push(criterion);
        }

        // return the review
        Self { criteria }
    }

    /// Export a review to a CSV string.
    pub fn to_csv(&self) -> String {
        // create an empty mutable vector to store the data
        let mut data: Vec<Vec<&str>> = Vec::new();

        // push a header row to the data
        data.push(vec![CRITERION_STR, SELECTION_STR, COMMENTS_STR]);

        // push the percentage score to the data
        let percent_score_string = self.percent_score_string();
        data.push(vec![SCORE_STR, percent_score_string.as_str(), ""]);

        // iterate through criteria in the review
        for criterion in &self.criteria {
            // add the label, selection and comment associated with the citerion
            // to the data
            let label = criterion.label();
            let selection = match criterion.selection() {
                Some(option) => option.label(),
                None => "",
            };
            let comment = criterion.comment();
            data.push(vec![label, selection, comment]);
        }

        let rows: Vec<String> = data.iter().map(|row| row.join(CSV_COL_DELIMITER)).collect();
        let csv: String = rows.join(CSV_ROW_DELIMITER);
        csv
    }
}

// Tests for the qams_core crate
#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{Criterion, CriterionOption, CriterionOptionScore, Review};

    // Test the nax_points and total_points methods on the review are working correctly.
    #[test]
    fn test_review_max_and_total_points() {
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

        fs::write("review.csv", review.to_csv()).expect("Failed to write review to CSV!");

        assert_eq!(review.max_points(), 3);
        assert_eq!(review.total_points(), 1);
    }

    #[test]
    /// Test the from_csv method on the review is working correctly.
    fn test_review_from_csv() {
        let csv = fs::read_to_string("sample_scorecard.csv")
            .expect("Failed test because sample scorecard couldn't be loaded.");
        let csv = csv.trim();
        let review = Review::from_csv(csv);

        assert_eq!(review.criteria.len(), 7);
        assert_eq!(review.max_points(), 6);
    }
}
