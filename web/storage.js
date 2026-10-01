miniquad_add_plugin({
  name: "game_storage",
  version: 1,
  register_plugin(imports) {
    const key = "army-manager-arcade.save.v1";
    const bytes = () => new TextEncoder().encode(localStorage.getItem(key) || "");

    imports.env.game_storage_len = () => bytes().length;
    imports.env.game_storage_read = (ptr, capacity) => {
      const value = bytes();
      const length = Math.min(value.length, capacity);
      new Uint8Array(wasm_memory.buffer, ptr, length).set(value.subarray(0, length));
      return length;
    };
    imports.env.game_storage_save = (ptr, length) => {
      localStorage.setItem(key, UTF8ToString(ptr, length));
    };
    imports.env.game_is_desktop = () => window.matchMedia("(pointer: fine)").matches ? 1 : 0;
  },
});
