fn main() {
    // Defining structs
    struct Student {
        student_id: String,
        grade: i32,
        name: String,
        parent_name: String,
    }

    // Instantiating structs
    let student1 = Student {
        student_id: String::from("1653"),
        grade: 5,
        name: String::from("Jamal Bobricot"),
        parent_name: String::from("Joe Bobricot"),
    };

    println!(
        "{}'s student id is {}, and his grade is {}",
        student1.name, student1.student_id, student1.grade
    );

    // Creating Instances with Struct Update Syntax
    let new_student1 = Student {
        grade: 6,
        ..student1
    };

    println!(
        "the new {}'s student id is {}, and his updated grade is {}",
        new_student1.name, new_student1.student_id, new_student1.grade
    )
}
