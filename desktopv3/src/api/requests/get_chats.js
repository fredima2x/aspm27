import { apiFetch } from "../client";

export default function get_chats() {
  return apiFetch('/chats', {
    method: 'GET'
  });
}
