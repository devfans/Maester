use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;
use serde_json::Value;
use crate::utils::*;

pub enum GodsnodeType {
    Root,
    Godsnode,
    Leaf,
}

pub enum GodsnodeClass {
    General,
}

pub type Godsnodes = Rc<RefCell<HashMap<usize, Vec<Weak<Godsnode>>>>>;
pub type GodsnodeQ = Vec<Weak<Godsnode>>;

pub struct InitGodsnodeQ {
    pub app_meta: GodswoodMeta,
    pub nodes: GodsnodeQ,
}

#[derive(Clone)]
pub struct GodswoodMeta {
    pub path: GodsnodePath,
}

impl GodswoodMeta {
    pub fn new() -> GodswoodMeta {
        GodswoodMeta {
            path: GodsnodePath::new_path(),
        }
    }

    pub fn parse_app_name(path: &String) -> Option<String> {
        if !path.starts_with('.') { return None }
        let tokens: Vec<&str> = path.split('.').collect();
        if tokens.len() > 1 && tokens[1].len() > 0 {
            return Some(tokens[1].to_string());
        }
        None
    }
}

#[derive(Clone)]
pub struct GodsnodePath {
    path: String,
    depth: usize,
}

impl GodsnodePath {
    pub fn new_path() -> GodsnodePath {
        GodsnodePath {
            path: String::new(),
            depth: 0,
        }
    }

    pub fn append(&mut self, name: &String) {
        self.path.push_str(&(".".to_owned() + name));
        self.depth += 1;
    }

    pub fn new(root: String) -> GodsnodePath {
        GodsnodePath {
            path: root,
            depth: 1,
        }
    }

    pub fn read(&self) -> String {
        self.path.clone()
    }

    pub fn read_depth(&self) -> usize {
        self.depth
    }
}

pub type GodswoodMetaMap = HashMap<String, GodswoodMeta>;

pub type Godsnode = RefCell<GodsnodeProto>;
pub struct GodsnodeProto {
    pub id: u64,
    pub name: String,
    pub display_name: String,
    pub node_type: GodsnodeType,
    pub parents: Vec<Weak<Godsnode>>,
    pub children: Vec<Weak<Godsnode>>,
    pub service_type: GodsnodeClass,
    pub app_meta_map: GodswoodMetaMap,
}

impl GodsnodeProto {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            display_name: String::new(),
            node_type: GodsnodeType::Godsnode,
            parents: Vec::new(),
            children: Vec::new(),
            service_type: GodsnodeClass::General,
            app_meta_map: HashMap::new(),
        }
    }

    pub fn get_children(&self) -> &Vec<Weak<RefCell<Self>>> {
        &self.children
    }

    pub fn get_parents(&self) -> &Vec<Weak<RefCell<Self>>> {
        &self.parents
    }

    pub fn add_parent(&mut self, node: Weak<Godsnode>) {
        self.parents.push(node);
    }

    pub fn add_child(&mut self, node: Weak<Godsnode>) {
        self.children.push(node);
    }
}

pub type Store = RefCell<StoreProto>;
pub struct StoreProto {
    id: u64,
    store: HashMap<u64, Rc<Godsnode>>,
    index: HashMap<String, u64>,
}

impl StoreProto {
    pub fn new() -> Rc<Store> {
        Rc::new(RefCell::new( Self {
            id: 0,
            store: HashMap::new(),
            index: HashMap::new(),
        }))
    }
}

pub trait StoreOps {
    fn new_node(&self) -> Rc<Godsnode>;
    fn add_node(&self, raw: &Value, name: String) -> Rc<Godsnode>;
    fn add_app_node(&self, raw: &Value) -> Rc<Godsnode>;
    fn add_leaf_node(&self, name: &String, raw: &Value) -> Rc<Godsnode>;
    fn update_index(&self, name: &String, index: u64);
    fn get_weak_node(&self, path: &String) -> Option<Weak<Godsnode>>;
}


impl StoreOps for Rc<Store> {
    fn new_node(&self) -> Rc<Godsnode> {
        let mut node = GodsnodeProto::new();
        let mut store = self.borrow_mut();
        let id = store.id;
        node.id = id;
        store.id += 1;
        let new_node = Rc::new(RefCell::new(node));
        store.store.insert(id, new_node.clone());
        new_node
    }
    fn add_node(&self, raw: &Value, name: String) -> Rc<Godsnode> {
        let node = self.new_node();
        {
            let mut state = node.borrow_mut();
            state.name = name;
            state.display_name = raw.get_str("display_name", "new node");
            state.node_type = GodsnodeType::Godsnode;
        }
        node
    }

    fn add_leaf_node(&self, name: &String, raw: &Value) -> Rc<Godsnode> {
        let node = self.add_node(raw, name.clone());
        {
            let mut state = node.borrow_mut();
            state.node_type = GodsnodeType::Leaf;
        }
        node
    }
    fn add_app_node(&self, raw: &Value) -> Rc<Godsnode> {
        let name = raw.get_str("name", "new_application");
        let node = self.add_node(raw, name);
        {
            let mut state = node.borrow_mut();
            state.node_type = GodsnodeType::Root;
        }
        node
    }

    fn update_index(&self, name: &String, index: u64) {
        let mut state = self.borrow_mut();
        state.index.insert(name.clone(), index);
    }

    fn get_weak_node(&self, path: &String) -> Option<Weak<Godsnode>> {
        let state = self.borrow();
        if let Some(id) = state.index.get(path) {
            if let Some(node) = state.store.get(&id) {
                return Some(Rc::downgrade(&node.clone()));
            }
        }
        None
    }
}

