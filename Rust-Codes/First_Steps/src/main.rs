fn main() {
    let text = String::from("Basico Rust");
    let length = length_str(&text);
    let is_prime = if is_prime(length) { "Es" } else { "No es" };

    println!(
        "El string {} Tiene una longitud de  {} y {} un número primo",
        text, length, is_prime
    );
}

fn length_str(string: &str) -> usize {
    string.len()
}

fn is_prime(length: usize) -> bool {
    if length <= 1 {
        return false;
    }
    for i in 2..length {
        if length % i == 0 {
            return false;
        }
    }
    true
}
