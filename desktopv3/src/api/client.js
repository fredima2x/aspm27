import { BASE_SERVER_URL } from "../config/config"
import { get_token } from "../stores/token"


export async function apiFetch(endpoint, options = {}) {
  let res;

  try {
    res = await fetch(`${BASE_SERVER_URL}${endpoint}`, {
      ...options,
      headers: get_header()
    });
  } catch (error) {
    console.error("Network error:", error);
    return null;
  }

  return res;
}

function get_header() {
  const token = get_token();
  if (!token) {
    console.warn("Header was Created with no Token!");
    return {
      'Content-Type': 'application/json',
    }
  } else {
    console.debug("Header was Created with a Token!", token);
    return {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    }
  }
}
