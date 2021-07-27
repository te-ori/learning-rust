extern crate school;
pub use school::lessons::Lesson;

#[cfg(test)]
mod test {
    #[test]
    fn library_test() {
        println!("library::lib");
        println!("first: {:?}", super::Lesson::new_first("this is first library".to_string()));
        println!("second: {:?}", super::Lesson::new_second("this is second library".to_string()));
    }
}