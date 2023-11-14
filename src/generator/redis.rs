// mod heatmap2;
// pub mod parser;

pub mod implementation_specific_operations {

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
    fn find_htmp(lista_heatmaps:&mut Vec<Heatmap>, command: String) -> Result<&mut Heatmap, &'static str> {
        for htmp in lista_heatmaps {
            if htmp.get_command() == &command {
                return Ok(htmp)
            }
        }
        return Err("No htmp in lista_htmp");
    }

    pub fn create_heatmaps(mut lista_heatmaps: Vec<Heatmap>, mut fila_linhas_log: VecDeque<Vec<String>>) -> Vec<Heatmap> {
        let mut vec_commands: Vec<String> = Vec::new();
        let i = 0;

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
                    let mut target_htmp;
                    match find_htmp(&mut lista_heatmaps, temp_command) {
                        Ok(htmp) => target_htmp = htmp,
                        Err(error) => panic!("{:?}",error)
                    }
                    // lista_heatmaps.into_iter().find(x.command == &temp_command);
                    /* TODO: Parse data into the previously created heatmaps */
                    target_htmp.add_data(line[4].clone(), i)
                },
                None => println!("Empty line in log"),
            }
            //htmp.add
        }
        lista_heatmaps
    }
}