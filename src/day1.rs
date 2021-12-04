use super::common;

pub fn run() {
    let lines = common::file_to_vec("./data/day1/input.txt").unwrap();
    let depths: Vec<u16> = lines.into_iter().map(|line| line.parse().unwrap()).collect();
    println!("Day1: {}", increase_count(depths));
}

fn depth_increase(depth1: u16, depth2: u16) -> bool {
    depth2 > depth1
}

#[allow(dead_code)]
fn increase_count(depths: Vec<u16>) -> u16 {
    let mut count = 0;
    // Don't initialise first depth as will be written to on first loop
    let mut depth1: u16;
    // Write first element to second depth so it gets pushed to depth1 on first pass
    let mut depth2: u16 = depths[0];
    for depth in depths.iter().skip(1) {
        depth1 = depth2;
        depth2 = *depth;
        if depth_increase(depth1, depth2) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_increase_with_same_values_returns_false() {
        assert!(!depth_increase(200, 200));
    }

    #[test]
    fn test_depth_increase_with_decreasing_depth_returns_false() {
        assert!(!depth_increase(200, 190));
    }

    #[test]
    fn test_depth_increase_with_increasing_depth_returns_true() {
        assert!(depth_increase(190, 200));
    }

    #[test]
    fn test_increase_count_with_no_increases_returns_zero() {
        let depths: Vec<u16> = vec![199, 198, 197];
        assert_eq!(increase_count(depths), 0);
    }

    #[test]
    fn test_increase_count_with_example_returns_seven() {
        let depths: Vec<u16> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(increase_count(depths), 7);
    }
}
