fn main() {
   
    // Retrieve terminal size
    term_size = termion::terminal_size().unwrap();
    game_field = GameField::set_game_field(&term_size);

    // Welcome Screen
    render.render_welcome();
    game = input_screen(); 

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
        let stdin_channel = spawn_stdin_channel();
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        
        loop {
           
            // Render game
            pos_set = render.render_world(&world, &game_field);

            // Take user Input
            let user_input = take_user_input(stdin_channel); 
            if user_input == Action::End {
                break
            }

            // Run Systems
            world = weapon.try_pick_up(world, pos_set.0, pos_set.1);
            velocity.run(mut world, pos_set.0, user_input);



        }
    }



}
