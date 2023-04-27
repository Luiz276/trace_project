use std::collections::VecDeque;

fn main() -> () {
    let fila = VecDeque::new();
    fila.push_back(1);
    fila.push_back(2);
    fila.push_back(3);
    let x = fila.pop_front();
    println!("{0}", x);
}