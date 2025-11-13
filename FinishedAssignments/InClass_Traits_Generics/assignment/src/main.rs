trait ShowInfo {
    fn show_info(&self);
}

struct Undergrad {
    name: String,
    gpa: f32,
    major: String,
}

struct Grad {
    name: String,
    gpa: f32,
    major: String,
    thesis: String,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!(
            "Undergrad Student: {}, Major: {}, GPA: {:.2}",
            self.name, self.major, self.gpa
        );
    }
}

impl ShowInfo for Grad {
    fn show_info(&self) {
        println!(
            "Grad Student: {}, Major: {}, GPA: {:.2}, Thesis: {}",
            self.name, self.major, self.gpa, self.thesis
        );
    }
}

struct Enrollment<T: ShowInfo> {
    students: Vec<T>,
}

impl<T: ShowInfo> ShowInfo for Enrollment<T> {
    fn show_info(&self) {
        for student in &self.students {
            student.show_info();
        }
    }
}

fn main() {
    let under_enrollment = Enrollment {
        students: vec![
            Undergrad { name: "Daniel".into(), gpa: 3.7, major: "Computer Science".into() },
            Undergrad { name: "Eve".into(), gpa: 3.8, major: "Math".into() },
        ],
    };

    let grad_enrollment = Enrollment {
        students: vec![
            Grad { name: "Bob".into(), gpa: 3.9, major: "Physics".into(), thesis: "Quantum Field Simulation".into() },
            Grad { name: "Carol".into(), gpa: 4.0, major: "Chemistry".into(), thesis: "Organic Synthesis".into() },
        ],
    };

    println!("--- Undergrad Enrollment ---");
    under_enrollment.show_info();

    println!("\n--- Grad Enrollment ---");
    grad_enrollment.show_info();
}
