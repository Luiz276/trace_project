use std::collections::VecDeque;

use crate::heatmap::{heatmap2::{ImplementationSpecificOperations, Heatmap}, parser};

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

    fn create_heatmaps(&self, lista_heatmaps: Vec<Heatmap>, fila_linhas_log: VecDeque<Vec<String>>) -> () {
        let mut vec_commands: Vec<String> = Vec()::new();

        while !fila_linhas_log.is_empty() {
            let linha = fila_linhas_log.pop_front();
            let _timestamp = linha[0].clone();
            let command:String = linha[3].clone();
            if !vec_commands.contains(&command) {
                vec_commands.push(command);
                lista_heatmaps.push(new::Heatmap(command, 4))
            }
            let target_htmp;
            for htmp in lista_heatmaps {
                if htmp.get_command() == command {
                    target_htmp = htmp;
                }
            }
            htmp.add
        }
    }
}