
// // with fmt::Debug
// struct UnPrintable(i32);

// #[derive(Debug)]
// struct DebugPrintable(i32);

fn main(){
    // std::process::Command::new("clear").status().unwrap();

    print!("\x1B[2J\x1B[1;1H");
    // Print with `{:?}` is similar to with `{}`.
    println!("{:?} month in a year.", 12);
    print!("{esc}clear", esc = 27 as char);

    // assert!(Command::new("cls").status().or_else(|_| Command::new("clear")).unwrap().success());

    // clearscreen::clear().expect("failed to clear screen");
    // termion::clear

}
