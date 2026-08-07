import { baseURL } from "./config.js";

export async function login(username, password) {

  try {
    const headersObj = { "Content-Type": "application/json" };
    const response = await fetch(`${baseURL}/login`, {
      method: "POST",
      headers: headersObj,
      body: JSON.stringify({ username, password })
    });

    const contentType = response.headers.get("content-type") || "";
    let data;
    if (contentType.includes("application/json")) {
      data = await response.json();
    } else {
      data = await response.text();
    }

    if (!response.ok) {
      console.error("Login failed:", data);
      return null;
    }

    return data;

  } catch(error) {
    console.error(`Fetch error: ${error}`);
    return null;
  }
}
