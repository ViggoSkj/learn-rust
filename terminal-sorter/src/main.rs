mod quick_sort;

enum TerminalInput {
    Sort,
    Number(u32),
    Invalid,
    Error,
}

fn parse_terminal_string(string: &String) -> TerminalInput {
    let number = match string.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            if string.trim().to_lowercase() == "sort" {
                return TerminalInput::Sort;
            }
            return TerminalInput::Invalid;
        }
    };

    return TerminalInput::Number(number);
}

fn get_terminal_input() -> TerminalInput {
    let mut buffer = String::new();

    print!(": ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!"); // flush the std io buffer so we can see print before whe read stdin
    let r = std::io::stdin().read_line(&mut buffer);

    match r {
        Err(_) => return TerminalInput::Error,
        _ => (),
    }

    return parse_terminal_string(&buffer);
}

fn main() {
    let mut numbers: Vec<u32> = Vec::new(); //vec![100, 1, 5, 1, 5, 7];
    
    loop {
        let input = get_terminal_input();
        match input {
            TerminalInput::Number(number) => {
                numbers.push(number);
                println!("Adding number {}", number);
                println!("Numbers in the list: {:?}", numbers);
            }
            TerminalInput::Sort => {
                println!("Sorting...");
                quick_sort::sort(&mut numbers);
                println!("sorted {:?}", numbers);
            }
            TerminalInput::Invalid => {
                println!("Invalid input");
            }
            TerminalInput::Error => {
                println!("Somthing when wrong");
            }
        }
    }
}
