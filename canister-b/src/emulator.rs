use types::*;

thread_local! {
  pub static MODULE_STATE: ModuleState = ModuleState::new();
}
