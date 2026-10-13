fn print_hi() {
    println!("hi");
}

fn print_number(number: i32) {
    println!("{number}");
}

fn print_initials(first: char, last: char) {
    println!("{first}.{last}")
}

fn approximate_pi() -> f32 {
    3.1415926535897932384
}

fn main() {
    // functions
    print_hi();

    // parameters
    print_number(5);

    print_initials('M', 'U');

    // Functions with return values
    let pi = approximate_pi();
    println!("pi: {pi}");
}
