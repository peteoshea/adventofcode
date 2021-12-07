fn split_diagnotic_into_bits(diagnostic: String) -> Vec<u32> {
    let mut result = Vec::new();
    for digit in diagnostic.chars() {
        result.push(digit.to_string().parse().unwrap());
    }
    result
}

pub fn calculate_rates(diagnostics: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_calculate_rates_with_example_returns_expected_bits() {
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
        assert_eq!(calculate_rates(&diagnostics), (vec![1, 0, 1, 1, 0], vec![0, 1, 0, 0, 1]));
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
