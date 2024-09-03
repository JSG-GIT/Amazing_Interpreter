use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::fs::OpenOptions;
use std::io::Write;

pub fn convert(path: &str) -> Result<(), Box<dyn Error>> {
    // Path to the input CSV file
    let input_path = path;

    // Path to the output CSV file
    let output_path = path;

    // Open the input CSV file
    let input_file = File::open(input_path)?;
    let reader = io::BufReader::new(input_file);

    // Read all values from the input CSV into a vector
    let mut values: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        for value in line.split(';') {
            values.push(value.to_string());
        }
    }

    // Ensure we have exactly 400 values
    if values.len() != 400 {
        return Err(Box::from("The input CSV must contain exactly 400 values."));
    }

    // Collect and map the 'D' values
    let mut d_values: Vec<&str> = values.iter()
        .filter(|v| v.starts_with('D') && v[1..].parse::<u8>().map_or(false, |num| num >= 1 && num <= 9))
        .map(|v| v.as_str())
        .collect();

    // Sort the 'D' values based on their numeric suffix and reassign them to the lowest possible values
    d_values.sort_by_key(|v| v[1..].parse::<u8>().unwrap());

    let mut d_value_map = HashMap::new();
    for (i, &value) in d_values.iter().enumerate() {
        let new_value = format!("D{}", i + 1);
        d_value_map.insert(value, new_value);
    }

    // Apply the remapping to the original values using references
    let transformed_values: Vec<String> = values.iter()
        .map(|v| {
            if let Some(new_value) = d_value_map.get(v.as_str()) {
                new_value.clone()
            } else {
                v.clone()
            }
        })
        .collect();

    // Open the output CSV file
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;

    // Write the reshaped data to the output CSV with semicolons as separators
    for row in 0..20 {
        let start = row * 20;
        let end = start + 20;
        let row_values = &transformed_values[start..end];
        writeln!(output_file, "{}", row_values.join(";"))?;
    }

    Ok(())
}
