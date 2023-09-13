// mod heatmap2;
// pub mod parser;

pub mod implementationSpecificOperations {
    use std::collections::VecDeque;

    use crate::Heatmap;
    // fn parse_into(&self, filepath: &str) -> () {
    //     /* TODO */
    //     let mut fila: VecDeque<Vec<String>> = VecDeque::new();

    //     match parser::insert_queue(filepath, &mut fila) {
    //         Err(_) => panic!("abort"),
    //         Ok(_) => println!("parse to queue OK")
    //     }

    // }

    pub fn create_heatmaps(mut lista_heatmaps: Vec<Heatmap>, mut fila_linhas_log: VecDeque<Vec<String>>) -> Vec<Heatmap> {
        let mut vec_commands: Vec<String> = Vec::new();

        while !fila_linhas_log.is_empty() {
            let linha = fila_linhas_log.pop_front();
            match linha {
                Some(line) => {
                    let _timestamp = line[0].clone();
                    let command:String = line[3].clone();
                    let temp_command = command.clone();
                    if !vec_commands.contains(&command) {
                        vec_commands.push(command.clone());
                        lista_heatmaps.push(Heatmap::new(command, 4))
                    }
                    let mut target_htmp: &Heatmap;
                    for htmp in &lista_heatmaps[..] {
                        if htmp.get_command() == &temp_command {
                            target_htmp = htmp;
                            break;
                        }
                    }
                    /* TODO: Parse data into the previously created heatmaps */
                },
                None => println!("Empty line in log"),
            }
            //htmp.add
        }
        lista_heatmaps
    }
}