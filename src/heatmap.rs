use std::collections::Vec;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub struct heatmap {
    /*
    need a matrix capable of storing frequency of events: matrix[event_type][time_block] = frequency
    */

    /*
    matrix[i][j] that stores the heatmap values,
    where i=time and j=commands
    */
    frequency: Vec<Vec<String>>,

    time_block: usize,              // timespan

    time: Vec<u32>,
    commands: Vec<&str>,
}

trait heatmap {
    fn create_heatmap(&mut self, log_filepath: &str) -> &'static heatmap;
}

impl heatmap {
    fn create_heatmap(&mut self, log_filepath: &str) -> &'static heatmap {
        let file = File::open(filepath)?;
        let f = BufReader::new(file);
        self.frequency = Vec::new();
    }
}
