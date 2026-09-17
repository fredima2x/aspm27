import { CACHE_PREFIX } from "./locations"

export function set_cache(key, value) {
    try {
        if (!localStorage.getItem(CACHE_PREFIX)) { 
            localStorage.setItem(CACHE_PREFIX, JSON.stringify({}));
        }

        const cache = JSON.parse(localStorage.getItem(CACHE_PREFIX));
        cache[key] = value;
        localStorage.setItem(CACHE_PREFIX, cache);
        
    } catch {
        console.error("Failed to Cache", [key, value]);
        return false
    }
}

export function get_cache(key) {
    try {
        const cache = JSON.parse(localStorage.getItem(CACHE_PREFIX));
        if (!cache[key]) { return false } else { return cache[key] }
    } catch {
        console.error("Failed to read Cache", key);
    }
}

export function clear_cache() {
    try {
        localStorage.removeItem(CACHE_PREFIX);
    } catch {
        console.error("Failed to delete Cache");
    }
}