/*!  
  # Lifetimes

  *Lifetime*, bir değerin, bir bellek adresinde kaldığı süreye deniyor. *Concrete Lifetime* ise bir değerin belirli bir bellek alanında oluştuğu ilk andan, o bellek alanından kaldırıldığı son ana kadar geçen süreye denir. Bellek alanının içeride kesintisisiz değer tutyor olması, tam olarak *concrete lifetime* ı tanımlamayabilir. Çünkü ilk başta oraya atanan değer drop ediltikten sonra o bellek alanı, o değere tahsis edilmiş değildir. Bundan sonrasında bellek alanı başka bir değeri tutmaya baladığında yeni bir değerin *lifetime*'ından bahsedilir. Özetle;

  Bir *value*'nun *lifetime*'ının:
  * **Bşlangıcı** bellekte kendisi için bir ayrılan yere yerleştiğiandır. Bu bir değişkene literaller ile atama, başka bir değişkenin değerini atama, bir fonksiyon çağrıldığında, onun parametresine aktarama vb. olabilir.
  * **Bitişi** ise bellekteki o alanın o değer için tahsisi sona erdiğindedir. *Local* ve *primitive* değerler için bu süre bir fonksiyon içerisinde tanımlandıkları andan, fonksiyondan çıkılan  -*return*- ana kadardır. Rust, bir fonksiyondan çıkıldığında o fonksiyon için tanımlanmış bütün local ve primitive valueları siler. Emin olmamakla birlikte manuel olarak `drop()` fonksiyonun bir değişken üzerinde çalıştırılması da lifetime'ın sonunu getirebilir. 

  Refereanslar için de lifetime geçerlidir. Onların lifetime'ı ise
  * Bellekte onlar için bir yer tahsis edildiğinde başlar,
  * Bu alan drop edildiğinde ya da refere ettikleri *heap* alanının *owner*lığı başka bir referansa geçtiğinde yani *move* edildiğinde biter.
  Referanslar için önemli bir şart daha vardır: **bir referansın *lifetime*'ı, refere ettiği değerden daha uzun olamaz!** Böylece bir referansın geçersiz bir bellek alanını refere etmesi engellenerek bu sebepten oluşabilecek hataların önüne geçilmiş olur.
*/
/*!
 ## Örnek 1: Aynı scope'a sahip değer ve referansları
 ```
pub fn simple_reference() {
    /// List değeri oluşturuluyor. `list`'in lifetime'ı burada
    /// başlıyor
    let list = vec![1,2,3];       
    
    /// Bu listenin ilk iki elemanını refere eden bir referans
    /// oluşturuluyor. `first_two`'nun lifetime'ı da burada
    /// başlıyor. `list` hala hayatta.  
    let first_two = &list[0..2];

    println!("first two are {:?}", first_two);
    println!("list is {:?}", list);
    println!("first two are {:?}", first_two);
    
    /// Bu noktada hem `list`'in hem de `first_two`'nun yaşam
    /// ömürleri burada bitiyor. Yani `first_two`'nun yaşam zamanı
    /// refere ettiği `list`'in yaşam zamanı içierisinde kalıyor. 
}
```
## Örnek 2: Inner Scope'da oluşturulan referans

 ```
 pub fn reference_in_an_inner_scope() {
    /// `list` burada oluşturuluyor.
    let list = vec![1,2,3];

    {   /// Yeni bir *scope* oluşturuluyor. Bu scope içerisinde
        /// oluşturulan değişkenlerin ömrü bu scope'dan çıkıldığında
        /// bitecek.
        
        /// `first_two` burada oluşturuluyor ve yaşam zamanı burada
        /// başlıyor. 
        let first_two = &list[0..2];

        println!("list is {:?}", list);
        
        /// `list`'in scope'u bu scope'u da kapsadığı için `list`
        /// hala hayatta, dolayısıyla onu refere eden `first_two` da
        /// burada kullanılabilir durumda.
        println!("first two are {:?}", first_two);
        
    } /// inner scope burada tamamlanıyor, bu yüzden de `first_two`
      /// 'nun ömrü burada bitiyor. Fakat `first_two` bir referans
      /// olduğu için refere ettiği bellek alanı üzerinde bir hükmü
      /// yok.  

    println!("list is stile alive: {:?}", list);
}
 ```

 ## Örnek 3: Referansın bir fonksiyona parametre olarak verilmesi

 ```
 
pub fn pass_reference_to_another_function() {
    /// `list` burada oluşturuluyor. `list` bu fonksiyon'un
    /// çalışması bitene kadar hayatta kalmaya devam edecek. Bu 
    /// fonksiyon çalışmaya başladıktan sonra, içinde başka bir
    /// fonksiyon çağrısı yapıldığında, bu fonksiyon hala çalışıyor
    /// demektir. Zaten öçağrılan fonksiyonun çalışması
    /// tamamlandıktan sonra tekrar bu fonksiyona dönecek.  
    let list = vec![1,2,3,4];
    
    /// `print_first_two` fonksiyonuna `list`'in referansı aktarılıyor.  
    print_first_two(&list);

    println!("list is {:?}", list);
    
    print_first_two(&list);
}

/// Fonksiyon bir `i32 slice`'ının referansını alıyor.    
pub fn print_first_two(borrowed_list: &[i32]) {
    /// Bu referansın ömrü bu fonksiyonun dışında da geçerli olduğu için
    /// fonksiyon içerisinde kullanılabilir. 
    let first_two = &borrowed_list[0..2];

    println!("first two are {:?}", first_two);
} /// Fonksiyon tamamlandığında `first_two`'nun ve dahi `borrowed_list`
  /// 'in ömrü tamamlanacak. Fakat ikisinin de referans olduğu ve refere 
  /// ettikleri değerin `ömrü` üzerinde etkileri olmadıkları unutulmamalı.
  ```
*/

pub fn simple_reference() {
    // List değeri oluşturuluyor. `list`'in lifetime'ı burada
    // başlıyor
    let list = vec![1,2,3];       
    
    // Bu listenin ilk iki elemanını refere eden bir referans
    // oluşturuluyor. `first_two`'nun lifetime'ı da burada
    // başlıyor. `list` hala hayatta.  
    let first_two = &list[0..2];
    
    println!("first two are {:?}", first_two);
    println!("list is {:?}", list);
    println!("first two are {:?}", first_two);
    
    // Bu noktada hem `list`'in hem de `first_two`'nun yaşam
    // ömürleri burada bitiyor. Yani `first_two`'nun yaşam zamanı
    // refere ettiği `list`'in yaşam zamanı içierisinde kalıyor. 
}



pub fn reference_in_an_inner_scope() {
    // `list` burada oluşturuluyor.
    let list = vec![1,2,3];

    {   // Yeni bir *scope* oluşturuluyor. Bu scope içerisinde
        // oluşturulan değişkenlerin ömrü bu scope'dan çıkıldığında
        // bitecek.
        
        // `first_two` burada oluşturuluyor ve yaşam zamanı burada
        // başlıyor. 
        let first_two = &list[0..2];

        println!("list is {:?}", list);
        
        // `list`'in scope'u bu scope'u da kapsadığı için `list`
        // hala hayatta, dolayısıyla onu refere eden `first_two` da
        // burada kullanılabilir durumda.
        println!("first two are {:?}", first_two);

        
    } // inner scope burada tamamlanıyor, bu yüzden de `first_two`
      // 'nun ömrü burada bitiyor. Fakat `first_two` bir referans
      // olduğu için refere ettiği bellek alanı üzerinde bir hükmü
      // yok.  

    println!("list is stile alive: {:?}", list);
}

pub fn pass_reference_to_another_function() {
    // `list` burada oluşturuluyor. `list` bu fonksiyon'un
    // çalışması bitene kadar hayatta kalmaya devam edecek. Bu 
    // fonksiyon çalışmaya başladıktan sonra, içinde başka bir
    // fonksiyon çağrısı yapıldığında, bu fonksiyon hala çalışıyor
    // demektir. Zaten öçağrılan fonksiyonun çalışması
    // tamamlandıktan sonra tekrar bu fonksiyona dönecek.  
    let list = vec![1,2,3,4];
    
    // `print_first_two` fonksiyonuna `list`'in referansı aktarılıyor.  
    print_first_two(&list);

    println!("list is {:?}", list);
    
    print_first_two(&list);
}

// Fonksiyon bir `i32 slice`'ının referansını alıyor.    
pub fn print_first_two(borrowed_list: &[i32]) {
    // Bu referansın ömrü bu fonksiyonun dışında da geçerli olduğu için
    // fonksiyon içerisinde kullanılabilir. 
    let first_two = &borrowed_list[0..2];

    println!("first two are {:?}", first_two);
} // Fonksiyon tamamlandığında `first_two`'nun ve dahi `borrowed_list`
  // 'in ömrü tamamlanacak. Fakat ikisinin de referans olduğu ve refere 
  // ettikleri değerin `ömrü` üzerinde etkileri olmadıkları unutulmamalı.