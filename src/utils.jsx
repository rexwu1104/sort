import { invoke } from "@tauri-apps/api";

export async function set(key, value) {
    return await invoke(`set_${key}`, { [`${key}`]: value });
}

export async function get(key) {
    return await invoke(`get_${key}`, {});
}

export async function call(name) {
    return await invoke(name, {});
}