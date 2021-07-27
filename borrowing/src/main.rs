#[derive(Debug)]
struct S(i32);

impl Drop for S {
    fn drop(&mut self) {
        println!("dropped {}.", self.0);
    }
}

fn main() {
    // simple_move();
    // reference();
    // in_block_assignment();
    // orders_of_drop();
    // referencinf_wrong_order();
    // some_borrowing_checks();
    // some_borrowings();
    some_valid_borrowings();
}

fn simple_move() {
    {
        let a = S(1);
        let b = a;
        println!("{:?}", b);
    }

    println!("lel");
}

fn reference() {
    let s1 = S(1);

    {
        let r_s1 = &s1;
        println!("r_s1 {:?}", r_s1);
        println!("*r_s1 {:?}", *r_s1);
    }
    println!("s1 still alive {:?}", s1);
}

fn in_block_assignment() {
    let ref_to_n;
    {
        let n = 12; // n değişkeni inner scope'da oluşturuluyor.
        ref_to_n = &n; // `ref_to_n`'e `n`'in referansı atanıyor. Ownerlık hala `n`'de
        println!("ref_to_n, {}", ref_to_n);
    } // ~~ Bu bloktan çıktıktan sonra `n`'in ömrü tükeniyor böylece içerisindeki data da
      // ~~ silinecek. Çünkü n owner olan dataydı.
      // ~~ `ref_to_n`'in kendisi bu scope'u da kapsayan bir scope olduğu için o yaşamaya
      // ~~ devam edecek. Fakat içerisinde `n`'e referans vardı ve `n`'in oluşturulduğu
      // ~~ scopetan çıkıldığı için bu alan drop edilecek. Bu durumda `ref_to_n` geçersiz
      // ~~ bir alanı refere edecek.
      // println!("ref_to_n, {}", ref_to_n); // ~~ `n` does not live long enough
}

fn borrow_from_inside_a_vector() {
    let mut v = vec![12];
    let ref_to_first = &v[0];
    // v.push(13); // ~~ cannot borrow `v` as mutable because it is also borrowed as immutable
    // ~~ Buradaki sorun şu: `ref_to_first`, o anda vektörün ilk elemanın adres bilgisini
    // ~~ tutuyor. `v.push()` metodunun çağrılması ise vektörün heap'de yeniden
    // ~~ konumlandırılmasına sebep olabilir. Vektöre yeni bir eleman eklenmeye
    // ~~ çalışıldığında eğer vekötürn eleman sayıssı kapasitesini aşacaksa, heap'te
    // ~~ mevcut kapasitesinin iki katı büyüklüğünde yeni bir alan tutularak, mevcut
    // ~~ elemanlar aynı sırayla buraya kopyalanır. Daha sonra yeni eleam bu yeni alanın
    // ~~ içine eklenir. Eski alan ise artık geçersizdir. `ref_to_first` ise bu durumdan
    // ~~ habersiz bir şekilde eski alanı göstermeye devam etmekte.
    println!("{}", ref_to_first);
}

struct X(char);

impl Drop for X {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

fn orders_of_drop() {
    let _a = X('a');
    let _b;
    let _c = X('c');
    _b = X('b');
}

fn referencinf_wrong_order() {
    let n = 12;
    let mut _r;
    let m = 13;
    _r = &m;
    _r = &n;
}

fn some_borrowing_checks() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];

    // direct access
    // for i in 0..v.len() {
    //     println!("e: {}", v[i]);
    // }

    // mutable borrowing
    // for e in &mut v {
    //     *e = *e * 2;
    //     println!("e: {}", e);
    // }

    // accessing by iterator
    // v.iter().for_each(|e| println!("e: {}", e));

    // accessing by iterator
    // let m = v.iter().find(|e| *e % 3 == 0).unwrap_or(&0);
    // println!("m3: {}", m);
    let mut iter = v.iter();
    for i in (2..5).rev() {
        println!("looking for {}", i);
        println!(
            "mod {} {:?}\n",
            i,
            iter.find(|e| {
                println!("    current e {}", e);
                *e % i == 0
            })
        );
    }

    let m = &v.into_iter().find(|e| e % 50 == 0).unwrap_or(0);
    println!("m50: {}", m);

    // println!("v: {:?}", v);
    //    v.iter().for_each(|e| println!("e: {}", e));
}

fn mut_vec_elem(e: &mut i32) {
    *e = 20
}

fn some_borrowings() {
    let a = 12;
    let mut b = 13;
    print!("{} ", a);
    {
        let c = &a;
        let d = &mut b;
        print!("{} {} ", c, d);
    }
    b += 1;
    print!("{}", b);
}

fn some_valid_borrowings() {
    {
        println!("multiple *immutable* borrowings of immutable variable which both live until the end of the block:");
        let a = 12;
        let _b = &a;
        let _c = &a;

        println!("a: {}, _b: {}, _c: {}", a, _b, _c);
    }

    {
        println!("multiple *immutable* borrowings of mutable variable which both live until the end of the block:");
        let mut a = 12;
        let _b = &a;
        let _c = &a;

        println!("a: {}, _b: {}, _c: {}", a, _b, _c);
    }
    
    {
        println!("multiple *immutable* borrowings of mutable variable");
        let mut a = 12;
        let _b = &a;

        println!("a: {}, _b: {}", a, _b);
    }
}
