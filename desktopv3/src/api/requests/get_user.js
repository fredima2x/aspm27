import { apiFetch } from "../client";

export function get_user(identifier) {
    function get_path(identifier) {
        if (typeof identifier == "string") {
            return `/users/name/${identifier}`
        } else if (typeof identifier == "bigint") {
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