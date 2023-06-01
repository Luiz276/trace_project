/*
 Heatmap var structure as suggested by my advisor
 */

#[derive(Debug)]
pub struct Heatmap {
    var: Vec<String>,               // Vector that represents the variables acessed by a requisition
    timeblock_number: usize,
    timeblock_size: usize,        // block of reqs that make a single timespan, number of reqs inside such block
    frequency: Vec<Vec<u64>>    // Matrix responsible for storing the frequencies of var in a timeblock
}

#[allow(dead_code)]
impl Heatmap {
    pub fn new(timeblock_size: usize) -> Heatmap {
        Heatmap {
            var: Vec::new(),
            timeblock_number: 0,
            timeblock_size,
            frequency: Vec::new()
        }
    }

    pub fn get_var(&self) -> &Vec<String> {
        &self.var
    }

    fn get_mut_var(&mut self) -> &mut Vec<String> {
        &mut self.var
    }

    fn equalize_lines(&mut self) -> () {
        for i in 0..self.var.len() {
            while self.frequency[i].len() < self.timeblock_number {
                self.frequency[i].push(0)
            }
        }
    }

    pub fn add_var(&mut self, var: String) -> () {
        let mut_var = self.get_mut_var();
        mut_var.push(var);
        let temp_vec:Vec<u64> = Vec::new();
        self.frequency.push(temp_vec);
        self.equalize_lines()
    }

    pub fn get_timeblock_size(&self) -> &usize {
        &self.timeblock_size
    }

    pub fn get_frequency_table(&self) -> &Vec<Vec<u64>> {
        &self.frequency
    }

    pub fn get_frequency_at(&self, i:usize, j:usize) -> &u64 {
        &self.frequency[i][j]
    }

    pub fn add_data(&mut self, i:usize, j:usize, req_number: usize) -> () {
        if req_number%self.timeblock_size == 0 {
            self.timeblock_number += 1;
        }
        self.equalize_lines();
        self.frequency[i][j] += 1
    }
}

trait ImplementationSpecificOperations {
    fn parse_into(&self, filepath: &str) -> ();
}