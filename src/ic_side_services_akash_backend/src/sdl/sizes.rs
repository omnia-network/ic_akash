use std::ops::Mul;

const PREFIXES: [&str; 6] = ["k", "m", "g", "t", "p", "e"];

fn parse_size_string(size: &str) -> Result<(f64, String, String), String> {
    let regex = regex::Regex::new(r"^([\d.]+)([a-zA-Z])([a-zA-Z]*)$").unwrap();
    if let Some(captures) = regex.captures(size) {
        let value = captures.get(1).unwrap().as_str().parse::<f64>().unwrap();
        let unit1 = captures.get(2).unwrap().as_str().to_lowercase();
        let unit2 = captures.get(3).unwrap().as_str().to_lowercase();
        Ok((value, unit1, unit2))
    } else {
        Err(format!("Invalid size string: {}", size))
    }
}

pub fn convert_resource_string(resource_str: &str) -> Result<f64, String> {
    let (value, prefix, unit) = match parse_size_string(&resource_str.to_lowercase()) {
        Ok(result) => result,
        Err(err) => return Err(err),
    };

    let power = PREFIXES.iter().position(|p| *p == prefix);
    let base: u32 = match unit.as_str() {
        "i" => 1024,
        _ => 1000,
    };

    Ok(match power {
        Some(power) => {
            value.mul(f64::try_from(base.pow(power as u32 + 1)).map_err(|e| e.to_string())?)
        }
        None => value,
    })
}

fn parse_cpu_resource_string(size: &str) -> Result<(f64, String), String> {
    let regex = regex::Regex::new(r"^([\d.]+)([a-zA-Z]*)$").unwrap();
    if let Some(captures) = regex.captures(size) {
        let value = captures.get(1).unwrap().as_str().parse::<f64>().unwrap();
        let unit = captures.get(2).unwrap().as_str().to_lowercase();
        Ok((value, unit))
    } else {
        Err(format!("Invalid size string: {}", size))
    }
}

pub fn convert_cpu_resource_string(str: &str) -> Result<u32, String> {
    let (value, unit) = parse_cpu_resource_string(&str.to_lowercase())?;

    Ok(match unit.as_str() {
        "m" => value as u32,
        _ => (value * 1000.0) as u32,
    })
}
