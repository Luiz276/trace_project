mod parser;
mod reqs;
mod heatmap;
use std::collections::VecDeque;
use heatmap::{Heatmap, HeatmapGeneric};

static FILEPATH: &str = "./redis_get_set.log";

#[allow(dead_code)]
fn print_fila(fila: &mut VecDeque<Vec<String>>) -> () {
    while !fila.is_empty() {
        let x = fila.pop_front();
        println!("{:?}", x);
    }
}

#[allow(dead_code)]
fn print_heatmap(heatmap: &Heatmap) -> () {
    for i in 0..<Heatmap as HeatmapGeneric>::get_commands_len(heatmap) {
        for j in 0..<Heatmap as HeatmapGeneric>::get_time_len(heatmap) {
            println!("{:?}", <Heatmap as HeatmapGeneric>::get_frequency_at(heatmap, i, j))
        }
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

    //reqs::send_command(fila);
    let heatmap = <Heatmap as HeatmapGeneric>::create_heatmap(FILEPATH);
    print_heatmap(&heatmap);
    println!("program OK")
    //parser::parse("redis_get_set.log")
}
