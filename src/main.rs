fn main() {
    // Control Flow

    // if expressions.
    let age = 19;

    if age >= 18 {
        println!("You can vote!");
    } else {
        println!("You can't vote!");
    }

    // Multiple conditions.
    if age <= 5 {
        println!("Toddler");
    } else if age <= 12 {
        println!("Kid");
    } else if age <= 17 {
        println!("Teen");
    } else {
        println!("Adult");
    }

    // if in a let statement
    let underage: bool = if age < 18 { true } else { false };
    println!("Is underage: {underage}.");

    // loop
    let mut decades: i32 = 0;
    {
        let mut age = age;
        loop {
            if age >= 10 {
                age -= 10;
                decades += 1;
            } else {
                break;
            }
        }
    }
    println!("Lived for {decades} decades.");

    if age < 3 {
        println!("Not in school.");
    } else if age >= 3 && age <= 5 {
        println!("Kindergarden");
    } else if age >= 6 && age <= 10 {
        println!("Elementary school");
    } else if age >= 11 && age <= 13 {
        println!("Middle school");
    } else if age >= 14 && age <= 18 {
        println!("High school");
    } else {
        println!("Not in school.")
    }
}
