use crossterm::event::{read, Event, KeyCode, KeyEvent};

fn user_input() -> String {
    let mut ans = String::new();
    loop {
        let event = read();
        if let Ok(e) = event {
            if let Event::Key(key) = e {
                match key.code {
                    KeyCode::Char(char) => ans.push(char),
                    KeyCode::Enter => return ans,
                    _ => {}
                }
            }
        }
    }
}
fn main() {
    println!("hello world");
    let _ans = user_input();
}
