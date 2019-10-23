use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::collections::{VecDeque, HashMap};
use crate::span::godswood::node::*;
use serde_json::Value;
use crate::utils::JsonMap;

pub struct Godswoods {
    pub woods: Rc<RefCell<HashMap<String, Rc<RefCell<Godswood>>>>>,
    pub store: Rc<Store>,
}

impl Godswoods {
    pub fn new() -> Self {
        Self {
            woods: Rc::new(RefCell::new(HashMap::new())),
            store: StoreProto::new(),
        }
    }

    pub fn add_wood(&mut self, raw: &Value) {
        let mut wood = GodswoodProto::default(self.store.clone());
        wood.parse_from_json(raw);
        wood.init_nodes();
        let mut woods = self.woods.borrow_mut();
        let name = wood.read_name();
        let mut godswood = Godswood {
            wood,
            scales: HashMap::new(),
            base_scale: 4.0,
            base_gap: 20.0,
        };

        godswood.calculate_scales();
        woods.insert(name, Rc::new(RefCell::new(godswood)));
    }
}

pub struct GodswoodProto {
    depth: usize,
    nodes_by_depth: Rc<RefCell<HashMap<usize, Vec<Weak<Godsnode>>>>>,
    root: Weak<Godsnode>,
    store: Rc<Store>,
}

impl GodswoodProto {
    pub fn init_nodes(&mut self) {
        let nodes_by_depth = self.nodes_by_depth.clone();
        log!("Initializing nodes for wood");
        if let Some(node) = self.root.upgrade() {
            let mut nodes_by_depth = nodes_by_depth.borrow_mut();

            // Flush nodes queue first
            nodes_by_depth.clear();
            let app_name: String;
            let app_display_name: String;
            let mut app_meta: GodswoodMeta;
            {
                let mut app = node.borrow_mut();
                // info!("Drawing tree architecture of application {}", app.display_name);
                app_display_name = app.display_name.clone();
                app_meta = GodswoodMeta::new();
                app_name = app.name.clone();
                app_meta.path.append(&app.name);
                self.depth = app_meta.path.read_depth();
                app.app_meta_map.insert(app_name.clone(), app_meta.clone());
                self.store.update_index(&app_meta.path.read(), app.id);
                let entry = nodes_by_depth.entry(self.depth).or_insert(Vec::new());
                entry.push(self.root.clone());

            }
            let mut tasks: VecDeque<InitGodsnodeQ> = VecDeque::new();
            tasks.push_back(InitGodsnodeQ {
                app_meta: app_meta,
                nodes: node.borrow().children.clone(),
            });

            loop {
                let task = tasks.pop_front();
                if task.is_none() {
                    break;
                }
                let task = task.unwrap();

                for child in task.nodes.iter() {
                    if let Some(node) = child.upgrade() {
                        let mut kid_app_meta = task.app_meta.clone();
                        let mut kid = node.borrow_mut();
                        kid_app_meta.path.append(&kid.name);
                        log!("Initializing {}", kid_app_meta.path.read());
                        self.depth = kid_app_meta.path.read_depth();
                        kid.app_meta_map.insert(app_name.clone(), kid_app_meta.clone());
                        self.store.update_index(&kid_app_meta.path.read(), kid.id);
                        let entry = nodes_by_depth.entry(self.depth).or_insert(Vec::new());
                        entry.push(child.clone());

                        tasks.push_back(InitGodsnodeQ {
                            app_meta: kid_app_meta.clone(),
                            nodes: kid.children.clone(),
                        });
                    }
                }
            }

            // info!("Finished drawing tree architecture of application {}", app_display_name);
        }

    }

    // Sample application tree
    // app:
    //   children:
    //     node1:
    //       children:
    //          node3:
    //             children:
    //     node2:
    //       children

    pub fn parse(&mut self, raw:& Value) {
        let root = self.store.add_app_node(&raw);
        self.root = Rc::downgrade(&root);
        if let Some(children) = raw["children"].as_object() {
            if !children.is_empty() {
                GodswoodProto::parse_children(&root, &children, &mut self.store);
            }
        }
    }

    pub fn parse_children(parent_node: &Rc<Godsnode>, children: & JsonMap, store: &mut Rc<Store>) {
        for (name, raw) in children.iter() {
            let mut node = store.add_node(&raw, name.clone());
            if let Some(sub_children) = raw["children"].as_object() {
                if ! sub_children.is_empty() {
                    GodswoodProto::parse_children(&mut node, sub_children, store);
                }
            }
            let mut parent = parent_node.borrow_mut();
            parent.add_child(Rc::downgrade(&node));
            let mut child = node.borrow_mut();
            child.add_parent(Rc::downgrade(parent_node));
            // info!("linking parent {} with child {}", parent.name, child.name);
        }
    }


        fn default(store: Rc<Store>) -> Self {
        Self {
            depth: 0,
            nodes_by_depth: Rc::new(RefCell::new(HashMap::new())),
            root: Weak::new(),
            store: store,
        }
    }

    pub fn parse_from_json(&mut self, raw: &Value) {
        self.parse(raw);
    }

    pub fn get_nodes_by_depths(&self) -> &Godsnodes {
        &self.nodes_by_depth
    }
    pub fn get_depth(&self) -> usize {
        self.depth
    }
    pub fn get_root(&self) -> Weak<RefCell<GodsnodeProto>> {
        self.root.clone()
    }

    pub fn read_name(&self) -> String {
        let root = self.root.clone();
        root.upgrade().unwrap().borrow().name.clone()
    }
}

pub struct Godswood {
    pub wood: GodswoodProto,
    pub scales: HashMap<usize, f32>,
    pub base_scale: f32,
    pub base_gap: f32,
}

impl Godswood {
    pub fn new(wood: GodswoodProto) -> Godswood {
        Godswood {
            wood,
            scales: HashMap::new(),
            base_scale: 1.0,
            base_gap: 5.0,
        }
    }

    fn calculate_scales(&mut self) {
        log!("Initializing scales for wood");
        let nodes = self.wood.get_nodes_by_depths();
        let depth = self.wood.get_depth();
        let nodes = nodes.borrow();
        for i in 1..depth {
            let items = nodes.get(&i).unwrap();
            let mut kids_max = 0;
            for item in items.iter() {
                if let Some(node) = item.upgrade() {
                    let node = node.borrow();
                    let kids = node.get_children();
                    if kids.len() > kids_max {
                        kids_max = kids.len();
                    }
                } else {
                    // error!("Unexpected weak node lost connection");
                }
            }

            let scale: f32;

            if kids_max <= 1 {
                scale = 1.0;
            } else {
                let angle = PI as f32/ kids_max as f32;
                scale = 1.0 / angle.sin() + 1.0;
            }

            log!("Calculated scale {} for depth {}", scale, i);
            self.scales.insert(i, scale);
        }
        self.scales.insert(depth, 1.0);
        let mut scale = 1.0f32;
        for i in (1..depth).rev() {
            let v = self.scales.get_mut(&i).unwrap();
            *v *= scale;
            scale = *v;
        }
    }

    pub fn render_test(&self) {
        let nodes = self.wood.get_nodes_by_depths();
        let max_depth = self.wood.get_depth();
        let nodes = nodes.borrow();
        for i in 1..max_depth + 1 {
            let items = nodes.get(&i).unwrap();
            // info!("Drawing nodes with depth: {}", i);
            let mut kids_max = 0;
            for item in items.iter() {
            }
        }
    }

    /*
    pub fn render(self) {
        let root = self.wood.get_root();
        Godswood::<N, T>::draw_node(root.clone());
        let node  = root.upgrade().unwrap();
        let parent = node.read().unwrap();
        let children = parent.get_children();
        if children.len() > 0 {
        }
    }
    */
}
