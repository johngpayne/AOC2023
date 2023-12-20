pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        nums[0]
    } else {
        let inner = lcm(&nums[1..]);
        nums[0] * inner / gcd(nums[0], inner)
    }
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
