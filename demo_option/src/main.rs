fn main() {
    let name = String::from("Mohammed_Hoque");
    let letter = name.chars().nth(15);

    // match similar to C# switch statement
    let value = match letter{
        Some(character) => character.to_string(),
        None => String::from("--- Not Found ---")
    };

    println!("Value: {}", value);
}
