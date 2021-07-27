
struct A;
struct B;
struct S(A, B);
struct SGen<T>(T);

// Aşağıdaki fonkisyonların hepsi kendilerine verilen paramterlerin 
// `ownership`liğini alıp, hemen biterler. Bu ensada da variabel'ların 
// refere ettiği alanlar silinir. Çünkü owner artık uçmuştur.

// Bu *generic* olmayan bir fonksiyon

// `S` `tuple` tipinde bi struct ve iki `property`si var. 
// Birinin tipi `A struct`'ı, diğerinin ki `B struct`'ı. Artık bu tipin 
// adı `S`. Bu yüzden bir *şeyin* bu tipte olacağını belirteceksek tipin 
// adını çağırmamız yeterli.
fn reg_fn(_s: S) {}

// Bu fonkisyonun parametresinin tipi `SGen<A>` olarak belirtilmiş. `A` 
// *concrete* bir tip olduğundan bu fonksiyon *generic* değildir.
fn gen_spec_t(_s: SGen<A>) {}

// Burada da parametre olarak primitve bir tip verilmiş. `SGen<>` `struct`unun 
// tip belirteci *generic tip parametresi* olmadığı için bu fonksiyon da generic 
// değildir.
fn gen_spec_i32(_s: SGen<i32>) {}

// Burada ise fonksiyonun tip paramteresinde `T` *bağımsız değişkeni* var. 
// Bu durumda `generic<T>` fonksiyonu için *`T` üzerinde generictir* denir.
fn generic<T>(_s: SGen<T>) {}

pub fn use_it() {

    // Burada bir *şey*in tipinin `S` olması gerektiğini belirtmiyoruz! `S` 
    // tipinde yeni bir *şey* oluşturuyoruz. `S` tipinde yen bir şey oluşturuken de
    // beklediği parametrelerin doğru bir şekilde doldurulması gerekmekte.
    reg_fn(S(A, B));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    
    generic::<char>(SGen('a')); // *implicitliy specified generic type parameter*
    generic(SGen('c')); // *explicitliy specified generic type parameter*
    generic(SGen(10));
    generic(SGen("bir"));
}