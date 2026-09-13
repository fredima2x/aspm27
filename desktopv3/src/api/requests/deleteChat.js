import { apiFetch, generateUuid } from "../client.js";

export async function deleteChat(chatId) {
  const response = await apiFetch(`/chats/${chatId}`, {
    method: "DELETE"
  });

  return response;
}
