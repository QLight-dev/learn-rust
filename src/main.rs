fn main() {
    // Referencing
    let word = String::from("Bonjour");
    let translated = translate_word(&word);
    println!("{} is in {}", word, translated);
}

fn translate_word(word: &str) -> String {
    if word == "Hello" {
        return "English".to_string();
    } else if word == "Bonjour" {
        return "French".to_string();
    } else {
        return "Unknown language".to_string();
    }
}
