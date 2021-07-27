#[derive(Debug)]
pub enum Level {
    First,
    Second,
    Third,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn level_test() {
        println!("scholl::common::level");
        println!("{:?}", Level::First);
        println!("{:?}", Level::Second);
        println!("{:?}", Level::Third);
    }
}