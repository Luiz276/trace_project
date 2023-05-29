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
    frequency: Vec<Vec<u64>>,

    #[allow(dead_code)] // Temporário, só para testes iniciais
    time_block: f64,    // timespan

    time: Vec<u32>,
    commands: Vec<String>,
}

pub trait HeatmapGeneric {
    fn create_heatmap(log_filepath: &'static str) -> Heatmap;
    fn get_command_index(command:&String, command_list: &Vec<String>) -> usize;
    fn get_commands_len(heatmap: &Heatmap) -> usize;
    fn get_time_len(heatmap: &Heatmap) -> usize;
    fn get_frequency_at(heatmap: &Heatmap, i: usize, j: usize) -> u64;
}

impl<'a> HeatmapGeneric for Heatmap{
    fn create_heatmap<'b>(log_filepath: &'static str) -> Heatmap {
        let mut fila: VecDeque<Vec<String>> = VecDeque::new();
        let mut frequency = Vec::new();
        let time:Vec<u32> = Vec::new();
        let mut command_list = Vec::new();
        let time_block = 0.01;

        match parser::insert_queue(log_filepath, &mut fila) {
            Err(_) => panic!("abort"),
            Ok(_) => println!("parse to queue OK")
        }

        for i in fila {
            let _timestamp = i[0].clone();
            let command:String = i[3].clone();
            let freq_index = Self::get_command_index(&command, &command_list);
            if !command_list.contains(&command) {
                command_list.push(command);
                frequency.push(Vec::new());
                frequency[freq_index][0] = 0;
            }

            frequency[freq_index][0] += 1
        }
        return Heatmap {
            frequency,
            time_block,
            time,
            commands: command_list
        }

    }

    fn get_command_index(command: &String, command_list: &Vec<String>) -> usize {
        let i:usize = 0;
        while i < command_list.len() {
            if &command_list[i]==command {
                break
            }
        }
        return i
    }

    fn get_commands_len(heatmap: &Heatmap) -> usize {
        return heatmap.commands.len()
    }

    fn get_time_len(heatmap: &Heatmap) -> usize {
        return heatmap.time.len()
    }

    fn get_frequency_at(heatmap: &Heatmap, i: usize, j: usize) -> u64 {
        return heatmap.frequency[i][j]
    }
}

// fn main() {
//     Heatmap::create_heatmap("./redis_get_set.log");
//     println!("OK");
// }