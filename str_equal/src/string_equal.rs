use std::collections::VecDeque;

#[cfg(test)]
mod test {
    #[test]
    fn t_1() {
        let v1 = vec![
            "ab".to_string(),
            "c".to_string(),
            "d".to_string(),
            "ef".to_string(),
        ];

        let v2 = vec!["abc".to_string(), "de".to_string(), "f".to_string()];

        assert!(super::array_strings_are_equal(v1, v2));
    }

    #[test]
    fn t_2() {
        let v1 = vec![
            "ab".to_string(),
            "c".to_string(),
            "d".to_string(),
            "ef".to_string(),
        ];

        let v2 = vec!["abc".to_string(), "de".to_string(), "f".to_string()];

        assert!(super::array_strings_are_equal2(v1, v2));
    }
}

pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    word1.concat() == word2.concat()
}

pub fn array_strings_are_equal2(word1: Vec<String>, word2: Vec<String>) -> bool {
    let mut v1: VecDeque<Option<char>> = VecDeque::new();
    let mut v2: VecDeque<Option<char>> = VecDeque::new();
    let mut it1 = word1.iter();
    let mut it2 = word2.iter();
    let mut cnt = 0;
    let r = loop {
        println!("{} loop started", cnt);
        if v1.len() < 1 {
            match it1.next() {
                Some(s) => {
                    println!("it1: {}", s);
                    s.chars().for_each(|c| v1.push_back(Some(c)));
                }
                None => {
                    println!("it1 none");
                    v1.push_back(None);
                }
            }
        }

        if v2.len() < 1 {
            match it2.next() {
                Some(s) => {
                    println!("it2: {}", s);
                    s.chars().for_each(|c| v2.push_back(Some(c)));
                }
                None => {
                    println!("it1 none");
                    v2.push_back(None);
                }
            }
        }

        let c1 = v1.pop_front();
        let c2 = v2.pop_front();
        println!("c1 {:?} c2: {:?}", c1, c2);
        cnt += 1;

        if c1 != c2 {
            break false;
        }

        if c1 == Some(None) {
            break true;
        }

        if cnt > 10 {
            break false;
        }
    };

    r
}
