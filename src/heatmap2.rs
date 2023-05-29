/*
 Heatmap data structure as suggested by my advisor
 */

#[derive(Debug)]
struct Heatmap {
    data: Vec<String>,          // Vector that represents the variables acessed by a requisition
    timeblock_size: usize,        // block of reqs that make a single timespan, number of reqs inside such block
    frequency: Vec<Vec<u64>>    // Matrix responsible for storing the frequencies of data in a timeblock
}

impl Heatmap {
    fn new(timeblock_size: usize) -> Heatmap {
        Heatmap {
            data: Vec::new(),
            timeblock_size,
            frequency: Vec::new()
        }
    }

    fn get_data(&self) -> &Vec<String> {
        &self.data
    }

    fn get_mut_data(&mut self) -> &mut Vec<String> {
        &mut self.data
    }

    fn add_data(&mut self, data: String) -> () {
        let mut_data = self.get_mut_data();
        mut_data.push(data);
        let temp_vec:Vec<u64> = Vec::new();
        self.frequency.push(temp_vec)
    }

    fn get_timeblock_size(&self) -> &usize {
        &self.timeblock_size
    }

    fn get_frequency_table(&self) -> &Vec<Vec<u64>> {
        &self.frequency
    }

    fn get_frequency_at(&self, i:usize, j:usize) -> &u64 {
        &self.frequency[i][j]
    }
}

fn main() {
    let mut heatmap = Heatmap::new(4);
    heatmap.add_data("data".to_string());
    heatmap.add_data("data2".to_string());
    println!("{:?}", heatmap)
}