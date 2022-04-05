use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait State {
    fn call(&self, canister: &str, method: &str);
}

// pub type StateHashmap = Rc<RefCell<HashMap<String, Box<dyn State>>>>;
pub type StateHashmap = Rc<RefCell<HashMap<String, Rc<RefCell<dyn State>>>>>;

pub struct ModuleState {
    pub state: StateHashmap,
}

impl ModuleState {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

// TODO: Hashmap::extend() requires an iter for pointer of Rc ref,
// this should be a wrapper I guess with its own respectful Iter impl
pub fn merge_states(s1: StateHashmap, s2: StateHashmap) {
    let mut s1 = s1.borrow_mut();
    let mut s2 = s2.borrow_mut();

    for (k, v) in s2.iter() {
        s1.insert(k.clone(), v.clone());
    }
    for (k, v) in s1.iter() {
        s2.insert(k.clone(), v.clone());
    }
}
