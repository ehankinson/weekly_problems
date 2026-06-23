#[path = "../dany/mod.rs"]
pub mod dany;

#[path = "../ethan/mod.rs"]
pub mod ethan;

#[path = "../farzin/mod.rs"]
pub mod farzin;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
