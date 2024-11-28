extern crate minreq;
use std::error::Error;

fn make_request() -> Result<String, Box<dyn Error>> {
    // Send the GET request to the URL
    let response = minreq::get("https://eapi.binance.com/eapi/v1/ticker").send()?;

    // Check if the response status code is 200 (OK)
    if response.status_code != 200 {
        return Err(format!("Error: Received status code {}", response.status_code).into());
    }

    // Get the body of the response as a String
    let body: String = String::from_utf8_lossy(response.as_bytes()).to_string(); // this extracts the body as a string

    Ok(body)
}

// Assumes a nice structured JSON, with no spaces etc...
fn parse_json_value(json_bytes: &[u8], key_bytes: &[u8], position: &mut usize) -> Option<String> {
    // Find the position of the key in the byte slice (no allocations)
    if let Some(key_start) = efficient_find_bytes(json_bytes, key_bytes, *position) {
        // Move past the key and colon
        let value_start = *position + key_start + key_bytes.len();

        // Try to find where the value ends (looking for comma or closing brace)
        let value_end = json_bytes[value_start..]
            .iter()
            .position(|&b| b == b',' || b == b'}')
            .unwrap_or(json_bytes.len() - value_start);

        let value_bytes = &json_bytes[value_start..value_start + value_end];

        // Next start position for a search
        *position = value_start + value_end + 1;

        // If the value is wrapped in quotes, strip them
        if value_bytes.starts_with(&[b'"']) && value_bytes.ends_with(&[b'"']) {
            return Some(
                String::from_utf8_lossy(&value_bytes[1..value_bytes.len() - 1]).into_owned(),
            );
        }
        Some(String::from_utf8_lossy(value_bytes).into_owned())
    } else {
        None
    }
}

/*
Find the position of a byte slice inside (needle) in another byte slice (haystack)
fn naive_find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    // If the needle is empty, return the start position (0)
    if needle.is_empty() {
        return Some(0);
    }

    // Loop through the haystack and check each possible starting position
    for i in start..=haystack.len() - needle.len() {
        let mut match_found = true;

        // Compare the slice of the haystack starting at position i with the needle
        for j in 0..needle.len() {
            if haystack[i + j] != needle[j] {
                match_found = false;
                break; // If there's a mismatch, stop comparing and move to the next position
            }
        }

        if match_found {
            return Some(i-start); // Return the position if a match is found
        }
    }

    None // Return None if no match is found
}
*/

// Find the position of a byte (needle) in another byte slice (haystack)
// TODO: A SIMD approach could be used to compare e.g. 32 bytes at once
fn efficient_find_bytes(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    haystack[start..]
        .windows(needle.len())
        .position(|window| window == needle)
}

fn main() {
    use std::time::Instant;
    let mut now;
    let mut elapsed;
    match make_request() {
        Ok(response) => {
            //println!("{}", response);
            let mut position: usize = 0;
            let key = "closeTime";

            let key_string = format!("\"{}\":", key);
            let key_bytes = key_string.as_bytes();

            loop {
                now = Instant::now();
                match parse_json_value(response.as_ref(), key_bytes, &mut position) {
                    Some(value) => {
                        elapsed = now.elapsed();

                        println!(
                            "The value of {0:<10}  {1:<16} pos {2:<10} {3:.2?}",
                            key, value, position, elapsed
                        );
                    }
                    None => {
                        println!("Key '{}' not found", key);
                        break;
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Request failed: {}", err);
        }
    }
}
