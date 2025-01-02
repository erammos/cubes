use crate::prelude::*;
use glam::Mat4;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
type LinkRef = Rc<RefCell<GameObjectNode>>;
pub struct GameObject(LinkRef);
impl GameObject {
    pub fn add_behavior(&mut self, behavior: MonoBehavior) {
        self.0.borrow_mut().behavior.push(behavior);
    }
    pub fn set_local_transform(&self, transform: Mat4) {
        *self.0.borrow().local_transform.borrow_mut() = transform;
    }
    pub fn get_world_transform(&self) -> Mat4 {
        *self.0.borrow().world_transform.borrow()
    }
    pub fn get_local_transform(&self) -> Mat4 {
        *self.0.borrow().local_transform.borrow()
    }
    pub fn get_name(&self) -> String {
        self.0.borrow().name.clone()
    }
}
type WeakLink = Weak<RefCell<GameObjectNode>>;
pub type MonoBehavior = Rc<RefCell<dyn Behavior>>;
pub trait Behavior {
    fn awake(&mut self, renderer: &Graphics);
    fn update(&mut self, this: GameObject, delta_time: f32);
}
pub type Transform = RefCell<Mat4>;

pub struct GameObjectNode {
    name: String,
    mesh_id: Option<MeshId>,
    local_transform: Transform,
    world_transform: Transform,
    behavior: Vec<MonoBehavior>,
    parent: Option<WeakLink>,
    children: Vec<LinkRef>,
}
impl GameObjectNode {
    pub fn new(name: &str) -> GameObject {
        GameObject(Rc::new(RefCell::new(Self {
            behavior: vec![],
            parent: None,
            world_transform: RefCell::new(Mat4::IDENTITY),
            local_transform: RefCell::new(Mat4::IDENTITY),
            children: vec![],
            mesh_id: None,
            name: name.to_string(),
        })))
    }

    pub fn add_child(root: &GameObject, child: &GameObject) {
        root.0.borrow_mut().children.push(Rc::clone(&child.0));
        child.0.borrow_mut().parent = Some(Rc::downgrade(&root.0));
    }
    #[allow(dead_code)]
    pub fn remove_child(root: &GameObject, child: &GameObject) {
        root.0
            .borrow_mut()
            .children
            .retain(|x| !Rc::ptr_eq(x, &child.0));
        child.0.borrow_mut().parent = None;
    }
    pub fn init(root: &GameObject, renderer: &Graphics) {
        let mut stack = vec![Rc::clone(&root.0)];
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            let node = node.borrow();

            for behavour in &node.behavior {
                behavour.borrow_mut().awake(renderer);
            }
            for child in &node.children {
                stack.push(Rc::clone(child));
            }
        }
    }

    pub fn update(root: &GameObject, delta_time: f32) {
        let mut stack = vec![Rc::clone(&root.0)];
        while !stack.is_empty() {
            let node_rc = stack.pop().unwrap();
            let node = node_rc.borrow();

            for behavior in &node.behavior {
                behavior
                    .borrow_mut()
                    .update(GameObject(Rc::clone(&node_rc)), delta_time);
            }
            if node.parent.is_some() {
                let parent = node.parent.as_ref().unwrap().upgrade().unwrap();
                *node.world_transform.borrow_mut() =
                    *parent.borrow().world_transform.borrow() * *node.local_transform.borrow();
            }

            for child in &node.children {
                stack.push(Rc::clone(child));
            }
        }
    }
}
