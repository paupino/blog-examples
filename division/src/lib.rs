pub fn div(dividend: u32, divisor: u32) -> (u32, u32) {
    // Get the two's complement of the number
    let ones_complement = divisor ^ 0xFFFF_FFFF;
    let twos_complement = ones_complement + 1;

    // Keep subtracting from the dividend until we can't anymore
    let mut working = dividend;
    let mut quotient = 0;
    while divisor <= working {
      working = working.wrapping_add(twos_complement);
      quotient += 1;
    }
    let remainder = working; // Leftover
    (quotient, remainder)
}

pub fn div2(dividend: u32, divisor: u32) -> (u32, u32) {
    // Get the two's complement of the number
    let ones_complement = divisor ^ 0xFFFF_FFFF;
    let twos_complement = ones_complement + 1;

    // Start off the quotient with the dividend
    let mut quotient = dividend;
    let mut remainder = 0u32;
    
    // Loop through the number of bits in the number (32)
    for _ in 0..32 {
      let carry = quotient >> 31;
      quotient = quotient << 1;
      remainder = remainder << 1;
      remainder = remainder | carry;

      let test = remainder.wrapping_add(twos_complement);
      if (test & 0x8000_0000) == 0 {
          remainder = test;
          quotient |= 1;
      }
    }

    (quotient, remainder)
}

#[cfg(test)]
mod tests {
    use super::{div, div2};

    #[test]
    fn naive_div() {
        assert_eq!(div(100, 4), (25, 0));
        assert_eq!(div(100, 3), (33, 1));
    }

    #[test]
    fn faster_div() {
        assert_eq!(div2(100, 4), (25, 0));
        assert_eq!(div2(100, 3), (33, 1));
    }
}
