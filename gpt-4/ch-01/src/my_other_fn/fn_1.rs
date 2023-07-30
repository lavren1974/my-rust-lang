pub fn sum_fn10(mut x: u32) -> u32 {
    x += 10;
    x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sum_fn10_test() {
        let x: u32 = 100;
        let y: u32 = sum_fn10(x);
        println!("Test sum_fn10_test: {}", y);
        assert_eq!(y, 110);
    }
}
