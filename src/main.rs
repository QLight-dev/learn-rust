fn main() {
    // Boxes
    let color = Box::new("green"); // Moves "green" on the heap and color points the to value "green" 
    println!("{color}");

    // cloning
    let student_year = vec![2, 11, 3, 4, 8];
    let new = advance_to_next_year(student_year.clone());

    for student in &student_year {
        println!("{student}");
    }
}

fn advance_to_next_year(mut student_year: Vec<i32>) -> Vec<i32> {
    for student in &mut student_year {
        *student += 1;
    }
    student_year
}
