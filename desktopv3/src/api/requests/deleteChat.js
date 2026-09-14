import { apiFetch } from "../client.js";

export function delete_chat(chatId) {
  return apiFetch(`/chats/${chatId}`, {
    method: 'DELETE'
  });
}

