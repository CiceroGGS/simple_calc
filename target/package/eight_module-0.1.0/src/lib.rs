pub mod calc_near_x {

    /// # Essa funcao faz uma soma e adiciona 1
    pub fn sum_plus_one(x: u8, y:u8) -> u8 {
        x + y + 1
    }

    /// # Essa funcao faz uma subtracao e subtrai 1
    pub fn sub_less_one(x: u8, y:u8) -> u8 {
        if x <= y {
            return 0;
        }
        x - y - 1
    }
}

#[cfg(test)]
mod test {
    use super::calc_near_x;

    #[test]
    fn test_sum() {
        let result = calc_near_x::sum_plus_one(5, 6);
        let expected = 12;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_sub() {
        let result = calc_near_x::sub_less_one(5, 6);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_failed() {
        let result = calc_near_x::sub_less_one(6, 6);
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sub_failed2() {
        let result = calc_near_x::sub_less_one(6, 1);
        let expected = 4;
        assert_eq!(result, expected);
    }

}