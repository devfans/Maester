use dragon::{core, ecs};
mod system;
mod component;
mod node;
mod tree;
mod stage;
use system::movement::MovementSystem;
use stage::Godsstage;


pub fn create_godswood(app: &wand::Application) -> wand::WorldSpan {
    let state = app.get_state();
    let world_span = wand::WorldSpan::new(state.clone(), app.context.clone(), "world", "World", 1., 1.);
    let w = &world_span.world.state;
    // Register movement system and enter godsstage
    {
        let movement_system = MovementSystem::new(w.clone(), app.input.clone());
        w.register_system("movement", movement_system);

        let stage = Godsstage::new(w.clone());
        w.enter("godswood", stage);
    }
    world_span
}
