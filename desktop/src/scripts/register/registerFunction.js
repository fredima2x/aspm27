import { baseURL } from "../config.js";

export async function register(username, password) {
  try {
    const response = await fetch(`${baseURL}/users`, {
      method: 'POST',
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password })
    });
    return await response.json();
  } catch(error) {
    console.error(`Fetch error: ${error}`);
  }
}
