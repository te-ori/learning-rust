mod intro {
    extern crate rand;

    fn simulate_game<'a>(home: &'a str, away: &'a str) -> &'a str {
        if rand::random() {
            home
        } else {
            away
        }
    }

    pub fn play() {
        let team1 = "t1".to_string();
        {
            let team2 = "t2".to_string();
            let winner = simulate_game(&team1, &team2);

            println!("{} vs {}: {} won", team1, team2, winner);
        }

        println!("Can still access {}", team1);
    }

    // pub fn cant_play() {
    //     let t1 = "t1".to_string();

    //     let winner = {
    //         let t2 = "t2".to_string();

    //         simulate_game(&t1, &t2)
    //     };

    //     println!("{} won", winner);
    // }

    fn t<'a>(a: &'a mut String, c: &mut String) -> &'a String {
        if c.len() > 0 && a.len() > 0 {
            a.push('*');
        } else {
            a.push('#')
        }

        a
    }

    fn id(a: i32) -> i32 {
        a
    }

    fn call_id() {
        println!("{}", id(1));

        let n = 5;
        println!("{}", id(n));

        println!("{}", id(1 + 2));
        println!("{}", id(i32::pow(4, 4)));
    }

    fn id_vec(a: Vec<i32>) -> Vec<i32> {
        a
    }

    fn modify_vec_and_return(mut a: Vec<i32>) -> Vec<i32> {
        if a.len() > 0 {
            a[0] = a[0] * 2;
        }

        a
    }

    fn modify_vec_but_not_return(mut a: Vec<i32>) {
        if a.len() > 0 {
            a[0] = a[0] * 2;
        }
    }

    pub fn use_vec_funcs() {
        let v1 = vec![1, 2, 3];
        println!("{:?}", v1);
        println!("{:?}", id_vec(v1));
        // println!("{:?}", id_vec(v1)); // ~~ v1'in ownershipliği fonksiyon çağrısı sırasında
        // ~~ fonksiyon parametresine geçti. Bu nedenle `v1`
        // ~~ hala stack'te olmasına rağmen boş herhangi bir
        // ~~ değeri refer etmiyor. Onun refere ettiği
        // ~~ değerin ownerlığı fonksiyon parametresinden
        // ~~ başka bir yere atanmadığı için `id_vec`'in
        // ~~ çalışması bittiğinde bellekten silinecek.
        // ~~ `v1`'de herhangi bir yeri refere etmediği için
        // ~~ tekrar kullanılamaz.
        let mut v2 = vec![2, 3, 4];
        println!("{:?}", v2);
        println!("{:?}", id_vec(v2));

        v2 = vec![5, 6, 7];
        println!("{:?}", v2);
    }

    fn gf<T>(v: T) -> Option<T> {
        Some(v)
    }
}

mod example {
    // fn ex1(a: i32, name: &str, v: &[i32]) -> &str {
    //     name
    // }
    fn ex1_fix_1<'a>(a: i32, name: &'a str, v: &'a [i32]) -> &'a str {
        &name
    }
    fn ex1_fix_2<'a, 'b>(a: i32, name: &'a str, v: &'b [i32]) -> &'a str {
        &name
    }
    // fn ex1_not_fix_1<'a, 'b>(a: i32, name: &'a str, v: &'b [i32]) -> &'b str {
    //     &name
    // }
}