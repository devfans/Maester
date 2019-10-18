use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
use std::f32::consts::PI;
use dragon::{ecs::{WorldState, Stage, TransformComponent}, core};
use crate::span::godswood::tree::*;
use crate::span::godswood::component::GodsnodeComponent;


pub struct Godsstage {
    state: Rc<WorldState>,
    woods: Godswoods,
}

impl Godsstage {
    pub fn new(state: Rc<WorldState>) -> Self {
        Self {
            state,
            woods: Godswoods::new(),
        }
    }
}

impl Stage for Godsstage {
    fn on_enter(&mut self) {
        let wood = r#"
            {
                  "name": "sample-application",
                  "children": {
                    "service1": {
                      "children": {
                        "service5": {
                          "children": {
                            "service6": {},
                            "service7": {}
                          }
                        }
                      }
                    },
                    "service2": {
                      "children": {
                        "service10": {
                          "children": {
                            "service21": {},
                            "service22": {},
                            "service23": {}
                          }
                        },
                        "service11": {
                          "children": {
                          }
                        }
                      }
                    },
                    "service4": {
                      "children": {
                        "service3": {}
                      }
                    }
                  }
                }
            "#;
        self.woods.add_wood(&serde_json::from_str(wood).unwrap());
        let vertices: Vec<core::Point3<f32>> = [
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

        macro_rules! create_node {
            ($node: expr, $pos: expr, $id: expr, $name: expr) => {
                {
                    // Attach cube entity
                    let entity = self.state.create_entity();
                    let mut transform = TransformComponent::default();
                    transform.set_translation_xyz($pos.0, $pos.1, $pos.2);
                    let mesh = core::BasicMesh::new(vertices.clone(), vec!(9, 11, 13));
                    self.state.bind_component(entity, mesh);
                    self.state.bind_component(entity, transform);
                    self.state.bind_component(entity, GodsnodeComponent { node: $node });
                }
            }
        }

        let mut nodes = VecDeque::new();
        let woods = self.woods.woods.borrow();
        for wood in woods.values() {
            let wood = wood.borrow();
            // let gap = wood.base_gap * -1.0f32;

            nodes.push_back(((0.0, 0.0, -100.0), wood.wood.get_root(), 1));

            loop {
                let node = nodes.pop_front();
                if node.is_none() {
                    break;
                }

                let ((x, y, z), node, depth) = node.unwrap();
                let node_arc = node.upgrade().unwrap();
                let scale = wood.scales.get(&depth).unwrap() * wood.base_scale;
                let node = node_arc.borrow();
                create_node!(node_arc.clone(), (x, y, z), node.id, node.name.clone());

                let children = node.get_children();
                let size = children.len();
                if size == 0 {
                    continue;
                } else if size == 1 {
                    // draw_line!(Point3::new(x, y, z), Vector3::new(0.0, -wood.base_gap, 0.0));
                    nodes.push_back(((x, y - wood.base_gap, z), children[0].clone(), depth + 1));
                    continue;
                }

                // draw_line!(Point3::new(x, y, z), direction);
                // draw_circle!(Point3::new(x, y - wood.base_gap, z), scale);
                // break;

                let mut points = Vec::new();

                let angle = 2f32 * PI / size as f32;
                for i in 0..size {
                    let angle = angle * i as f32;
                    let kid_x = x - scale * angle.cos();
                    let kid_y = y - wood.base_gap;
                    let kid_z = z - scale * angle.sin();

                    // draw_line!(Point3::new(x, y, z), Vector3::new(kid_x - x, kid_y - y, kid_z - z));
                    points.push((kid_x, kid_y, kid_z));
                }

                for node in children.iter() {
                    nodes.push_back((points.pop().unwrap(), node.clone(), depth + 1));
                }
            }
        }
    }
}
