#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::ChangeColor(x, y, z) => println!("ChangeColor {}-{}-{}", x, y, z),
        Message::Move { x, y: new_y } => println!("Move {}-{}", x, new_y),
        Message::Write(text) => println!("Write {}", text),
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_large_enum() {
        let quit = Message::Quit;
        process_message(quit);
        let changeColor = Message::ChangeColor(44, 55, 66);
        process_message(changeColor);
        let my_move = Message::Move { x: 11, y: 88 };
        process_message(my_move);
        let my_write = Message::Write("ZZZZZZZZZZZZZZ".to_string());
        process_message(my_write);
    }

    #[test]
    fn tests_patt() {
        dbg!("Hello tests_patterns!");
    }
}
