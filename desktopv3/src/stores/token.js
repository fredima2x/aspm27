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
    const auth_token = localStorage.getItem(TOKEN_STORAGE_KEY);
    console.debug("Getting auth_token from LS", auth_token);
    return auth_token;
  } catch {
    return false;
  }
}
