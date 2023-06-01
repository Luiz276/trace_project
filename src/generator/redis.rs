mod heatmap2;
pub mod parser;

impl Redis for ImplementationSpecificOperations {
    fn parse_into(&self, filepath: &str) -> () {
        /* TODO */
        let mut fila: VecDeque<Vec<String>> = VecDeque::new();

        match parser::insert_queue(filepath, &mut fila) {
            Err(_) => panic!("abort"),
            Ok(_) => println!("parse to queue OK")
        }

    }
}