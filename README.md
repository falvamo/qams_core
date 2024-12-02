# qams_core

QAMS (Quality Assurance Management System) is a purpose-built application for managing Quality Assurance reviews and scorecards. The core package provides structs and functionality in Rust that can be shared across both the CLI (Command Line Interface) and GUI (Graphical User Interface) versions of the program.

## Overview

`qams_core` provides the following Rust structures:
- `Criterion` - Represents a criterion on which a review is conducted. Has fields:
    - `label` (`String`) - e.g., "Did the product ...?" or "Did the employee ...?".
    - `options` (`Vec` of `CriterionOption` instances) the options available to be selected in this criterion.
    - `selection_index` (the index of the currently selected `CriterionOption` in the vector).
- `CriterionOption` - Represents an option within a Criterion, one of which becomes the selection during a review. Has fields: 
    - `label` (`String`) - e.g., "YES" or "NO".
    - `score` (`CriterionOptionScore`) - the scoring schema assocaited with this option.
- `Review` - Represents a QA review in the application. Has fields: 
    - `criteria` (`Vec` of `Criterion` instances) the criteria on which this review is conducted.  

and the following enumeration
- `CriterionOptionScore` - Represents the scoring schema associated with a CriterionOption. Has options: 
    - `Points(i32)` When selected, the option's point value is added to the review's total point value to calculate the score (unless the review contains a Fatal selection elsewhere).
    - `Fatal` When selected, this review's total point value will be 0 regardless of the other selections in the review.

For full API documentation, clone the repository and run `cargo doc --open`.