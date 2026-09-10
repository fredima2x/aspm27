import { BASE_SERVER_URL } from "../config/config"
import { get_token } from "../stores/token"


export async function apiFetch(endpoint, options = {}) {
  const res = await fetch(`${BASE_SERVER_URL}${endpoint}`, {
    ...options,
    headers: get_header()
  })

  if (!res.ok) {
    const error = await res.json().catch(() => ({ message: 'Unbekannter Fehler' }))
    throw new Error(error.message || `HTTP ${res.status}`)
  }

  return res.json()
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
