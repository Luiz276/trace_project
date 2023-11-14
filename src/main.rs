mod heatmap;
mod generator;

use std::collections::VecDeque;
use heatmap::heatmap2::Heatmap;
use crate::heatmap::parser;
use crate::generator::redis::implementation_specific_operations;

static FILEPATH: &str = "./redis_test.log";

#[allow(dead_code)]
fn print_fila_commands(fila: &mut VecDeque<Vec<String>>) -> () {
    while !fila.is_empty() {
        let x = fila.pop_front();
        println!("{:?}", x);
    }
}

#[allow(dead_code)]
fn print_fila_tempos(fila: &mut VecDeque<f64>) -> () {
    while !fila.is_empty() {
        let x = fila.pop_front();
        println!("{:?}", x);
    }
}

#[allow(dead_code)]
fn print_heatmap(heatmap: &Heatmap) -> () {
    for i in heatmap.get_frequency_table() {
        for j in i {
            print!("{:?}", j)
        }
        print!("\n")
    }
}

fn main() {
    let mut fila_linhas_log: VecDeque<Vec<String>> = VecDeque::new();
    let mut fila_tempos: VecDeque<f64> = VecDeque::new();

    match parser::insert_queue(FILEPATH, &mut fila_linhas_log, &mut fila_tempos) {
        Err(_) => panic!("abort"),
        Ok(_) => println!("parse to queue OK")
    }

    let mut lista_heatmaps: Vec<Heatmap> = Vec::new();
    lista_heatmaps = implementation_specific_operations::create_heatmaps(lista_heatmaps, fila_linhas_log.clone());

    print_fila_commands(&mut fila_linhas_log);
    print_fila_tempos(&mut fila_tempos);
    
    println!("program OK")
}
