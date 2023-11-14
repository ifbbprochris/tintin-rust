use std::collections::HashMap;

#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    info: String
}

#[derive(Debug)]
struct Class {
    id: u32,
    name: String,
    students: HashMap<u32, Student>
}

impl Class {
    fn add_student(&mut self, stu: &Student) {
        println!("add_student {:?}", stu);
        let new_stu = Student {
            id: stu.id,
            name: stu.name.to_string(),
            info: stu.info.to_string()
        };
        self.students.insert(new_stu.id, new_stu);
    }

    fn remove_student(&mut self, stu: &Student) {
        println!("remove_student {:?}", stu);
        self.students.remove(&stu.id);
    }

    fn edit_student_info(&mut self, stu: Student, info: &String) {
        println!("edit_student_info {:?}", stu);
        let new_stu = Student {
            info: info.to_string(),
            ..stu
        };
        self.students.insert(stu.id, new_stu);
    }

    fn find_student_by_id(&self, id: u32) -> Option<&Student> {
        println!("find_student_by_id {}", id);
        let student = self.students.get(&id);
        student
    }
}

fn main() {
    let mut cla = Class {
        id: 1,
        name: String::from("class 2 grade 3 "),
        students: HashMap::new()
    };
    let student = Student {
        id: 1,
        name: String::from("Chris Zhou"),
        info: String::from("Chris Zhou info"),
    };
    let student2 = Student {
        id: 2,
        name: String::from("Eric Zhou"),
        info: String::from("Eric Zhou info"),
    };
    let student3 = Student {
        id: 3,
        name: String::from("Lucy"),
        info: String::from("Lucy info"),
    };
    let student4 = Student {
        id: 4,
        name: String::from("Lily"),
        info: String::from("Lily info"),
    };

    cla.add_student(&student);
    cla.add_student(&student2);
    cla.add_student(&student3);
    cla.add_student(&student4);
    println!("{:?}", cla);

    cla.remove_student(&student2);
    println!("after remove student2 {:?}", cla);

    let info = String::from("update Lucy info");
    cla.edit_student_info(student3, &info);
    println!("after edit student3 {:?}", cla);

    let target = cla.find_student_by_id(3);
    println!("target {:?}", target);

    let nontarget = cla.find_student_by_id(5);
    println!("nontarget {:?}", nontarget);
}
