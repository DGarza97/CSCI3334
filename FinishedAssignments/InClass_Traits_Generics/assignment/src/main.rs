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

enum Student {
    Undergrad(Undergrad),
    Grad(Grad),
}

impl ShowInfo for Student {
    fn show_info(&self) {
        match self {
            Student::Undergrad(u) => u.show_info(),
            Student::Grad(g) => g.show_info(),
        }
    }
}

struct Enrollment {
    students: Vec<Student>,
}

impl ShowInfo for Enrollment {
    fn show_info(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}

fn main() {
    let undergrad = Student::Undergrad(Undergrad {
        name: "Daniel".into(),
        gpa: 3.7,
        major: "Computer Science".into(),
    });

    let grad = Student::Grad(Grad {
        name: "Bob".into(),
        gpa: 3.9,
        major: "Physics".into(),
        thesis: "Quantum Field Simulation".into(),
    });

    let enrollment = Enrollment {
        students: vec![undergrad, grad],
    };

    println!("Enrolled Students");
    enrollment.show_info();
}
