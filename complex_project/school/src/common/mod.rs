pub mod level;

#[cfg(test)]
mod test {
    #[test]
    fn common_test() {
        println!("scholl::common::mod");
        println!("common module test");
    }

    #[test]
    fn level_in_common_module() {
        println!("scholl::common::mod");
        println!("level in common module: {:?}", super::level::Level::Third);
    }
}