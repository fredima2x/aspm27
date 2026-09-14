import { apiFetch } from "../client";

export default function get_messages(chat_id, limit, offset) {
    return apiFetch(`/chats/${chat_id}/get_messages`, {
        method: "POST",
        body: JSON.stringify({
            limit, offset
        })
    });
}