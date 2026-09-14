import { apiFetch } from "../client";

export default function send_message(chat_id, content) {
    return apiFetch(`/chats/${chat_id}/messages`, {
        method: "POST", 
        body: JSON.stringify({ content })
    });
}