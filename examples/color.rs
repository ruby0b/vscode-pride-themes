use std::collections::HashMap;

struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

fn main() {
    let mut colors = HashMap::new();

    // Insert some colors.
    colors.insert(
        "blue".to_string(),
        Color { r: 85, g: 205, b: 252 }
    );
    colors.insert(
        "pink".to_string(),
        Color { r: 247, g: 168, b: 184 }
    );

    // Look up some colors.
    for &name in &["pink", "green"] {
        match colors.get(name) {
            Some(c) => println!("RGB for {}: {}, {}, {}", name, c.r, c.g, c.b),
            None    => println!("I don't know what color {} is!", name),
        }
    }
}

