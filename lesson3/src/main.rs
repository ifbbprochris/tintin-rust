use std::collections::HashMap;

#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    age: i16,
    gender: i8,  //  0 is male, 1 is femal
}

#[derive(Debug)]
struct Class {
    id: u32,
    grade: i16,
    number: i16,
    students: HashMap<u32, Student>
}

fn add_student_to_class(student: Student, class: &mut Class) {
    class.students.insert(student.id, student);
}

fn main() {
    let mut class = Class {
        id: 1,
        grade: 6,
        number: 15,
        students: HashMap::new()
    };
    let student = Student {
        id: 1,
        name: String::from("Chris Zhou"),
        age: 33,
        gender: 0
    };
    add_student_to_class(student, &mut class);
    println!("{:?}", class);
}
