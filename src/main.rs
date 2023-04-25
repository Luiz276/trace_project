mod parser;

fn main() {
    match parser::parse("./redis_get_set.log") {
        Err(_) => panic!("abort"),
        Ok(_) => println!("OK")
    };
    //parser::parse("redis_get_set.log")
}
