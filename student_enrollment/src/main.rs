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
        println!("Undergrad {}", self.name);
        println!("GPA {}", self.gpa);
        println!("Major {}", self.major);
    }
}

impl ShowInfo for Grad {
    fn show_info(&self) {
        println!("Grad {}", self.name);
        println!("GPA {}", self.gpa);
        println!("Major {}", self.major);
        println!("Thesis {}", self.thesis);
    }
}

struct Enrollment<T: ShowInfo> {
    list: Vec<T>,
}

impl<T: ShowInfo> Enrollment<T> {
    fn new() -> Self {
        Enrollment { list: Vec::new() }
    }

    fn add(&mut self, s: T) {
        self.list.push(s);
    }

    fn show_all(&self) {
        self.list.iter().for_each(|s| s.show_info());
    }
}

fn main() {
    let u = Undergrad {
        name: "Alice".into(),
        gpa: 3.4,
        major: "CS".into(),
    };

    let g = Grad {
        name: "Bob".into(),
        gpa: 3.9,
        major: "DS".into(),
        thesis: "AI Research".into(),
    };

    let mut ug_students = Enrollment::new();
    ug_students.add(u);

    let mut grad_students = Enrollment::new();
    grad_students.add(g);

    ug_students.show_all();
    grad_students.show_all();
}
