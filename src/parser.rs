use std::fs::File;
//use std::path::Path;
use std::io::{self, BufReader, BufRead};
use std::collections::VecDeque;

pub fn parse(line:String, queue:&mut VecDeque<Vec<String>>) -> () {
    let parts: std::str::Split<&str> = line.split(" ");
    let mut vec: Vec<String> = Vec::new();
    for mut i in parts {
        let text = i;
        let start_ch = text.chars().nth(0).unwrap().to_string();
        let end_ch = text.chars().last().unwrap().to_string();
        if start_ch=="\"" || start_ch == "[" {
            i = i.trim_start();
        }
        if end_ch=="\"" || end_ch == "]" {
            i = i.trim_end();
        }
        vec.push(i.to_string())
    }
    queue.push_back(vec);
}

pub fn insert_queue(filepath:&str, queue:&mut VecDeque<Vec<String>>) -> io::Result<()> {
    let file = File::open(filepath)?;
    let f = BufReader::new(file);

    let mut count = 0;
    for line in f.lines() {
        let other_line = line.unwrap();
        parse(other_line, queue);
        count+=1;
        if count==5 {
            break;
            // só pra não usar as 1000 linhas do log
            // TODO: retirar para versão final
        }
    }
    Ok(())
}
