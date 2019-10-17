use dragon::{core, ecs};
mod system;
mod component;
use system::movement::MovementSystem;


pub fn create_godswood(app: &wand::Application) -> wand::WorldSpan {
    let state = app.get_state();
    let world_span = wand::WorldSpan::new(state.clone(), app.context.clone(), "world", "World", 1., 1.);
    let w = &world_span.world.state;
    {
        // Attach cube entity
        let entity = world_span.world.state.create_entity();
        let vertices = [
            (-2., -2., 4.,),
            (2., -2., 4.),
            (2., 2., 4.),
            (-2., 2., 4.),
            (-2., -2., 4.,),
            (-2., -2., -4.,),
            (2., -2., -4.),
            (2., 2., -4.),
            (-2., 2., -4.),
            (-2., -2., -4.,),
            // other lines
            (2., -2., 4.),
            (2., -2., -4.),
            (2., 2., 4.),
            (2., 2., -4.),
            (-2., 2., 4.),
            (-2., 2., -4.)
        ]
            .into_iter()
            .map(|v| core::Point3::new(v.0, v.1, v.2)).collect();
        let mesh = core::BasicMesh::new(vertices, vec!(9, 11, 13));
        let mut transform = ecs::TransformComponent::default();
        transform.set_translation_xyz(-5., 0., -16.);
        w.bind_component(entity, mesh);
        w.bind_component(entity, transform);
        let movement_system = MovementSystem::new(w.clone(), app.input.clone());
        w.register_system("movement", movement_system);
    }

    // Add a simple mesh entity
    {
        // Attach cube entity
        let entity = world_span.world.state.create_entity();
        let vertices = [
            (-2., -2., 4.,),  // 0
            (2., -2., 4.),    // 1
            (2., 2., 4.),     // 2
            (-2., 2., 4.),    // 3
            (-2., -2., -4.,), // 0' 4
            (2., -2., -4.),   // 1' 5
            (2., 2., -4.),    // 2' 6
            (-2., 2., -4.),   // 3' 7
        ]
            .into_iter()
            .map(|v| core::Point3::new(v.0, v.1, v.2)).collect();
        let polygons = vec!(
            (0, 2, 1, "silver"),
            (0, 2, 3, "silver"),
            (4, 5, 6, "grey"),
            (4, 7, 6, "grey"),
            (0, 7, 3, "white"),
            (0, 7, 4, "white"),
            (1, 6, 2, "blue"),
            (1, 6, 5, "blue"),
            (2, 6, 7, "red"),
            (2, 3, 7, "red"),
            (0, 5, 1, "orange"),
            (0, 5, 4, "orange"),
        ).into_iter().map(|(a, b, c, d)| (a, b, c, d.to_string())).collect();
        let mesh = core::SimpleMesh::new(vertices, polygons);
        let mut transform = ecs::TransformComponent::default();
        transform.set_translation_xyz(5., 0., -16.);
        w.bind_component(entity, mesh);
        w.bind_component(entity, transform);
        let movement_system = MovementSystem::new(w.clone(), app.input.clone());
        w.register_system("movement", movement_system);
    }
    world_span
}
