use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn spawn_stdin_channel() -> Receiver<termion::event::Key> {

    let mut stdin = termion::async_stdin().keys();
    let (tx, rx) = mpsc::channel::<termion::event::Key>();

    thread::spawn(move || {
        loop {
            let input = stdin.next();

            if let Some(Ok(key)) = input {
                tx.send(key).unwrap()
            }
        }
    });
    rx
}

fn pick_last_value(latest_keys: std::sync::mpsc::TryIter<termion::event::Key>) -> termion::event::Key {
    let mut final_val = termion::event::Key::Null;
    for x in latest_keys {
        final_val = x;
    }
    final_val
}

fn input_screen() -> bool {

    let stdin_channel = spawn_stdin_channel();
    let game = false;

    loop {
        
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        let latest_keys = stdin_channel.try_iter();
        let final_val = pick_last_value(latest_keys);
    
        match parse_input_welcome(final_val) {
            termion::event::Key::Char('p') => {
                game = true;
                break;
            },
            termion::event::Key::Char('q') => {
                break;
            } 
        }

        sleep(500)
    }
}


pub take_user_input(rx: Receiver<termion::event::Key>) {
        
    let latest_keys = rx.try_iter();
    let final_val = pick_last_value(latest_keys);


}

pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Shot,
    Break,
    End
}

pub fn parse_game_input(key: termion::event::Key) -> Action {
    match key {
        termion::event::Key::Char('q') => Action::End,
        termion::event::Key::Char('b') => Action::Break,
        termion::event::Key::Char('A') => Action::Up,
        termion::event::Key::Char('B') => Action::Down,
        termion::event::Key::Char('D') => Action::Left,
        termion::event::Key::Char('C') => Action::Right,
        termion::event::Key::Char('s') => Action::Shot,
    }
}
