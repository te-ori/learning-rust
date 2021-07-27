mod common;
pub use common::level::Level as Level;

pub mod lessons;

#[derive(Debug)]
pub struct Name(String);

#[cfg(test)]
mod test {
    #[test]
    fn scholl_lib() {
        println!("scholl::lib");
        println!("first: {:?}", super::lessons::Lesson::new_first("this is first".to_string()));
        println!("second: {:?}", super::lessons::Lesson::new_second("this is second".to_string()));
    }
}