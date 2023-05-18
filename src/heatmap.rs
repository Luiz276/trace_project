mod parser;

use std::vec::Vec;
use std::collections::VecDeque;

pub struct Heatmap {
    /*
    need a matrix capable of storing frequency of events: matrix[event_type][time_block] = frequency
    */

    /*
    matrix[i][j] that stores the heatmap values,
    where i=commands and j=time
    */
    frequency: Vec<Vec<u32>>,

    time_block: f64,              // timespan

    time: Vec<u32>,
    commands: Vec<&'static str>,
}

trait HeatmapGeneric {
    fn create_heatmap(&mut heatmap, log_filepath: &str) -> &'static Heatmap;
}

impl dyn HeatmapGeneric {
    fn create_heatmap(&self, heatmap: Heatmap, log_filepath: &str) -> &'static Heatmap {
        let mut fila: VecDeque<Vec<String>> = VecDeque::new();
        heatmap.frequency = Vec::new();
        heatmap.time = Vec::new();
        heatmap.commands = Vec::new();
        heatmap.time_block = 0.01;

        match parser::insert_queue(log_filepath, &mut fila) {
            Err(_) => panic!("abort"),
            Ok(_) => println!("parse to queue OK")
        }

        for i in vec {
            let timestamp = &i[0];
            let command = &i[3];

            if !heatmap.commands.contains(command.as_str()) {
                heatmap.commands.append(commands.as_str);
                heatmap.frequency.append(Vec::new());
                heatmap.frequency[-1].append(0)
            }

            heatmap.frequency[heatmap.commands.index(command.as_str)][0] += 1
        }

    }
}

fn main() {
    let heat_map = heatmap::create_heatmap("./redis_get_set.log");
    println!("OK");
}