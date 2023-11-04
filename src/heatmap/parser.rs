use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::VecDeque;

fn parse_fix_fields(vec:&Vec<String>) -> Vec<String> {
    let mut final_vec = Vec::new();
    for i in 0..6 {
        final_vec.push(vec[i].clone());
        //println!("{}", vec[i])
    }
    for i in 0..vec.len() {
        if vec[i].contains("field") {
            //let mut temp_vec: Vec<String> = Vec::new();
            //let mut temp_val:String = "".to_string();
            let mut new_vec: Vec<String> = Vec::new();
            for j in i+1..vec.len() {
                if vec[j].contains("field") {
                    //new_vec.push(temp_val);
                    break;
                }
                let temp_val = vec[j].clone();
                new_vec.push(temp_val);
                new_vec.push(" ".to_string())
            }
            new_vec.remove(new_vec.len()-1);
            final_vec.push(new_vec.concat());
        }
    }
    return final_vec;
}

fn parse(line:String, command_queue:&mut VecDeque<Vec<String>>, ts_queue: &mut VecDeque<f64>) -> () {
    let parts: std::str::Split<&str> = line.split(" ");
    let mut vec: Vec<String> = Vec::new();
    for i in parts {
        // let start_ch = text.chars().nth(0).unwrap().to_string();
        // let end_ch = text.chars().last().unwrap().to_string();
        let mut new_i = i.trim_start_matches(['\"', '[']);
        new_i = new_i.trim_end_matches(['\"', ']']);
        vec.push(new_i.to_string())
    }
    let mut comm_vec:Vec<String> = vec.clone();
    ts_queue.push_back(vec[0].parse().unwrap());
    if vec[3].contains("HMSET") {
        comm_vec = parse_fix_fields(&vec);
    }
    command_queue.push_back(comm_vec);
}

pub fn insert_queue(filepath:&str, command_queue:&mut VecDeque<Vec<String>>, ts_queue: &mut VecDeque<f64>) -> io::Result<()> {
    let file = File::open(filepath)?;
    let f = BufReader::new(file);

    let mut count = 0;
    for line in f.lines() {
        let other_line = line.unwrap();
        parse(other_line, command_queue, ts_queue);
        count+=1;
        if count==6 {
            break;
            // só pra não usar as 1000 linhas do log
            // TODO: retirar para versão final
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //------------------------------------------------------------------------//
    //                                                                        //
    //nenhum teste vai passar por causa de um valor fixado em parse_fix_fields//
    //                                                                        //
    //------------------------------------------------------------------------//

    use crate::heatmap::parser::parse_fix_fields;

    fn init_single_field() -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push("123".to_string());
        vec.push("field23".to_string());
        vec.push("abc".to_string());
        vec.push("def".to_string());
        return vec
    }

    fn init_multiple_field() -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        vec.push("123".to_string());
        vec.push("field23".to_string());
        vec.push("abc".to_string());
        vec.push("def".to_string());
        vec.push("field".to_string());
        vec.push("321".to_string());
        return vec
    }

    #[test]
    fn test_parse_fix_fields_multiple_fields() {
        //let vec = ["123", "field23", "abc", "def", "field"];
        let vec: Vec<String> = init_multiple_field();
        let mut correct = Vec::new();
        correct.push("123".to_string());
        correct.push("field23".to_string());
        correct.push("abc def".to_string());
        correct.push("field".to_string());
        correct.push("321".to_string());
        let res = parse_fix_fields(&vec);
        assert_eq!(correct, res);
    }

    #[test]
    fn test_parse_fix_fields_single_fields() {
        let vec = init_single_field();
        let mut correct = Vec::new();
        correct.push("123".to_string());
        correct.push("field23".to_string());
        correct.push("abc def".to_string());
        let res = parse_fix_fields(&vec);
        assert_eq!(correct, res);
    }
}
