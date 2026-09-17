import { logout, logout_all_devices } from "../api/requests/logout";
import { clear_cache } from "../stores/cache";
import { clear_token, save_token } from "../stores/token";

export async function full_logout() {
    try {
        await logout();
    } catch {
        console.log("Failed to Logout!, deleting Cache Anyway...  Bye Bye");
    } finally {
        clear_cache();
    }
}

export async function logout_all(except_this_one) {
    try {
        if (except_this_one === false) {
            await logout_all_devices(false);
            clear_token();
        } else {
            const res = await logout_all_devices(true);
            save_token(res.body.auth_token);
        }
    } catch {
        console.log("Failed Proper Logout!, deleting Everything Anyway...     Sike");
    } finally {
        clear_cache();
    }
}