mod sorts;
use::text_io::scan;

fn main() {
    println!("Choose a program from 'clui' to run");
    println!("1. Sorting algorithms visualization");
    println!("2. Physics engine");
    println!("3. Something else in command line graphic ui");
    println!("0. Exit");
    let choice: i32;
    scan!("{}",choice);
    match choice {
        1 => sorts::start(),
        2 | 3 => not_implemented_msg(),
        0 => std::process::exit(0x0100),
        _ => main()   
    }
    sorts::start();
}

fn not_implemented_msg() {
    println!("not implemented yet sorry :(");
    main();
}