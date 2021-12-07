fn split_diagnotic_into_bits(diagnostic: String) -> Vec<u32> {
    let mut result = Vec::new();
    for digit in diagnostic.chars() {
        result.push(digit.to_string().parse().unwrap());
    }
    result
}

pub fn calculate_power_rates(diagnostics: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut gamma_rate = Vec::new();
    let mut epsilon_rate = Vec::new();
    let mut bit_counts: Vec<u32> = Vec::new();
    let mut threshold: u32 = 0;
    let mut update_threshold: bool = true;
    let mut first: bool = true;
    for diagnostic in diagnostics.iter() {
        if update_threshold {
            threshold += 1;
        }
        update_threshold = !update_threshold;
        for (index, bit) in split_diagnotic_into_bits(diagnostic.to_string())
            .iter()
            .enumerate()
        {
            if first {
                bit_counts.push(*bit);
            } else {
                bit_counts[index] += bit;
            }
        }
        if first {
            first = !first;
        }
    }
    for bit_count in bit_counts {
        let mut bit = 0;
        if bit_count >= threshold {
            bit = 1;
        }
        gamma_rate.push(bit);
        epsilon_rate.push(invert_bit(bit));
    }
    (gamma_rate, epsilon_rate)
}

fn invert_bit(bit: u32) -> u32 {
    let mut result = 0;
    if bit == 0 {
        result = 1;
    }
    result
}

pub fn convert_bits_to_decimal(bits: Vec<u32>) -> u32 {
    let mut bit_multiplier: u32 = 1;
    let mut result = 0;
    for bit in bits.iter().rev() {
        result += bit * bit_multiplier;
        bit_multiplier *= 2;
    }
    result
}

pub fn calculate_life_support_values(diagnostics: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut stripped_list = diagnostics.to_vec();
    let mut bit_index: usize = 0;
    // println!("Oxygen calculation:");
    while stripped_list.len() > 1 {
        let (gamma, _epsilon) = calculate_power_rates(&stripped_list);
        let bit_value = gamma[bit_index];
        // println!("Bit_index: {} value: {}", bit_index, bit_value);
        stripped_list = strip_items(&stripped_list, bit_index, bit_value);
        // println!("Updated list: {:?}", stripped_list);
        bit_index += 1;
    }
    let oxygen_generator = split_diagnotic_into_bits(stripped_list[0].to_string());
    stripped_list = diagnostics.to_vec();
    bit_index = 0;
    // println!("CO2 calculation:");
    while stripped_list.len() > 1 {
        let (_gamma, epsilon) = calculate_power_rates(&stripped_list);
        let bit_value = epsilon[bit_index];
        // println!("Bit_index: {} value: {}", bit_index, bit_value);
        stripped_list = strip_items(&stripped_list, bit_index, bit_value);
        // println!("Updated list: {:?}", stripped_list);
        bit_index += 1;
    }
    let co2_scrubber = split_diagnotic_into_bits(stripped_list[0].to_string());
    (oxygen_generator, co2_scrubber)
}

fn strip_items(diagnostics: &Vec<String>, bit_index: usize, value: u32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for diagnostic in diagnostics.iter() {
        let bits = split_diagnotic_into_bits(diagnostic.to_string());
        if bits[bit_index] == value {
            result.push(diagnostic.to_string());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_items_for_first_bit_with_example_data_returns_expected_vector() {
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
        let bit_index: usize = 0;
        let bit_value: u32 = 1;
        assert_eq!(
            strip_items(&diagnostics, bit_index, bit_value),
            vec![
                "11110".to_string(),
                "10110".to_string(),
                "10111".to_string(),
                "10101".to_string(),
                "11100".to_string(),
                "10000".to_string(),
                "11001".to_string(),
            ]
        );
    }

    #[test]
    fn test_strip_items_for_second_bit_with_example_data_returns_expected_vector() {
        let diagnostics: Vec<String> = vec![
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
        ];
        let bit_index: usize = 1;
        let bit_value: u32 = 0;
        assert_eq!(
            strip_items(&diagnostics, bit_index, bit_value),
            vec![
                "10110".to_string(),
                "10111".to_string(),
                "10101".to_string(),
                "10000".to_string(),
            ]
        );
    }

    #[test]
    fn test_calculate_life_support_values_with_example_data_returns_expected_bits() {
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
        assert_eq!(
            calculate_life_support_values(&diagnostics),
            (vec![1, 0, 1, 1, 1], vec![0, 1, 0, 1, 0])
        );
    }

    #[test]
    fn test_convert_bits_to_decimal_with_example_gamma_returns_22() {
        assert_eq!(convert_bits_to_decimal(vec![1, 0, 1, 1, 0]), 22);
    }

    #[test]
    fn test_invert_bit_with_zero_returns_one() {
        assert_eq!(invert_bit(0), 1);
    }

    #[test]
    fn test_invert_bit_with_one_returns_zero() {
        assert_eq!(invert_bit(1), 0);
    }

    #[test]
    fn test_calculate_power_rates_with_example_returns_expected_bits() {
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
        assert_eq!(
            calculate_power_rates(&diagnostics),
            (vec![1, 0, 1, 1, 0], vec![0, 1, 0, 0, 1])
        );
    }

    #[test]
    fn test_split_diagnotic_into_bits_with_long_string_returns_vector_of_numbers() {
        assert_eq!(
            split_diagnotic_into_bits("001001101011".to_string()),
            vec![0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1]
        );
    }

    #[test]
    fn test_split_diagnotic_into_bits_returns_vector_of_numbers() {
        assert_eq!(
            split_diagnotic_into_bits("00100".to_string()),
            vec![0, 0, 1, 0, 0]
        );
    }
}
