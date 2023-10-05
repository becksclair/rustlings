// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

pub trait NumericGrade {
    fn print(&self) -> String;
}
pub trait StrGrade {
    fn print(&self) -> String;
}

impl NumericGrade for ReportCard {
    fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

impl StrGrade for ReportCard {
    fn print(&self) -> String {
        let grade = match &self.grade {
            1.0 => "A+",
            2.0 => "A",
            3.0 => "A-",
            4.0 => "B+",
            5.0 => "B",
            6.0 => "B-",
            7.0 => "C+",
            8.0 => "C",
            9.0 => "C-",
            10.0 => "D+",
            11.0 => "D",
            12.0 => "D-",
            13.0 => "F+",
            14.0 => "F",
            15.0 => "F-",
            _ => "F",
        };
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            NumericGrade::print(&report_card),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 1.0,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            StrGrade::print(&report_card),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
