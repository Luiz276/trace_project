mod parser;
use std::collections::VecDeque;

fn print_fila(fila: &mut VecDeque<std::str::Split<'_, &str>>) -> () {
    while !fila.is_empty() {
        let x = fila.pop_front();
        println!("{:?}", x);
    }
}

fn main() {
    let mut fila: VecDeque<std::str::Split<'_, &str>> = VecDeque::new();
    //let ref_fila = &fila;

    // match parser::parse("./redis_get_set.log") {
    //     Err(_) => panic!("abort"),
    //     Ok(_) => println!("OK")
    // };
    match parser::insert_queue("./redis_get_set.log", &mut fila) {
        Err(_) => panic!("abort"),
        Ok(_) => println!("queue OK")
    }
    print_fila(&mut fila);
    println!("OK")
    //parser::parse("redis_get_set.log")
}
