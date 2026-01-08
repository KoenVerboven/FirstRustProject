fn main() {
    let yourname = "Koen Verboven";
    let earth_weight: f32 = 75.0; // Example weight on Earth in kg

    println!("Koen Verboven 8 januari 2026");

    println!(
        "Weight on the Earth of {:?}: {:.2} kg",
        yourname, earth_weight
    );

    let moon_weight = calculate_weight_on_moon(earth_weight);
    println!(
        "Weight on the Moon of {:?}: {:.2} kg",
        yourname, moon_weight
    );
}

fn calculate_weight_on_moon(earth_weight: f32) -> f32 {
    let moon_gravity_factor: f32 = 0.165;
    earth_weight * moon_gravity_factor
}
