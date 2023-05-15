mod parser;
mod reqs;
use std::collections::VecDeque;

static FILEPATH: &str = "./redis_get_set.log";

#[allow(dead_code)]
fn print_fila(fila: &mut VecDeque<Vec<String>>) -> () {
    while !fila.is_empty() {
        let x = fila.pop_front();
        println!("{:?}", x);
    }
}

fn main() {
    let mut fila: VecDeque<Vec<String>> = VecDeque::new();
    //let ref_fila = &fila;

    // match parser::parse("./redis_get_set.log") {
    //     Err(_) => panic!("abort"),
    //     Ok(_) => println!("OK")
    // };
    match parser::insert_queue(FILEPATH, &mut fila) {
        Err(_) => panic!("abort"),
        Ok(_) => println!("parse to queue OK")
    }
    //print_fila(&mut fila);

    reqs::send_command(fila);
    println!("program OK")
    //parser::parse("redis_get_set.log")
}
