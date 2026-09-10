import { apiFetch } from "../client";

export function get_chats() {
  return apiFetch('/chats', {
    method: 'GET'
  });
}
