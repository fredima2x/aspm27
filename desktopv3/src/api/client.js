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

    if (!response.ok) {
      const text = await response.text();
      throw new Error(`API request failed (${response.status}): ${text}`);
    }

    const contentType = response.headers.get("content-type") || "";
    if (contentType.includes("application/json")) {
      return await response.json();
    }

    return await response.text();
  } catch (error) {
    console.error("Network error:", error);
    return null;
  }
}