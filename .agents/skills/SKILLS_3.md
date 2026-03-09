
---
name: NelApp_Tauri_threadsafe_state
description: use this skill when coding Rust in a Tauri backend for a NeverEngineLabs app to manage state safely
---
1. Make a new folder `src-tauri/src/globals`
2. Create a new file `statics.rs`
3. Create a new file `app_state.rs`
4. Add the following boilerplate code to Cargo.toml:
```toml
[dependencies]
once_cell = '1.21.3'
```
5. Add the following boilerplate code to statics.rs:
```rust
use crate::globals::app_state::NelAppState;
use crate::APP_HANDLE;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
pub static APP_HANDLE: OnceCell<AppHandle> = OnceCell::new();
pub fn init_app_handle(handle: AppHandle) {
    APP_HANDLE
        .set(handle)
        .expect("AppHandle already initialized");
}
pub fn app_handle() -> &'static AppHandle {
    APP_HANDLE.get().expect("APP_HANDLE not initialized")
}
pub fn with_mut_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut NelAppState) -> R,
{
    let app_handle = app_handle();
    let state = app_handle.state::<Mutex<NelAppState>>();
    let mut locked_state = state.lock().unwrap();
    let result = f(&mut locked_state);
    drop(locked_state); // Explicit drop for clarity
    result
}
pub fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&NelAppState) -> R,
{
    let app_handle = app_handle();
    let state = app_handle.state::<Mutex<NelAppState>>();
    let locked_state = state.lock().unwrap();
    f(&locked_state)
}
```
6. Add the following boilerplate code to app_state.rs:
```rust
pub struct NelAppState {   // add your pub state members here}
impl Default for NelAppState {     fn default() -> Self {
         NelAppState {          // initialize your state members here        }    }}
```
7. In `src-tauri/src/lib.rs` includen the following setup step:
```rust  
    let tauri_builder = tauri::Builder::default();
   tauri_builder.setup(|app| {   app.manage(StdMutex::new(NelAppState::default()));   init_app_handle(app.handle().clone());   Ok(())   })
```
8. Throughout the code, use `with_mut_state` and `with_state` to access the state

```rust
with_mut_state(|st8| {
    st8.some_member = 123;
});
``` 

```rust
with_state(|st8| {
    st8.some_member
}
```

9. Throughout the code, use `app_handle()` to get the AppHandle
10. This replaces the magic, but slightly difficult to read Tauri official `state` API:
```
 app: tauri::AppHandle,
   state: tauri::State<'_, std::sync::Mutex<NelAppState>>
```
11. You can now safely access the state AND get an app_handle from any thread.