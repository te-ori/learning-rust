use std::ffi::c_void;

/// `c_void`: C2deki `void *` -void pointer'a karşılık gelen Rust tipi-
/// `HModule`: Windows'ta uygulama ve dll'lere *module* denmekte. `HModule`
/// ise bu modolün `handle`'ı. `Handle` ise genel olarak işletim sistemi
/// bazında, bir öğenin id'si olarak düşünülebilir. Bir tamsayıdır. `HModule`
/// handle'ı bir modülün bellekteki başlangıç adresini refere eder.
pub type HModule = *const c_void;
pub type FarProc = *const c_void;

/// Calling Convention, bir fonksiyonun çağrılması sırasında parametrelerin
/// bu fonksiyona nasıl aktarılacağını ve fonksiyonun döndüüğü sonucun, 
/// fonksiyonun çağarıdığı noktaya nasıl akatarılacağını belirler.
/// (https://en.wikipedia.org/wiki/Calling_convention)
///
/// Callin Convention işlemci mimarisine (x86, arm, vs.), platforma (windows,
/// linux,.. ) hatta compiler'a göre bile değişebilir. Örneğin Windows
/// sistem fonksiyonları çağrılacaksa, `stdcall` convention'ına göre
/// parametreler aktarılmalı.
/// (https://docs.microsoft.com/en-us/cpp/cpp/stdcall?view=msvc-160)
///
/// Rust'da harici bir kütüphanenin -windows sistem çağroları için de olablir,
/// ya da herhangi bir programlama dili ile yazılmış bir dll vb de olabilir-
/// fonksiyonları kullanılacaksa -ister dinamik isterse de statik bind edilsin-
/// bu fonksiyonların prototipleri `extern` bloğu içerisinde deklare edilmeli.
/// Çalışma zamanında bu fonksiyonlarla iletişimin hangi convention ile 
/// sağlanacağı `extern`'den emen sonra `""` işaretleri arasına yazılabilir.
extern "stdcall" {
    pub fn LoadLibraryA(name: *const u8) -> HModule;
    pub fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

pub type MessageBoxA = extern "stdcall" fn(*const c_void, *const u8, *const u8, u32);