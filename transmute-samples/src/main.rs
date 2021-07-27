use std::mem::transmute;

fn main() {
    let a: [u8;4] = [255,255,0,0];

    unsafe {
        let ia0: i32 = transmute(a);
        let ia1: u32 = transmute(a);
        let ua0: u8 = transmute(a[0]);
        let ua3: u8 = transmute(a[3]);

        println!("a: {:?}", a);
        println!("ia0: {}", ia0);
        println!("ia1: {}", ia1);
        println!("ua0: {}", ua0);
        println!("ua3: {}", ua3);

        let u0: u8 = 255;
        // `i8` tipi (-128) - (+127) arasındaki değerleri 
        // alabilir. `u8` ise (0) - (+255). Normal şartlarda
        // `u0`'ın 255 değeri `i1`'e atanmaya çalışılsaydı
        // hata alınırdır. Fakat `transmute` ile tip kontrolü
        // yapılmadan bit bit kopyalama yapıldığı için, `u0`'ın
        // `11111111` olan içeriği `i1`'e aktarılır, `i8` için
        // ise bu dizilimin karşılığı (-1)'dir.
        let i1: i8 = transmute(u0);

        println!("u0: {}", u0);
        println!("i1: {}", i1);
    }
}
