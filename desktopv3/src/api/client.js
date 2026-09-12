import { BASE_SERVER_URL } from "../config/config"
import { get_token } from "../stores/token"

export async function apiFetch(endpoint, options = {}) {
  const token = get_token();
  const requestHeaders = {
    "Content-Type": "application/json",
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
    ...(options.headers || {})
  };

  try {
    const response = await fetch(`${BASE_SERVER_URL}${endpoint}`, {
      ...options,
      headers: requestHeaders
    });


    const contentType = response.headers.get("content-type") || "";
    if (contentType.includes("application/json")) {
      return {
        raw: response,
        body: await response.json(),
      };
    } else {
      return {
        raw: response,
        body: {
          "non_json": true
        },
      }
    }

  } catch (error) {
    console.error("Network error:", error);
    return null;
  }
}

// Testing
export function generateUuid() {
  if (globalThis.crypto && typeof globalThis.crypto.randomUUID === "function") {
    return globalThis.crypto.randomUUID();
  }

  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, (char) => {
    const value = Math.floor(Math.random() * 16);
    const replacement = char === "x" ? value : (value & 0x3) | 0x8;
    return replacement.toString(16);
  });
}