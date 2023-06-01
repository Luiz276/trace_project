mod heatmap2;
pub mod parser;

impl Redis for ImplementationSpecificOperations {
    fn parse_into(&self, filepath: &str) -> () {
        let mut fila: VecDeque<Vec<String>> = VecDeque::new();
    }
}