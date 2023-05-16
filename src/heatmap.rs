use std::collections::Vec;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub struct heatmap {
    /*
    need a matrix capable of storing frequency of events: matrix[type of event][time block] = frequency
    */
    frequency: Vec<Vec<String>>,
    time_block: usize,
}

trait heatmap_traits {
    fn create_heatmap(&mut self, log_filepath: &str) -> &'static heatmap;

    
}

impl heatmap {
    fn create_heatmap(&mut self, log_filepath: &str) -> heatmap{
        let file = File::open(filepath)?;
        let f = BufReader::new(file);
        self.frequency = Vec::new();
        self.frequency
    }
}
