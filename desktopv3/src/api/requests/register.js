import { apiFetch } from "../client";

export async function register(username, password) {
  return apiFetch("/users", {
    method: "POST",
    body: JSON.stringify({
      username,
      password,
    }),
  });
}
