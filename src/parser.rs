use std::fs::File;
use std::io::prelude::*;
//use std::path::Path;
use std::io::{self, BufReader};

pub fn parse(filepath:&str) -> io::Result<()> {
    // let path = Path::new(filepath);
    // let display = path.display();

    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }
    let file = File::open(filepath)?;
    let f = BufReader::new(file);

    let mut count = 0;
    for line in f.lines() {
        println!("{}", line.unwrap());
        count+=1;
        if count==5 {
            break;
            // só pra não printar as 100 linhas do log
        }
    }

    Ok(())
}