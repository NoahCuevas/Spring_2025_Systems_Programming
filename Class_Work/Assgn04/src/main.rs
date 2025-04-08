// In class Assignment

// Create a struct Student (major)
struct Student {
    major: String,
}


// Higher order functions update major
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String)) {
    for student in &mut collection {
        behavior(student, String::from("Computer Science"));
    }
    for (i, student) in collection.iter().enumerate() {     // print
        println!("Student {}: {}", i + 1, student.major);
    }
}


// First Order functions, assign_major(student,major_declared)
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}


// Create  vector or students1,2,3 and update all students major
fn main() {
    let students = vec![
        Student { major: String::from("Nothing") },
        Student { major: String::from("Nothingg") },
        Student { major: String::from("Nothinggg") },
    ];

    update_majors(students, assign_major);
}
