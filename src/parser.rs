use std::fs::File;
//use std::path::Path;
use std::io::{self, BufReader, BufRead};
use std::collections::VecDeque;

pub fn parse(line:String) -> std::str::Split<'static, &'static str> {
    return line.split(" ");
}

pub fn insert_queue(filepath:&str, queue:&mut VecDeque<std::str::Split<'_, &str>>) -> io::Result<()> {
    let file = File::open(filepath)?;
    let f = BufReader::new(file);

    let mut count = 0;
    for line in f.lines() {
        let other_line = line.unwrap();
        queue.push_back(parse(other_line));
        count+=1;
        if count==5 {
            break;
            // só pra não usar as 1000 linhas do log
            // TODO: retirar para versão final
        }
    }
    Ok(())
}
