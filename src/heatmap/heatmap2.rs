/*
 Heatmap var structure as suggested by my advisor
 */

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Heatmap {
    command: String,            // Comando ao qual o heatmap corresponde
    var: Vec<String>,           // Vector that represents the variables acessed by a requisition
    timeblock_number: usize,    // Número de timeblocks presentes no Heatmap
    timeblock_size: usize,      // block of reqs that make a single timespan, number of reqs inside such block
    frequency: Vec<Vec<u64>>    // Matrix responsible for storing the frequencies of var in a timeblock
}

#[allow(dead_code)]
impl Heatmap {
    pub fn new(command: String, timeblock_size: usize) -> Heatmap {
        Heatmap {
            command,
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

    fn add_var(&mut self, var: String) -> () {
        if !self.var.contains(&var) {
            let mut_var = self.get_mut_var();
            mut_var.push(var);
            let temp_vec:Vec<u64> = Vec::new();
            self.frequency.push(temp_vec);
            self.equalize_lines()
        }
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

    pub fn add_data(&mut self, var:String, req_number: usize) -> () {
        self.add_var(var.clone());
        if req_number%self.timeblock_size == 0 {
            self.timeblock_number += 1;
        }
        self.equalize_lines();
        let temp_col = (req_number/self.timeblock_size);
        let temp_line = self.var.iter().position(|x| x == &var).unwrap();
        self.frequency[temp_line][temp_col] += 1

        //        self.frequency[self.var.iter().position(|x| x == &var).unwrap()][req_number%self.timeblock_size] += 1
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }
}


#[cfg(test)]
mod tests {
    use crate::heatmap::heatmap2::Heatmap;
    
    #[test]
    fn test() {
        let mut heatmap = Heatmap::new("GET".to_string(), 4);
        heatmap.add_data("var2".to_string(), 0, );
        println!("{:?}", heatmap);
        heatmap.add_data("var".to_string(),  1);
        println!("{:?}", heatmap);
        heatmap.add_data("var2".to_string(),  2);
        println!("{:?}", heatmap);
        heatmap.add_data("var".to_string(),  3);
        println!("{:?}", heatmap);
        heatmap.add_data("var".to_string(),  4);
        println!("{:?}", heatmap);
        heatmap.add_data("var".to_string(),  5);
        println!("{:?}", heatmap);
        heatmap.add_data("var".to_string(),  6);
        println!("{:?}", heatmap);
        heatmap.add_data("var".to_string(),  7);
        println!("{:?}", heatmap);
    }
}