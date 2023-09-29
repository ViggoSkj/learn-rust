enum TerminalInput {
    Sort,
    Number(u32),
    Invalid,
}

fn parse_terminal_string(string: &String) -> TerminalInput {
    let number = match string.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            if string.trim().to_lowercase() == "sort" {
                return TerminalInput::Sort
            }
            return TerminalInput::Invalid
        },
    };

    return TerminalInput::Number(number);
}

fn get_terminal_input() -> TerminalInput {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer);
    return parse_terminal_string(&buffer);
}

fn main() {
    let mut numbers: Vec<u32> = Vec::new();

    loop {
        let input = get_terminal_input();
        match input {
            TerminalInput::Number(number) => {
                numbers.push(number);
                println!("Adding number {}", number);
                println!("Numbers in the list: {:?}", numbers);
            },
            TerminalInput::Sort => {
                println!("Sorting...");
            }
            TerminalInput::Invalid => {
                println!("Invalid input");
            }
        }
    }
}
