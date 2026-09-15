import { apiFetch } from "../client";

export function update_chat(chatId, chatName, chatDesc) {
  return apiFetch(`/chats/${chatId}`, {
    method: "PUT",
    body: JSON.stringify({
      chat_name: chatName,
      chat_desc: chatDesc,
    }),
  });
}