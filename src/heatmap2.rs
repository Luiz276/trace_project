/*
 Heatmap data structure as suggested by my advisor
 */

struct Heatmap {
    data: Vec<String>,
    timeblock_size: u64, // block of reqs that make a single timespan, number of reqs inside such block
    frequency: Vec<Vec<u64>>    // Matrix responsible for storing the frequencies of data in a timeblock
}

impl Heatmap {
    fn new(timeblock_size: u64) -> Heatmap {
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

    fn get_timeblock_size(&self) -> &u64 {
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
    let heatmap = Heatmap::new();
    println!("{:?}", heatmap)
}