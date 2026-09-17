import { CACHE_PREFIX } from "./locations";

function load_cache() {
    const raw = localStorage.getItem(CACHE_PREFIX);

    if (!raw) return {};

    try {
        return JSON.parse(raw);
    } catch {
        localStorage.removeItem(CACHE_PREFIX);
        return {};
    }
}

export function set_cache(key, value) {
    const cache = load_cache();

    cache[key] = value;

    localStorage.setItem(CACHE_PREFIX, JSON.stringify(cache));
}

export function get_cache(key) {
    const cache = load_cache();

    return Object.hasOwn(cache, key) ? cache[key] : false;
}

export function clear_cache() {
    localStorage.removeItem(CACHE_PREFIX);
}