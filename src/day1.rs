pub fn run() {
    println!("Okay lets do this thing...")
}

#[allow(dead_code)]
fn depth_increase(depth1: u32, depth2: u32) -> bool {
  let mut result = false;
  if depth2 > depth1 {
    result = true;
  }
  result
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
  fn test_depth_increase_with_increasing_depth_returns_truee() {
    assert!(depth_increase(190, 200));
  }
}
