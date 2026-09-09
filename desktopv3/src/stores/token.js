import { TOKEN_STORAGE_KEY } from "./locations";

export function save_token(token) {
  try {
    localStorage.setItem(TOKEN_STORAGE_KEY, token);
    return true
  } catch {
    return false
  }
}

export function get_token() {
  try {
    return localStorage.getItem(TOKEN_STORAGE_KEY);
  } catch {
    return false;
  }
}
