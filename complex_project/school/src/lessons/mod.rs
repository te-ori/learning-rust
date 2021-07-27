mod lesson;
pub use lesson::Lesson as Lesson;

#[cfg(test)]
mod test {
    #[test]
    fn lessons_module_test() {
        println!("scholl::lessons::mod");
        println!("first: {:?}", super::Lesson::new_first("this is lol1".to_string()));
        println!("second: {:?}", super::Lesson::new_second("this is lol2".to_string()));
    }
}