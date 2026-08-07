import { register } from "./register_function.js";
import { login } from "./login_function.js";

const passwordInput = document.querySelector('.password-input');
const usernameInput = document.querySelector(".username-input");

async function sign_up() {

  const password = passwordInput.value;
  const username = usernameInput.value;

  if (password.length < 8) {
    alert('Password must be at least 8 characters long');
    return;
  } else if (password.length > 64) {
    alert("Password can't be longer than 64 charaters");
    return;
  }

  let register_data = await register(username, password);

  if (!register_data) {
    console.error("Invalid register_data!", register_data);
    return;
  }

  let login_data = await login(username, password);

  if (!login_data) {
    console.error("Invalid login_data!", login_data);
    return;
  }

  localStorage.setItem("auth_token", login_data.token_string);

  document.startViewTransition(() => {
    window.location.href = "chat.html";
  });
}

document.querySelector(".sign-up-button").addEventListener("click", sign_up)
