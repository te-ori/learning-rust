// use crate::Level;

#[derive(Debug)]
pub struct Lesson {
    pub name: crate::Name,
    pub level: crate::Level,
}

impl Lesson {
    pub fn new_first(name: String) -> Lesson {
        Lesson {
            name: crate::Name(name),
            level: crate::Level::First,
        }
    }
    pub fn new_second(name: String) -> Lesson {
        Lesson {
            name: crate::Name(name),
            level: crate::Level::Second,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn lesson_test() {
        println!("scholl::lessons::lesson");
        println!("first: {:?}", super::Lesson::new_first("this is first".to_string()));
        println!("second: {:?}", super::Lesson::new_second("this is second".to_string()));
    }
}