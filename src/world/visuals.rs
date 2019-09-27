pub const GAME_WELCOME: &'static str = "╔══════════════════════════════════════╗\n\r\
                                        ║───┬Welcome to this beautiful Game────║\n\r\
                                        ║ p ┆ play                             ║\n\r\
                                        ║ q ┆ quit                             ║\n\r\
                                        ╚═══╧══════════════════════════════════╝";

pub const GAME_GOODBYE: &'static str = "╔══════════════════════════════════════╗\n\r\
                                        ║       Well that is a shame!          ║\n\r\
                                        ║        See you next time!            ║\n\r\
                                        ╚══════════════════════════════════════╝";

pub struct GameField{
    pub new_gamefield: String,
}

impl GameField {
    pub fn set_game_field(dims: &(u16, u16)) -> Self {

        let mut new_gamefield = String::from("╔");
        let top_end: &str = "╗\n\r";
        let middle_start: &str = "║";
        let middle_end: &str = "║\n\r";
        let bottom_start: &str = "╚";
        let bottom_end: &str = "╝";
        let extension: &str = "═";

        // Top row
        for i in 1..dims.0-1 {
            new_gamefield.push_str(extension);
        }
        new_gamefield.push_str(top_end);

        // Inbetween row
        for i in 1..dims.1-2 {
            new_gamefield.push_str(middle_start);
            for y in 1..dims.0-1 {
                new_gamefield.push_str(" ");
            }
            new_gamefield.push_str(middle_end);
        }

        // Bottom row
        new_gamefield.push_str(bottom_start);
        for i in 1..dims.0-1 {
            new_gamefield.push_str(extension);
        }
        new_gamefield.push_str(bottom_end);

        Self { new_gamefield }
    }
}

