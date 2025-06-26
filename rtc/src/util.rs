use rand::Rng;

pub fn generate_random_colour_hex() -> String {
    let mut bytes = [0u8; 3];
    rand::rng().fill(&mut bytes);
    bytes.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}