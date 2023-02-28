/**
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main(){
    // Printing with `{:?}` is similar to with `{}`.

    println!("{:?} months in a year.", 12);

    println!("{1:?} {0:?} is the {actor:?} name.", 
    "Slater",
    "Christian",
    actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));
}
*/

/*

#[derive(Debug)]
struct Person<'a> {

    // nimagadur mashi type lar bilan ishlayotganda
    // tagchiziq qo'yib ishlash kerak ekan bo'masam warning berib diqqat qilyabdi
    // nimo'sayam rust rust ekande
    // menga esa warning va errorlar yoqmaydi haha :)
    _name: &'a str,
    _age: u8
}

*/

/*
fn main(){  
    let _name = "Peter";
    let _age = 27;
    let peter = Person { _name, _age };
    // Pretty print :)
    println!("{:?}", peter);  // Person { _name: "Peter", _age: 27}

    println!("{:#?}", peter);  // # qo'yilganda formatlab berar ekan qoyil
    /* output
    Person {
        _name: "Peter",
        _age: 27,
    }
     */ 
}
*/

#[derive(Debug)]
struct Person<'a> {
    _name: &'a str,
    _age: u8
}

fn main(){
    let _name = "Peter";
    let _age = 27;
    let peter = Person {_name, _age}; // Personni ichidage propertylar bilan main dagi Personni ichiga bervoryatgan property ya'ni o'zgaruvchilar ham bir xil bo'lishi kerak ekan

    // Pretty print// go'zal jilovlanishlar ostide formatlab ekranga chiqarish
    println!("{:#?}", peter);
/*
    Person {
        _name: "Peter",
        _age: 27,
    }   
*/
}
