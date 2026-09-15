import { apiFetch } from "../client";

export default function get_profile() {
    return apiFetch("/profile", {
        method: "GET"
    });
}