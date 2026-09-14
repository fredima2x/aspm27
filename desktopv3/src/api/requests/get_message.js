import { apiFetch } from "../client";

export default function get_message(message_id) {
    return apiFetch(`/messages/${message_id}`, {method: "GET"});
}