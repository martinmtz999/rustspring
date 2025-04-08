// In class Assignment

// Create a struct Student (major)
struct Student{
    major:String,
}
// First order functions, assign_major(student, major_declared)

fn update_majors(collection: &mut [Student],behavior: fn(&mut Student, String), major: String){
    for student in collection.iter_mut(){
        behavior(student, major.clone());
    }
}

fn assign_major(s: &mut Student, major:String){
    s.major = major;
}

// Helper function to print
fn print_students(collection: &[Student]) {
    print!("Majors: ");
    for (i, student) in collection.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", student.major);
    }
    println!();
}

// Create a vector of students 1,2,3 and update all students major
fn main(){
    let mut collection = vec![
        Student {major: String::from("Not Declared") },
        Student {major: String::from("Not Declared") },
        Student {major: String::from("Not Declared") },
    ];

    print!("Original ");
    print_students(&collection);
    
    update_majors(&mut collection, assign_major, String::from("Computer Science"));
    print!("After changing the: ");
    print_students(&collection);

    update_majors(&mut collection, assign_major, String::from("Graphic Design"));
    print!("After 2nd change: ");
    print_students(&collection);
}

// No closures allowed