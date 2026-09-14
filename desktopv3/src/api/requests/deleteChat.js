import { apiFetch } from "../client.js";

export function deleteChat(chat) {
  const chatId = chat.chatId;
  return apiFetch(`/users/${chatId}`, {
    method: 'DELETE'
  });
}

