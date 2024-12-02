use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file).lines();
    let safe_reports = 0;
    lines
    .filter(|l| {
        if let Ok(line) = l {
            let values: Vec<i8> = line
                .split_whitespace()
                .map(|v| v.parse::<i8>().unwrap())
                .collect();

    // .count();
    println!("{safe_reports}");
    Ok(())
}

// compare value to previous
// same

fn get_readings_direction(experiments: &impl Iterator<Item = i8>) -> i8 {
    let mut previous_value = None;
    let mut direction = 0;
    experiments.take(3).for_each(|n| {
        if let Some(a) = previous_value {
            if a != previous_value {
                return a - previous_value;
            }
        } else {
            previous_value = Some(n);
        }
    });
    direction
}

fn determine_saftey(expiriments: impl Iterator<Item = i8>) -> bool {
    let mut safe = true;
    let direction = get_readings_direction(&expiriments);
    let mut bad_levels: u8 = 0;
    let mut current_value: Option<i8> = None;
    expiriments.for_each(|n| {
        if let Some(previous_value) = current_value {
            let result = n - previous_value;
            let difference = previous_value.abs_diff(n);
            let mut proper_direction = true;
            let mut proper_distance = true;

            if let Some(dir) = direction {
                if (result.is_positive() && dir.is_negative())
                    || (result.is_negative() && dir.is_positive())
                {
                    proper_direction = false;
                }
            }

            if difference == 0 || difference > 3 {
                proper_distance = false;
            }

            direction = Some(result);
            current_value = Some(n);
            if !proper_direction || !proper_distance {
                bad_levels += 1;
            }
        } else {
            current_value = Some(n);
        }

        match bad_levels {
            0 => (),
            1 => {
                safe = false;
            }
            _ => safe = false,
        }
    });
    safe
}
