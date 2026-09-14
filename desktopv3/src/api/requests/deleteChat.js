import { apiFetch } from "../client.js";

export function delete_chat(chat) {
  const chatId = chat.chatId;
  return apiFetch(`/users/${chatId}`, {
    method: 'DELETE'
  });
}

