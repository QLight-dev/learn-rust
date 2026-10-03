fn main() {
    // Mutability
    let mut some_random_variable = "hi";
    println!("the secret message is {some_random_variable}");
    some_random_variable = "bye";
    println!("the new secret message is {some_random_variable}");

    // Constants
    const SOME_RANDOM_CONST: &str = "hi this is some useless const";
    println!("const data: {SOME_RANDOM_CONST}");

    // Shadowing
    let one = 1;
    {
        let one = one * 5;
        println!("{one}");
    }
    println!("{one}");
}
