struct Student {
    name: String,
    major: String,
}

impl Student {
    fn new(n: String, m: String) -> Self {
        Student {
            name: n,
            major: m,
        }
    }

    fn get_major(&self) -> &String {
        return &self.major
    }

    fn set_major(&mut self, new_major: String) {
        self.major = new_major;
    }
}

fn main() {
    let mut student = Student::new("Antonio".to_string(), "Allied Health".to_string());
    println!("Student: {} \n Major: {}", student.name, student.get_major());
    student.set_major("Computer Science".to_string());
    println!("Student: {} \n New Major: {}", student.name, student.get_major());
}
