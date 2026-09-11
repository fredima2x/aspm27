import { apiFetch } from "../client";

export function get_user(identifier) {
    function get_path(identifier) {
        if (typeof identifier == "string") {
            return `/user/name/${identifier}`
        } else if (typeof identifier == "bigint") {
            return `/user/id/${identifier}`
        }
    }

    return apiFetch(
        get_path(identifier),
        {
            method: "GET",
        }
    )
}