import { apiFetch } from "../client";

export function logout() {
    return apiFetch("/logout", {
        method: "POST",
        body: {
            logout_all_devices: false,
            send_new_token: false,
        },
    })
}

export function logout_all_devices(get_new_token) {
    return apiFetch("/logout", {
        method: "POST",
        body: {
            logout_all_devices: true,
            send_new_token: get_new_token,
        },
    })
}