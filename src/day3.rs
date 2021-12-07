use super::common;
mod calc;

pub fn run() {
    let lines = common::file_to_vec("./data/day3/input.txt").unwrap();
    let (gamma, epsilon) = calculate_power_consumption(&lines);
    println!(
        "Day3: part 1 = gamma rate: {}, epsilon rate: {} => {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    let (oxygen_generator, co2_scrubber) = calculate_life_support_rating(&lines);
    println!(
        "Day3: part 2 = oxygen generator rating: {}, CO2 scrubber rating: {} => {}",
        oxygen_generator,
        co2_scrubber,
        oxygen_generator * co2_scrubber
    );
}

fn calculate_power_consumption(diagnostics: &Vec<String>) -> (u32, u32) {
  let (gamma_rate, epsilon_rate) = calc::calculate_power_rates(diagnostics);
  (
    calc::convert_bits_to_decimal(gamma_rate),
    calc::convert_bits_to_decimal(epsilon_rate),
  )
}

fn calculate_life_support_rating(diagnostics: &Vec<String>) -> (u32, u32) {
  let (oxygen_generator, co2_scrubber) = calc::calculate_life_support_values(diagnostics);
  (
    calc::convert_bits_to_decimal(oxygen_generator),
    calc::convert_bits_to_decimal(co2_scrubber),
  )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_life_support_rating_with_example_returns_expected_values() {
        let diagnostics: Vec<String> = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        assert_eq!(calculate_life_support_rating(&diagnostics), (23, 10));
    }

    #[test]
    fn test_calculate_power_consumption_with_example_returns_expected_values() {
        let diagnostics: Vec<String> = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        assert_eq!(calculate_power_consumption(&diagnostics), (22, 9));
    }
}
