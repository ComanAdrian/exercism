use std::collections::HashMap;

pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let grade_students = self.roster.get_mut(&grade);

        if grade_students.is_some() {
            grade_students.unwrap().push(student.to_string());
        } else {
            self.roster.insert(grade, vec![student.to_string()]);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.roster.iter().map(|v| *v.0).collect::<Vec<u32>>();

        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.roster.get(&grade).cloned().map(|mut v| {
            v.sort();
            v
        })
    }
}
