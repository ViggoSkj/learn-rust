use std::collections::HashMap;



struct Command {
    function: Box<dyn Fn()>,
}



fn ping() {
    println!("pong")
}

fn main() {

    let ping_command = Command {
        function: Box::new(ping),
    };

    let mut commands: HashMap<String, Command> = HashMap::new();

    commands.insert("ping".into(), ping_command);


    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let command = commands.get(&input);
        match command {
            Some(c) => (*c.function)(),
            None => println!("This command dose not exist"),
        }
    }
}
