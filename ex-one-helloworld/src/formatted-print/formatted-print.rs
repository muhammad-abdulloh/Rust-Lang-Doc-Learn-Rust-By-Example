
// Formatlash uslubi C# bilan bir xil ekan 
// impl f64;

// #![allow(dead_code)]
// #![allow(unused_variables)]

fn main(){
/*
    let x = 5 + 6;
    let y = 8 + 8;

    println!("X = {1}, Y = {0}", x, y);
    print!("X = {}, Y = {}", x, y);
*/

/* 
    //formatlashning raqamli uslubi
    // {} => manashu qavsni qo'yib ichiga shunchaki raqam
    // yoki bo'sh qoldirilsa ham error beradurg'o'y
    println!("0, {0}, 2, {1}, 4, {1}, 5, {0}, {0}", "Salom", 456);
*/

/* 
    // Argumentlarni nomlash mumkin ekan 
    println!("{subject}, {verb}, {object}",
            object = "Bu object",
            subject = "Bu subject",
            verb = "BU verb");
*/

/*
    // sonlarni spesific tipga o'tkazish      
    println!("Base 10:                 {}", 69420);   // 69420
    println!("Base 2 (binary):         {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):          {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal):   {:x}", 69420); // 10f2c
*/

/*
    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    // docs da shunday deb yozilgan lekin nima uchun
    // numberga nechchi bersak shu ekranga chiqatotganini tushunmayabman
    // tushunarli bu degani umumiy 5 ta probel space bo'sh joy tashlaydi va anashu 
    // ajratilgan bo'sh joyga berilgan numberni tiqadi.
    // misol uchun shunda 5 ta katak berilgan 
    // va u katakning ohiridan 1 tasiga 1(bir) soni joylashtirilgan va 
    // undan oldin 4 ta probel bo'sh katakcha ajratilgan haa okaayyy davaytee bolaaa
    println!("{number:>5}", number=1);  // 1

    // bunda esa number uchun 8 ta katak, bo'sh joy, probel ajratilgan
    // uning 4 tasiga bergan 1569 raqamimiz egalik qilyabdi va undan oldin
    // 4 ta bo'sh katak qoldi ya'ni probel tashlandi 
    println!("{number:>8}", number=1569);  // ____1569

    // bu holatda dastlab raqam yozilib qolgan joy o'ng tomonidan ajratilyabdi  
    println!("{number:<9}", number=12); // 12_______
*/

/*
    // biz bo'sh joylarni 0 bilan to'ldirishimiz mumkin ekan
    println!("{number:0>5}", number=1); // 00001

    // o'ng tomonnini ham biz ixtiyoriy narsa bilan to'ldirishimiz mumkin ekan
    // hoh u raqam bo'lsin hoh string
    println!("{number:0<5}", number=2); // 20000

    // buyerga hohlagan belgini qo'yish mumkin \ shundan tashqari
    // va bu belgilar 1 tadan ko'p bo'masligi kerak

    println!("{number:(<6}", number=4); // 4(((((

    // chap tarafdagi bo'sh joylarni 0 bilan to'ldiryabmiz
    // with ni beradigan joydagi o'zgaruvchiga $ belgisi biriktirilishi shart 
    // bo'masam error beradi
    println!("{number:0>with$}", number=1, with=5); // 00001

    // Rust hatto argumentlarning to'g'ri soniga ishonch hosil qilish uchun tekshiradi ekan qoyileee
    // hsuyerda 1 ni qo'yganimiz uchun error beradi chunki u argument yo'q mashnaqa :) -_- :)
    println!("My name is {0}, {1}, {0}", "Bond");
*/

// #[allow(dead_code)]
// struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

// use std::fmt::Display;
// use std::fmt::f64;

// struct Structure(usize);


// use std::io;
// struct Structure(i32);
// struct Structure(f64);
    // #![allow(dead_code)]
    /// ishlavotiiiiiiiiiiiiiiiii
    let number: f64 = 1.0;
    let with: usize = 5;
    println!("{number:>with$}");
}