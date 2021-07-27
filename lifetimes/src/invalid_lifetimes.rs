/*!
## Örnek 1: Inner scope'tan dışarı taşan referans

```
pub fn returning_reference_from_inner_scope() {
    /// `first_two` değişkenin ömrü içinde bulunduğu fonksiyonun çağrıldığında başlar
    /// ve fonksiyonun çalışması tamamlandığında biter.
    let first_two = { /// Burada yeni bir blok başlıyor. Bu bloğum ömrü fonksiyon
                      /// çalıştıktan sonra başlar, fonksiyonun ömrü tamamlanmadan
                      /// önce biter. Bu blok içinde tanımlanan değişkenlerin ömrü de
                      /// bu blok bittiğinde biter.
        /// `list` değişkeninin ömrü bu blok bitince bitecek.
        let list = vec![1, 2, 3];
        /// `list` değişkeninin ilk iki elemanını refere eden bir referans oluşturularak
        /// bloktan geri dönülmesi sağlanıyor ve ömrü bu bloğun dışında da devam eden
        /// `first_two` değişkenine bu referans atanıyor.
        &list[0..2] // ~~ `list` does not live long enough ~~
    };

    /// `first_two` değişkeni ömrü az önce tükenmiş olan `list` değişkenini refere eden
    /// `&list[0..2]` referansını taşımakta. Haliyle `first_two` değişkeni artık geçerli
    /// olmayan bir bellek alanını refere etmekte. rust bu kodu derlemeyecek.
    println!("First two are {:?}", first_two);
}


pub fn returning_reference_from_inner_scope_solved_1() {
    /// İki değişkende aynı *scope*'a taşınır. Böylece hem `list`'in hem de `&list[0..2]
    /// `'in ömrüleri fonksiyonun ömrüne eşitlenmiş olur. Aslında `list`'in ömrü
    ///  `first_two`'dan birazcık daha uzun, çünkü ondan önce tanımlanıyor.
    let list = vec![1, 2, 3];
    let first_two = &list[0..2];

    println!("First two are {:?}", first_two);
}

pub fn returning_reference_from_inner_scope_solved_2() {
    /// Bu yöntemde ise `list`'in ömrü yine inner scope'un dışına kadar uzatılır.
    /// Böylece inner scope bittiğinde `&list[0..2]`'nin kendisi tamamlanırken aktardığı
    /// referans yaşamaya devam eder.
    let list = vec![1, 2, 3];
    let first_two = {
        &list[0..2]
    };

    println!("First two are {:?}", first_two);
}
```
### Örnek 1 Çözüm yöntemleri
```
pub fn returning_reference_from_inner_scope_solved_1() {
    /// İki değişkende aynı *scope*'a taşınır. Böylece hem `list`'in hem de `&list[0..2]
    /// `'in ömrüleri fonksiyonun ömrüne eşitlenmiş olur. Aslında `list`'in ömrü
    ///  `first_two`'dan birazcık daha uzun, çünkü ondan önce tanımlanıyor.
    let list = vec![1, 2, 3];
    let first_two = &list[0..2];

    println!("First two are {:?}", first_two);
}

pub fn returning_reference_from_inner_scope_solved_2() {
    /// Bu yöntemde ise `list`'in ömrü yine inner scope'un dışına kadar uzatılır.
    /// Böylece inner scope bittiğinde `&list[0..2]`'nin kendisi tamamlanırken aktardığı
    /// referans yaşamaya devam eder.
    let list = vec![1, 2, 3];
    let first_two = { &list[0..2] };

    println!("First two are {:?}", first_two);
}

```
## Örnek 2: Bir fonksiyonun referans dönmesi

```
pub fn return_reference() -> &[i32] { // ~~ missing lifetime specifier
                                      // ~~ expected named lifetime parameter
                                      // ~~ help: this function's return type contains
                                      // ~~ a borrowed value, but there is no value for
                                      // ~~ it to be borrowed from
                                      // ~~ help: consider using the `'static`
                                      // ~~ lifetime: `&'static ~~

    /// `list`'in ömrü bu fonksiyon tamamlanacak.
    let list = vec![100, 34, 72, 55];

    /// Burada ise `list`'in ilk iki elemanını refere eden bir referans fonskiyondan
    /// geri döndürülüyor. Bu fonksiyonun dışında bu referans kullanılmaya çalışılırsa
    /// geçersiz bir alana erişilmeye çalışılacak. rust compiler'ı buna izin vermiyor.
    &list[0..2]
}

pub fn caller_of_return_reference() {
    /// `return_reference()` fonksiyonundan döndürülmeye çalışılan referansın ömrü o
    /// fonksiyon ile sınırlı. `caller_of_reference_return_returner_function()`nun ömrü
    /// kendi içinde çalıştırdıklarını da kapsadığı için, bu durumda `first_two`'ya
    /// kendi ömründen daha kısa ömrülü bir bellek alanının referansı atanmaya
    /// çalışılıyor.
    let first_two = return_reference();
    println!("First two are: {:?}", first_two);
}
```

### Örnek 2 Çözüm yöntemleri
```
/// Böyle bir durumda bellekte oluşturulan değerin referansı yerine kendisini dönmek
/// sorunu çözebilir. Böylece kullanılan bellek alanının yani değerin sorumluluğu
/// fonksiyonun çağırıldığı yere verilir. Yani bir tür `move` gerçekleştirilmiş olur.
pub fn return_reference_resolved() -> Vec<i32> {
    vec![100, 34, 72, 55]
}

pub fn caller_of_reference_return_resolved() {
    /// `list` fonksiyondan dönen değeri sahipleniyor. Böylece bu alanın ömrünün
    /// kontrolüde `list`'e geçmiş oluyor. `list`'ten de başka bir değişkene aktarılmaz
    /// ise `list`'in ömrünün bittiği yerde bu değer drop edilecek.
    let list = return_reference_resolved();
    let first_two = &list[0..2];
    println!("First two are: {:?}", first_two);
}
```
/// ## Örnek 3: Değeri *Move* edilmiş bir değişkenin referansını kullanmak
```
pub fn referencing_a_moved_value() {
    /// `list_a`'nın ömrü, fonksiyon çağrıldığında burada başlıyor ve fonksiyon çağrısı tamamlanana kadar sürüyor.
    /// Aslında burada olan şu:
    /// * Bellekte *heap* olarak yönetilen bölgede `vec![1,2,3,4]` için bir alan tahsis
    /// edilerek, ilgili değerler bu alana kaydediliyor.
    /// * *Stack* olarak yönetilen alanda ise heap'te tahsis edilen alanın adresini,
    /// büyüklüğünü, vb tutan bir değer daha oluşturuluyor.
    /// * Heap'deki alanın ömrünün kontrolü stack'deki '*öğeye*' veriliyor.
    /// Sonuç olarak `list_a` bütün bunları işaret eden değişken olmuş oluyor.
    let list_a = vec![1,2,3,4];

    /// Burada ise heap'te vektörü ve içindekileri tutan alanın sorumluluğu `list_b` ye
    /// aktarılıyor. Rust tabiri ile `move` ediliyor.
    /// Burada ise olanlar şöyle özetlenebilir:
    /// * Stack'te `list_b` için yeni bir alan tahsis ediliyor.
    /// * `list_a` nın stack'te tuttuğu ve heap'teki vektöre ait olan bilgiler
    /// `list_b`'nin stack'teki yeni yerine '*taşınıyor*'. Böylece heap'deki bu alanın
    /// ömrünün sorumluluğuda `list_b`'ye geçmiş oluyor.
    /// * `list_a`'nın içeriği `list_b`'ye '*taşındığı*' için list_a artık hiç bir şeyi
    /// refere etmiyor. Burada `list_a`'nın stack'te kalmaya devam ettiğine dikkat
    /// edilmeli. Ama artık heap'deki data ile ilişkisi kesildi ve hiç bir gerçek
    /// datayı kontrol etmiyor. Fakat, eğer *mutable* tanımlandıysa, ilerde aynı tipten
    /// başka değerleri alacak şekilde içeriği güncellenebilir.
    let list_b = list_a;

    /// `list_a` artık hiç bir şeyi refere etmediği için, ondan bir referans
    /// oluşturmaya çalışmak, bellekte geçersiz, tanımlanmamış bir alana erişmeye
    /// çalışmak olacaktır, ki rust buna izin vermiyor.
    let first_two = &list_a[0..2]; // ~~ borrow of moved value: `list_a`
                                   // ~~ value borrowed here after moverustc(E0382)

    println!("First two are: {:?}", first_two);
}
```

### Örnek 3-1
```
pub fn referencing_a_moved_value_2() {
    let list_a = vec![1,2,3];

    /// Burada `first_two`'ya `list_a`'nın ilk iki eleamına *erişim* hakkı tanınıyor.
    /// Böylece `first_two` `list_a`'dan refere ettiği alnın yönetim haklarını *ödünç*
    /// almış oluyor. İki önemli kısıtlama söz konusu:
    /// * Birincisi o alandaki veriler üzerinde değişiklik yapma
    /// ** Eğer `list_a` *mutable* olarak tanımlanmamış ise `first_two` ödeünç aldığı bu 'value' üzerinde değişiklik yapamaz.
    /// ** `list_a` mutable tanımlanmamışsa, `first_two` mutable tanımlanamaz.
    /// ** Eğer `list_a` mutable tanımlanmış olmasına rağmen `first_two` mutable bir
    /// referans olarak tanımlanmışsa yine bir değişiklik yapamaz.
    /// ** Eğer ikisi de mutable tanımlanmışsa buradaki veriler üzerinde değişiklik
    /// yapabilir.
    /// * İkincisi ise, `first_two`, `list_a`'an yönetimi ödünç aldığı için heap'teki
    /// alanın ömrü üzerinde hiç bir etkiye sahip değildir. Yani heap'teki bu alanı
    /// `drop` ederek o bellek alanının başka hernagi bir şeyin kontrolüne veremez.
    let first_two = &list_a[0..2];

    ///  Burada `first_two`'nun kullanımında bir sorun yok. Bu sefer rust'ın ownership'
    /// lik modelinde başka bir kısıtlamaya takılıyoruz.
    let list_b = list_a; // ~~ cannot move out of `list_a` because it is borrowed
                         // ~~ move out of `list_a` occurs hererustc(E0505)

    println!("first two are: {:?}", first_two);
}
```
### Örnek 3 Çözüm yöntemleri
```

pub fn referencing_a_moved_value_resolved_1() {
    let list_a = vec![1, 2, 3];

    {
        /// Burada yeni bir scope oluşturuluyor.
        /// `first_two` `liat_a`'nın iki elemanını ödünç alıyor. Fakat ömrü bu scope
        /// ile sınırlı olduğu için bu scope'tan çıkldığına bu ödünç alma işlemi de
        /// bitecek. Böylece `list_a` heap'teki alanın paylaşımsız tek sahibi olacak.
        let first_two = &list_a[0..2];
        println!("first two are: {:?}", first_two);
    }

    let list_b = list_a;
}

pub fn referencing_a_moved_value_resolved_2() {
    let list_a = vec![1, 2, 3];
     /// rust'ın güncel sürümlerinde, compiler `first_two`'nun son olarak nerde 
     /// kullanıldığına bakarak *ödünç alma* işleminin nerede bittiğine ve böylece
     /// `list_a`'nın nereden itibaren tek söz sahibi olduğuna karar verebiliyor.
     /// Böylece bir inner scope oluşturmaya da gerek kalmıyor
    let first_two = &list_a[0..2];
    println!("first two are: {:?}", first_two);

    let list_b = list_a; 
    // println!("first two are: {:?}", first_two); // ~~ first_two burada da kullanılsaydı yine hata alınacaktı.

}
```
*/

/// ### Örnek 1 Çözüm yöntemleri
pub fn returning_reference_from_inner_scope_solved_1() {
    /// İki değişkende aynı *scope*'a taşınır. Böylece hem `list`'in hem de `&list[0..2]
    /// `'in ömrüleri fonksiyonun ömrüne eşitlenmiş olur. Aslında `list`'in ömrü
    ///  `first_two`'dan birazcık daha uzun, çünkü ondan önce tanımlanıyor.
    let list = vec![1, 2, 3];
    let first_two = &list[0..2];

    println!("First two are {:?}", first_two);
}

pub fn returning_reference_from_inner_scope_solved_2() {
    /// Bu yöntemde ise `list`'in ömrü yine inner scope'un dışına kadar uzatılır.
    /// Böylece inner scope bittiğinde `&list[0..2]`'nin kendisi tamamlanırken aktardığı
    /// referans yaşamaya devam eder.
    let list = vec![1, 2, 3];
    let first_two = { &list[0..2] };

    println!("First two are {:?}", first_two);
}

/// ### Örnek iki çözüm yöntemi

/// Böyle bir durumda bellekte oluşturulan değerin referansı yerine kendisini dönmek
/// sorunu çözebilir. Böylece kullanılan bellek alanının yani değerin sorumluluğu
/// fonksiyonun çağırıldığı yere verilir. Yani bir tür `move` gerçekleştirilmiş olur.
pub fn return_reference_resolved() -> Vec<i32> {
    vec![100, 34, 72, 55]
}

pub fn caller_of_reference_return_resolved() {
    /// `list` fonksiyondan dönen değeri sahipleniyor. Böylece bu alanın ömrünün
    /// kontrolüde `list`'e geçmiş oluyor. `list`'ten de başka bir değişkene aktarılmaz
    /// ise `list`'in ömrünün bittiği yerde bu değer drop edilecek.
    let list = return_reference_resolved();
    let first_two = &list[0..2];
    println!("First two are: {:?}", first_two);
}

pub fn referencing_a_moved_value_resolved_1() {
    let list_a = vec![1, 2, 3];

    {
        /// Burada yeni bir scope oluşturuluyor.
        /// `first_two` `liat_a`'nın iki elemanını ödünç alıyor. Fakat ömrü bu scope
        /// ile sınırlı olduğu için bu scope'tan çıkldığına bu ödünç alma işlemi de
        /// bitecek. Böylece `list_a` heap'teki alanın paylaşımsız tek sahibi olacak.
        let first_two = &list_a[0..2];
        println!("first two are: {:?}", first_two);
    }

    let list_b = list_a;
}

pub fn referencing_a_moved_value_resolved_2() {
    let list_a = vec![1, 2, 3];
     /// rust'ın güncel sürümlerinde, compiler `first_two`'nun son olarak nerde 
     /// kullanıldığına bakarak *ödünç alma* işleminin nerede bittiğine ve böylece
     /// `list_a`'nın nereden itibaren tek söz sahibi olduğuna karar verebiliyor.
     /// Böylece bir inner scope oluşturmaya da gerek kalmıyor
    let first_two = &list_a[0..2];
    println!("first two are: {:?}", first_two);

    let list_b = list_a; 
    // println!("first two are: {:?}", first_two); // ~~ first_two burada da kullanılsaydı yine hata alınacaktı.

}
