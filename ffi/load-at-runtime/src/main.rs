#![allow(dead_code)]
#![allow(unused_imports)]
mod lib;
use std::ptr::null;
use std::mem::transmute;
use crate::lib::{GetProcAddress, LoadLibraryA, MessageBoxA};

fn main() {
    // `LoadLibraryA` windows api'sine ait bir fonksiyon olduğu
    // için C'nin tpileri ve kuralları geçerli. C'de ise Rust'daki
    // gibi bir `string` tipi yok. Onun yerine  stringin başlangıç
    // adresini tutan `char *` -char pinter- kullanılmakta ve
    // o adresten itibaren `'\0'` karakterine rastlanana kadarki
    // tüm karakterler -`'\0'` karakteri dahil- string olarak ele
    // alınır. Bu yüzden `LoadLibraryA` fonskiyonuna doğrudan string
    // yerine onun *pointer*'ı aktarılmakta.
    //
    // Ayrıca fonksiyonun çalışması C'ye göre olacağı için eğer
    // fonksiyon sonunda `'\0'` -null terminating- olamdan, yani
    // `LoadLibraryA("USER32.dll".as_ptr())`
    // şeklinde çağrılsaydı, bellekte `"USER32.dll` stringinin
    // bittiği bölgeden itibaren `'\0'`'e rastlayana kadar ki
    // diğer bellek alanlarının içeriği de char olarak değerlendirilip
    // strign'e eklenmeye devam edecekti. Bu durumda ise dll'in adı
    // bambaşka bir şey olarak fonksiyon tarafında alınaca ve dll
    // bulunamayacaktı.
    unsafe {
        // `LoadLibraryA` belirtilen modülü belleğe yükleyecek ve
        // bu modülün bellekteki başlangıç adresini dönecektir. 
        let h = LoadLibraryA("USER32.dll\0".as_ptr());

        // `GetProcAddress` ise bu modül içerisinde `MessageBoxA`
        // isimli bir fonksiyon arayacak ve eğer bulursa bu fonskiyonun
        // bellekteki adresini dönecektir.
        let f = GetProcAddress(h, "MessageBoxA\0".as_ptr());
        
        println!("Hello, world!");
        println!("h is {:?}", h);
        println!("f is {:?}", f);

        // `f` çalıştırmak istediğimiz fonksiyonun adresini tutmakta, 
        // fakat tek başına bu işlemimizi yapmaya yeterli değil. Bir 
        // *prototype*'ın bu bu adrese bind edilmesi gerekmekte. 
        // Prototipin tnaımı lib'de. 
        //
        // Burada `transmute` fonksiyonu ile `f`'içerisindeki değer
        // bit bit ve tamamen aynı sıra ile belirtilen değişekene
        // -ki burada `mb` olmakta- atar. `transmute` bu işlemi yaparken
        // herhangi bir tip kontrolü yapmaz. Yalnızca *kaynak ve hedef*
        // bellek alanlarının boyutlarının eşit olması gerekmekte. Bunun
        // dışında h,ç b,r kontrol yapılmadığı için kullanımında çok
        // dikkatli olunmalı. Aksi taktirde hem istenmeyen bir veri kopyalamıs
        // ile güvenlik açığına neden olabilir, hem de eğer kaynaktaki bit 
        // içeriği hedef tipin kabul etmediği bir değer olursa programın
        // bozulmasına neden olabilir.
        let mb: MessageBoxA = transmute(f);

        // Artık `mb` ilgili fonksiyona erişim sağlayabilir. Bu fonksiyon
        // yine C kurallarına göre çağrılıp kullanılabilir.
        mb(null(), "Hello from Rust\0".as_ptr(), "Loll\0".as_ptr(), 0);
    }
}