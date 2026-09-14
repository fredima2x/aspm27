import { apiFetch } from "../client";

export function create_chat(chatName, chatDesc) {
  return apiFetch('/chats', {
    method: 'POST',
    body: JSON.stringify({ chat_name: chatName, chat_desc: chatDesc })
  });
}