# OS110_RustMediumExercism
**Nama: Razka Agniatara**

**Username Exercism: Razka173**

**List Problem yang sudah saya selesaikan:**
+ Hamming
+ Pythagorean Triplet
+ Triangle
+ Isogram
+ Perfect Numbers

Esai ini dibuat untuk memenuhi tugas mata kuliah Sistem Operasi semester 110 Program Studi Ilmu Komputer Fakultas MIPA Universitas Negeri Jakarta yang diampu oleh Pak Eka Suryana.

Tujuan dari esai ini adalah untuk menjelaskan salah satu dari problem-problem yang sudah saya selesaikan pada situs <https://exercism.io>.

Pada kesempatan kali ini, saya memilih problem "Isogram" untuk saya bahas dikarenakan pada problem ini, saya mendapatkan banyak pengalaman ngoding setelah mengerjakan problem tersebut.

Berikut adalah deskripsi dari permasalahan "Isogram" dalam situs <https://exercism.io> :

---

# Isogram
Determine if a word or phrase is an isogram.

An isogram (also known as a "nonpattern word") is a word or phrase without a repeating letter, however spaces and hyphens are allowed to appear multiple times.

Examples of isograms:

    lumberjacks
    background
    downstream
    six-year-old

The word isograms, however, is not an isogram, because the s repeats.

---

### Cara saya dalam menyelesaikan masalah tersebut
Pertama, cara saya menyelesaikan masalah tersebut adalah memahami betul-betul masalah yang akan saya selesaikan. Saya membaca berulang-ulang deskripsi dari permasalahan tersebut sampai saya mengerti apa yang diinginkan oleh permasalahan tersebut.

Kedua, saya menganalisis masalah tersebut dengan cara saya berasumsi serta memikirkan berbagai kemungkinan-kemungkinan cara yang dapat saya dilakukan untuk menyelesaikan permasalah tersebut.

Ketiga, setelah saya menemukan kemungkinan-kemungkinan tersebut, lalu saya memilih satu kemungkinan yang menurut saya paling mudah bagi saya untuk mengimplementasikannya pada permasalahan tersebut.

Keempat, Happy Coding.

### Pengalaman yang saya dapatkan selama mengerjakan permasalahan tersebut
Banyak pengalaman yang saya dapatkan selama saya mengerjakan permasalahan ini, terutama dalam membaca pesan error yang disebabkan oleh kesalahan syntax maupun kesalahan logika.

Lalu, metode atau cara saya dalam menyelesaikan permasalahan ini yang menurut saya solusi saya cukup mudah untuk dimengerti. Inti dari permasalahan ini adalah kita harus bisa menentukan apakah sebuah kata atau kalimat yang di input termasuk _Isogram_ atau bukan.

Awalnya saya berpikir untuk menggunakan cara iterasi dalam menyelesaikan masalah ini. Yaitu dengan melakukan iterasi huruf pada kata yang di input. Namun cara ini menurut saya akan merepotkan. Lalu saya mendapatkan ide lain untuk menyelesaikan permasalahan ini. Yaitu dengan cara membuat duplikat dari input yang berupa _&str_ menjadi dua variabel baru yang serupa, lalu mengubah kedua duplikat tersebut menjadi huruf kecil semua. Selanjutnya saya menghilangkan karakter spasi dan strip dari kedua duplikat tersebut. Kemudian satu duplikat saya iterasi lalu saya masukkan ke dalam HashSet. Alasan kenapa saya menggunakan HashSet ialah karena dengan menggunakan HashSet, maka input yang saya iterasi tersebut akan menghilangkan duplikat huruf yang dimilikinya. Kemudian di langkah yang terakhir, saya membandingkan panjang variabel duplikat yang satunya dengan panjang HashSet tersebut. Jika panjangnya sama, maka input tersebut merupakan isogram.

Tentu cara atau metode tersebut merupakan pengalaman berharga yang tidak terpikirkan sebelumnya. Dan cara ini juga mudah dimengerti oleh orang yang baru membaca hasil kode saya dengan sedikit penjelasan.

### Izinkan saya mempersingkat apa yang saya katakan...
+ Saya menggunakan HashSet dalam menyelesaikan masalah ini.
+ Saya membuat dua duplikat dari masukan (input) yang diberikan, lalu yang duplikat satu saya proses ke dalam HashSet, kemudian duplikat kedua saya biarkan seperti aslinya.
+ Saya membandingkan panjang antara HashSet yang mana sudah berisikan data dari duplikat satu dengan panjang duplikat kedua.
+ Jika panjang keduanya sama, maka fungsi akan mengembalikan _true_ dan itu artinya masukan (input) ialah isogram.
+ jika panjang tidak sama maka fungsi akan mengembalikan _false_ dan itu artinya masukan (input) bukan merupakan isogram.

### Source Code

```rust
use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    //unimplemented!("Is {} an isogram?", candidate);
    if candidate.len() == 0 {
        return true;
    }
    let mut word1 = candidate.to_lowercase();
    let mut word2 = candidate.to_lowercase();

    word1 = word1.replace(" ", "");
    word1 = word1.replace("-", "");
    word2 = word2.replace(" ", "");
    word2 = word2.replace("-", "");

    let mut set = HashSet::new();
    for i in word1.chars() {
        set.insert(i);
    }

    if set.len() == word2.len() {
        return true;
    } else {
        return false;
    }
}
```

### Link-link yang membantu saya dalam menyelesaikan permasalahan ini :
+ <https://doc.rust-lang.org/std/collections/struct.HashSet.html>
+ <https://doc.rust-lang.org/1.0.0/book/for-loops.html>
