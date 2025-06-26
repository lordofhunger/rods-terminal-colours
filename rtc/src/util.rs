use rand::Rng;

pub fn generate_random_colour_hex() -> String {
    let mut bytes = [0u8; 3];
    rand::rng().fill(&mut bytes);
    bytes.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    if hex.len() != 6 {
        return Err("Invalid hex format. Use RRGGBB.".to_string());
    }
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component".to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component".to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component".to_string())?;
    Ok((r, g, b))
}

pub fn inverted_hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    let (r, g, b) = hex_to_rgb(hex)?;
    Ok((255 - r, 255 - g, 255 - b))
}