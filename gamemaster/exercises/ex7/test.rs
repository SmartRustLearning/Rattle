#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_return() {
        let s = "Hello Rustfi";
        
        assert_eq!(s, dangle());
    }
}