use super::common;

pub fn run() {
    let lines = common::file_to_vec("./data/day3/input.txt").unwrap();
    let (gamma, epsilon) = calculate(&lines);
    println!(
        "Day3: part 1 = gamma rate: {}, epsilon rate: {} => {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn calculate(diagnostics: &Vec<String>) -> (u32, u32) {
    let (gamma_rate, epsilon_rate) = calculate_rates(diagnostics);
    (
        convert_bits_to_decimal(gamma_rate),
        convert_bits_to_decimal(epsilon_rate),
    )
}

fn calculate_gamma_rate(diagnostics: &Vec<String>) -> (u32, u32, u32, u32, u32) {
    let mut bit1sum: u32 = 0;
    let mut bit2sum: u32 = 0;
    let mut bit3sum: u32 = 0;
    let mut bit4sum: u32 = 0;
    let mut bit5sum: u32 = 0;
    let mut threshold: u32 = 0;
    let mut update_threshold: bool = true;
    for diagnostic in diagnostics.iter() {
        if update_threshold {
            threshold += 1;
        }
        update_threshold = !update_threshold;
        let (bit1, bit2, bit3, bit4, bit5) = split_diagnotic_into_bits(diagnostic.to_string());
        bit1sum += bit1;
        bit2sum += bit2;
        bit3sum += bit3;
        bit4sum += bit4;
        bit5sum += bit5;
    }
    let mut bit1 = 0;
    let mut bit2 = 0;
    let mut bit3 = 0;
    let mut bit4 = 0;
    let mut bit5 = 0;
    if bit1sum >= threshold {
        bit1 = 1;
    }
    if bit2sum >= threshold {
        bit2 = 1;
    }
    if bit3sum >= threshold {
        bit3 = 1;
    }
    if bit4sum >= threshold {
        bit4 = 1;
    }
    if bit5sum >= threshold {
        bit5 = 1;
    }
    (bit1, bit2, bit3, bit4, bit5)
}

fn split_diagnotic_into_bits(diagnostic: String) -> (u32, u32, u32, u32, u32) {
    let mut digits = diagnostic.chars();
    let bit1: u32 = digits.next().unwrap().to_string().parse().unwrap();
    let bit2: u32 = digits.next().unwrap().to_string().parse().unwrap();
    let bit3: u32 = digits.next().unwrap().to_string().parse().unwrap();
    let bit4: u32 = digits.next().unwrap().to_string().parse().unwrap();
    let bit5: u32 = digits.next().unwrap().to_string().parse().unwrap();
    (bit1, bit2, bit3, bit4, bit5)
}

fn calculate_rates(
    diagnostics: &Vec<String>,
) -> ((u32, u32, u32, u32, u32), (u32, u32, u32, u32, u32)) {
    let gamma_rate = calculate_gamma_rate(diagnostics);
    let epsilon_rate = invert_bits(gamma_rate);
    (gamma_rate, epsilon_rate)
}

fn invert_bits(bits: (u32, u32, u32, u32, u32)) -> (u32, u32, u32, u32, u32) {
    let (bit1, bit2, bit3, bit4, bit5) = bits;
    (
        invert_bit(bit1),
        invert_bit(bit2),
        invert_bit(bit3),
        invert_bit(bit4),
        invert_bit(bit5),
    )
}

fn invert_bit(bit: u32) -> u32 {
    let mut result = 0;
    if bit == 0 {
        result = 1;
    }
    result
}

fn convert_bits_to_decimal(bits: (u32, u32, u32, u32, u32)) -> u32 {
    let (bit1, bit2, bit3, bit4, bit5) = bits;
    bit5 + (bit4 * 2) + (bit3 * 4) + (bit2 * 8) + (bit1 * 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_bits_to_decimal_with_example_gamma_returns_22() {
        assert_eq!(convert_bits_to_decimal((1, 0, 1, 1, 0)), 22);
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
    fn test_invert_bits_returns_expected_bits() {
        assert_eq!(invert_bits((1, 0, 1, 1, 0)), (0, 1, 0, 0, 1));
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
        assert_eq!(
            calculate_rates(&diagnostics),
            ((1, 0, 1, 1, 0), (0, 1, 0, 0, 1))
        );
    }

    #[test]
    fn test_split_diagnotic_into_bits() {
        assert_eq!(
            split_diagnotic_into_bits("00100".to_string()),
            (0, 0, 1, 0, 0)
        );
    }

    #[test]
    fn test_calculate_gamma_rate_with_example_returns_expected_bits() {
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
        assert_eq!(calculate_gamma_rate(&diagnostics), (1, 0, 1, 1, 0));
    }

    #[test]
    fn test_calculate_with_example_returns_expected_values() {
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
        assert_eq!(calculate(&diagnostics), (22, 9));
    }
}
