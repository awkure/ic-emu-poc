use std::cell::RefCell;
use std::rc::Rc;

use crate::emulator::MODULE_STATE;
use types::*;

static CANISTER_NAME: &str = "canister-b";

/// The local state of the canister
#[derive(Debug)]
#[allow(dead_code)]
struct CanisterState {
    b: i32,
}

/// This is a wrapper over `Rc<RefCell<CanisterState>>` because
/// rust has a limitation on impls for traits that are not in
/// the current crate for arbitrary types
#[derive(Clone)]
pub struct LocalState(Rc<RefCell<CanisterState>>);

impl LocalState {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(CanisterState { b: 1 })))
    }

    pub fn call(&self, canister: &str, method: &str) {
        self.0.borrow().call(canister, method)
    }
}

/// This is a state canister works with that also has a pointer to
/// the emulated global state for allowing inter canister calls.
struct StateMeta {
    state: LocalState,
    global: StateHashmap,
}

impl StateMeta {
    pub fn new(global: StateHashmap) -> Self {
        let state_local = LocalState::new();
        MODULE_STATE.with(|s| {
            let mut hm = s.state.borrow_mut();
            hm.insert(CANISTER_NAME.to_string(), state_local.0.clone());
        });
        Self {
            state: state_local,
            global,
        }
    }
}

thread_local! {
  /// The only problem with this static is that it's not initialized until one of the api
  /// methods use it in one way or the other, meaning that the canister does not see the
  /// others until the other canisters call some internal methods inside. We could wrap
  /// this up in `init` method.
  static STATE: StateMeta = StateMeta::new(MODULE_STATE.with(|s| Rc::clone(&s.state)));
}

impl State for CanisterState {
    fn call(&self, canister: &str, method: &str) {
        match canister {
            "self" => {
                println!("calling internal method: {}", method);
                dispatch(method);
            }
            name if name == CANISTER_NAME => {
                println!("calling internal method: {}", name);
                dispatch(method);
            }
            external => {
                println!("calling external canister: {}", external);
                MODULE_STATE.with(|s| match s.state.borrow().get(canister) {
                    Some(state) => state.borrow().call(canister, method),
                    None => panic!("canister not found: {}", canister),
                });
            }
        }
    }
}

/// A local dispatch for the api methods. This can be extended to
/// allow functions of different variadicity later by accepting a
/// serialized vec of arguments.
fn dispatch(s: &str) {
    match s {
        "show_meta" => show_meta(),
        "call_a" => call_a(),
        _ => panic!("no such method"),
    }
}

// ----------------------------------------------------------------
// api

pub fn show_meta() {
    let mut keys = Vec::<String>::new();
    STATE.with(|s| keys = s.global.borrow().keys().map(|k| k.clone()).collect());

    println!("in scope: {:?}", keys);
    println!("{:#?}", STATE.with(|s| Rc::clone(&s.state.0)));
}

pub fn call_a() {
    println!("calling canister-a from {}", CANISTER_NAME);
    STATE.with(|s| s.state.0.borrow().call("canister-a", "show_meta"));
}
