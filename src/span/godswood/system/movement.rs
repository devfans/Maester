use std::rc::Rc;
use dragon::ecs::*;
use dragon::core::*;
use wand::input::Input;

pub struct MovementSystem {
    state: Rc<WorldState>,
    input: Input,
}

impl MovementSystem {
    pub fn new(state: Rc<WorldState>, input: Input) -> Self {
        Self {
            state,
            input,
        }
    }
}

impl System for MovementSystem {
    fn tick(&mut self) {
        let c_store = self.state.component_store.borrow();
        let meshes = c_store.get::<MeshComponent>();
        let mut transforms = c_store.get_mut::<TransformComponent>();
        for (_, transform) in transforms.iter_mut().filter(|(entity, _)| {
            meshes.contains_key(entity)
        }) {
            transform.append_rotation(
                Vector3::y_axis(),
                self.input.borrow_mut().axis("a", "d") * 0.1
            );
            transform.append_rotation(
                Vector3::x_axis(),
                self.input.borrow_mut().axis("w", "s") * 0.1
            );
            transform.prepend_translation(
                Vector3::new(0., 0., self.input.borrow_mut().axis("v", "b") * 0.6)
            );
        }
        
        let active_camera = self.state.active_camera.get();
        let camera = transforms.get_mut(&active_camera).unwrap();
        camera.append_rotation(
            Vector3::y_axis(),
            self.input.borrow_mut().axis("ArrowLeft", "ArrowRight") * 0.1
        );
        camera.append_rotation(
            Vector3::x_axis(),
            self.input.borrow_mut().axis("ArrowUp", "ArrowDown") * 0.1
        );
        camera.prepend_translation(
            Vector3::new(0., 0., self.input.borrow_mut().axis("z", "x") * 0.6)
        );

        camera.prepend_translation(
            Vector3::new(0., self.input.borrow_mut().axis("i", "k") * 0.6, 0.)
        );

        let angle = self.input.borrow_mut().axis("j", "l") * 0.02 as f32;
        let trans = camera.translation().clone();

        let pos = Point3::new(0., 0., 0.) + UnitQuaternion::from_axis_angle(&Vector3::y_axis(), angle) * trans;
        // camera.set_translation_xyz(pos.x, pos.y, pos.z);
        camera.prepend_rotation(Vector3::y_axis(), angle);
    }
}

