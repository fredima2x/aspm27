import { CACHE_PREFIX } from "./locations"

export function set_cache(key, value) {
    try {
        localStorage.setItem(`${CACHE_PREFIX}_${key}`, JSON.stringify(value));
    } catch {
        console.error("Failed to Cache", [key, value]);
        return false
    }
}

export function get_cache(key) {
    try {
        const res = JSON.parse(localStorage.getItem(`${CACHE_PREFIX}_${key}`));
        if (!res) { return false } else { return res }
    } catch {
        console.error("Failed to read Cache", key);
    }
}