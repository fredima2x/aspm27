import { apiFetch, generateUuid } from "../client.js";

export async function deleteChat(chatId) {
  const uuid = typeof chatId === "string" && /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(chatId)
    ? chatId
    : generateUuid();

  const response = await apiFetch(`/chats/${uuid}`, {
    method: "DELETE"
  });

  if (response !== null) {
    removeFromLocalStorage(chatId);
  }

  return response;
}

export function removeChatFromLocalStorage(chatId) {
  const raw = localStorage.getItem("chats");
  const chats = raw ? JSON.parse(raw) : [];

  const updated = chats.filter(chat => chat.chatId !== chatId);

  localStorage.setItem("chats", JSON.stringify(updated));
}