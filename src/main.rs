mod world;
use world::components::*;
use world::input::*;
use world::visuals::*;
use world::systems::*;
use world::world::*;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io;

fn main() {
   
    // Retrieve terminal size
    let term_size = termion::terminal_size().unwrap();
    let game_field = GameField::set_game_field(&term_size);

    // Welcome Screen
    RenderingSystem::render_welcome();
    let game = input_screen(); 

    if game {

        // Game Setup
        let mut world = World::new();
        world.create_entity(EntityType::Human, &term_size);
        world.create_entity(EntityType::Monster, &term_size);
        world.create_entity(EntityType::Weapon, &term_size);

        // Setup necessary Systems
        let weapon = WeaponSystem;
        let velocity = VelocitySystem;

        // Setup Input channel
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        
        loop {
           
            // Render game
            let pos_set = RenderingSystem::render_world(&world, &game_field);

            // Take user Input
            let stdin_channel = spawn_stdin_channel();
            let user_input = take_user_input(stdin_channel); 
            if user_input == Action::End {
                break
            }

            // Run Systems
            WeaponSystem::try_pick_up(&mut world, &pos_set.0, &pos_set.1);
            VelocitySystem::run(&mut world, &pos_set.0, &user_input);
        }
    }



}
