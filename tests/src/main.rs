use std::rc::Rc;

fn main() {
    // First we initialize the instances of the canisters
    let ca = canister_a::api::LocalState::new();
    let cb = canister_b::api::LocalState::new();

    // Those calls also initialize their respective MODULE_STATE emulator hashmaps
    // inside so that later we can merge them into one big global state and call
    // the external canisters.
    ca.call("self", "show_meta");
    cb.call("self", "show_meta");

    // Since `MODULE_STATE` is defined per canister (crate), we need a way
    // to share them between each other
    let ca_emu_state = canister_a::emulator::MODULE_STATE.with(|s| Rc::clone(&s.state));
    let cb_emu_state = canister_b::emulator::MODULE_STATE.with(|s| Rc::clone(&s.state));
    types::merge_states(ca_emu_state, cb_emu_state);

    // After sharing state we can now call the external methods
    ca.call("canister-b", "show_meta");
    cb.call("canister-a", "show_meta");
}
