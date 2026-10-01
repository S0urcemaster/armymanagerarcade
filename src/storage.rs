use crate::game::Game;

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn game_storage_len() -> usize;
    fn game_storage_read(ptr: *mut u8, capacity: usize) -> usize;
    fn game_storage_save(ptr: *const u8, len: usize);
    fn game_is_desktop() -> u32;
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn game_storage_crate_version() -> u32 {
    1
}

#[cfg(target_arch = "wasm32")]
pub fn load() -> Option<Game> {
    let len = unsafe { game_storage_len() };
    if len == 0 {
        return None;
    }
    let mut bytes = vec![0_u8; len];
    let read = unsafe { game_storage_read(bytes.as_mut_ptr(), bytes.len()) };
    bytes.truncate(read.min(bytes.len()));
    Game::decode(std::str::from_utf8(&bytes).ok()?)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load() -> Option<Game> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn save(game: &Game) {
    let value = game.encode();
    unsafe { game_storage_save(value.as_ptr(), value.len()) };
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save(_game: &Game) {}

#[cfg(target_arch = "wasm32")]
pub fn is_desktop() -> bool {
    unsafe { game_is_desktop() != 0 }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn is_desktop() -> bool {
    true
}
