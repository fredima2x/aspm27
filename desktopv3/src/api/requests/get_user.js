import { apiFetch } from "../client";

export function get_user(identifier) {
    function get_path(identifier) {
        if (identifier.length < 32) {
            return `/users/name/${identifier}`
        } else {
            return `/users/id/${identifier}`
        }
    }

    return apiFetch(
        get_path(identifier),
        {
            method: "GET",
        }
    )
}