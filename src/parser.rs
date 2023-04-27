use std::fs::File;
use std::io::prelude::*;
//use std::path::Path;
use std::io::{self, BufReader};
use std::collections::VecDeque;

pub fn parse(filepath:&str) -> io::Result<()> {
    let file = File::open(filepath)?;
    let f = BufReader::new(file);

    let mut count = 0;
    for line in f.lines() {
        println!("{}", line.unwrap());
        count+=1;
        if count==5 {
            break;
            // só pra não printar as 1000 linhas do log
        }
    }

    Ok(())
}

pub fn parse_to_queue(filepath:&str, queue:&VecDeque<&str>) -> io::Result<()> {
    let file = File::open(filepath)?;
    let f = BufReader::new(file);

    let mut count = 0;
    for line in f.lines() {
        let other_line = line.unwrap();
        queue.push_back(other_line.as_str());
        count+=1;
        if count==5 {
            break;
            // só pra não printar as 1000 linhas do log
        }
    }
    Ok(())
}