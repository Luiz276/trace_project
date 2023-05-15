use std::collections::VecDeque;

//use redis::Commands;

// pub fn send_command() -> () {
//     //let client = redis::Client::open("redis://127.0.0.1:6379/");
//     let client = match redis::Client::open("redis://127.0.0.1:6379/") {
//         Ok(client) => client,
//         Err(_) => panic!("abort")
//     };
//     let con = client.get_connection();
//     let mut con = match con {
//         Ok(conexao) => conexao,
//         Err(_) => panic!("abort")
//     };

//     let bar: String = redis::cmd("GET")
//         .arg("foo")
//         .query(&mut con)
//         .expect("failed to execute GET for 'foo'");
//     println!("value for 'foo' = {}", bar);
// }

pub fn send_command(vec: VecDeque<Vec<String>>) -> () {
    let client = match redis::Client::open("redis://127.0.0.1:6379/") {
        Ok(client) => client,
        Err(_) => panic!("abort")
    };

    let con = client.get_connection();

    let mut con = match con {
        Ok(conexao) => conexao,
        Err(_) => panic!("abort")
    };

    for i in vec {
        let command = &i[3];
        let data = &i[4];
        if command == &String::from("HMSET") {
            let field = &i[5];
            let data2 = &i[6];

            let _: String = redis::cmd(command.as_str())
            .arg(data.as_str())
            .arg(field.as_str())
            .arg(data2.as_str())
            .query(&mut con)
            .expect("failed to execute HMSET");
            println!("HMSET")
        } else {
            let _: () = redis::cmd(command.as_str())
            .arg(data.as_str())
            .query(&mut con)
            .expect("failed to execute HMGETALL");
            //println!("{:?}", res);
            println!("HMGETALL")
        }
    }
    println!("REQS FINISHED")
}
