mod parser;
use std::collections::VecDeque;

fn print_fila(fila: VecDeque<&str>) -> () {

}

fn main() {
    let mut fila: VecDeque<&str> = VecDeque::new();
    //let ref_fila = &fila;

    match parser::parse("./redis_get_set.log") {
        Err(_) => panic!("abort"),
        Ok(_) => println!("OK")
    };
    parser::parse_to_queue("./redis_get_set.log", &mut fila);
    println!("OK")
    //parser::parse("redis_get_set.log")
}
