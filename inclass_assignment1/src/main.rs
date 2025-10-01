// Define the Student struct with name and major fields
struct Student {
    name: String,
    major: String,
}

// Implement methods for the Student struct
impl Student {
   
    pub fn new(name: &str, major: &str) -> Student {
        Student {
           
            name: name.to_string(),
            major: major.to_string(),
        }
    }

    /// 2. Method to update the student's major
    /// Takes &mut self because it modifies the struct's internal state
    pub fn set_major(&mut self, new_major: &str) {
        self.major = new_major.to_string();
        println!("[ACTION] {}'s major has been set to: {}", self.name, self.major);
    }

    /// 3. Method to get the student's current major
    /// Takes &self because it only reads the struct's internal state
    pub fn get_major(&self) -> &str {
        // Return a string slice (&str) reference to the major field
        &self.major
    }

    
    pub fn display_info(&self) {
        println!("\n--- Student Info ---");
        println!("Name: {}", self.name);
        println!("Major: {}", self.major);
        println!("--------------------");
    }
}

// Main function to demonstrate the usage of the Student struct
fn main() {
   
    let mut student1 = Student::new("Alicia cortina", "Physics");
    
 
    student1.display_info();

   
    let initial_major = student1.get_major();
    println!("\n[QUERY] Alicia's initial major is: {}", initial_major);

    
    student1.set_major("Computer Science");

   
    let updated_major = student1.get_major();
    println!("[QUERY] Alicia's updated major is: {}", updated_major);
    
    //
    student1.display_info();
}