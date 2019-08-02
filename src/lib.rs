use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

enum MoveX {
    Left,
    Right,
}

enum MoveY {
    Up,
    Down,
}

// initialise coordinates
pub struct Coordinates {
    pub cursor_x: u16,
    pub cursor_y: u16,
} 

impl Coordinates {
    fn new() -> Self {
        Self {
            cursor_x: 1,
            cursor_y: 1,
        }
    }

    fn move_x(&mut self, d: MoveX) {
        match d {
            MoveX::Left => {
                if self.cursor_x == 1 {}
                else { self.cursor_x -= 1 }
            },
            MoveX::Right => {
                self.cursor_x += 1
            },
        }
    }

    fn move_y(&mut self, d: MoveY) {
        match d {
            MoveY::Up => {
                if self.cursor_y == 1 {}
                else { self.cursor_y -= 1 }
            },
            MoveY::Down => {
                self.cursor_y += 1
            },
        }
    }
}

pub fn set_up() {
    
    // Get the standard input stream
    let stdin = stdin();

    // Get the standard output stream and go to raw mode
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Initialise coos
    let mut coos = Coordinates::new();

    // Welcome message
    write!(stdout, "{}{}Welcome to Game. To exit please type q.{}",
           termion::clear::All,
           termion::cursor::Goto(coos.cursor_x, coos.cursor_y),
           termion::cursor::Hide).unwrap();

    stdout.flush().unwrap();

    // Accept Input
    for c in stdin.keys() {

         // Clear the current line.
         for i in 1..100 {
             write!(stdout, "{}{}", termion::cursor::Goto(1, i), termion::clear::CurrentLine).unwrap();
         }

         // Print the key we type...
         match c.unwrap() {
             // Exit.
             Key::Char('q') => break,
             Key::Char(c)   => println!("{}", c),
             Key::Alt(c)    => println!("Alt-{}", c),
             Key::Ctrl(c)   => println!("Ctrl-{}", c),

             // Move around game
             Key::Left      => {
                 coos.move_x(MoveX::Left);
                 println!("{}o", termion::cursor::Goto(coos.cursor_x, coos.cursor_y));
             },
             Key::Right      => {
                 coos.move_x(MoveX::Right);
                 println!("{}o", termion::cursor::Goto(coos.cursor_x, coos.cursor_y));
             },
             Key::Up      => { 
                 coos.move_y(MoveY::Up);
                 println!("{}o", termion::cursor::Goto(coos.cursor_x, coos.cursor_y)); 
             },
             Key::Down      => {
                 coos.move_y(MoveY::Down);
                 println!("{}o", termion::cursor::Goto(coos.cursor_x, coos.cursor_y));
             },

             _              => println!("Other"),
         }

         // Flush again.
         stdout.flush().unwrap();
     }

     // Show the cursor again before we exit
     write!(stdout, "{}", termion::cursor::Show).unwrap();
}
