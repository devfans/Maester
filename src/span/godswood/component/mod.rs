use std::rc::Rc;
use dragon::ecs::Component;
use crate::span::godswood::node::Godsnode;

pub struct GodsnodeComponent {
    pub node: Rc<Godsnode>,
}

impl Component for GodsnodeComponent {}

