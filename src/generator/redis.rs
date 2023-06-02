use std::collections::VecDeque;

use crate::{heatmap::{heatmap2::{ImplementationSpecificOperations, Heatmap}, parser}};

// mod heatmap2;
// pub mod parser;

impl dyn ImplementationSpecificOperations {
    // fn parse_into(&self, filepath: &str) -> () {
    //     /* TODO */
    //     let mut fila: VecDeque<Vec<String>> = VecDeque::new();

    //     match parser::insert_queue(filepath, &mut fila) {
    //         Err(_) => panic!("abort"),
    //         Ok(_) => println!("parse to queue OK")
    //     }

    // }

    fn create_heatmaps(lista_heatmaps: Vec<Heatmap>, fila_linhas_log: VecDeque<Vec<String>>) -> () {

    }
}