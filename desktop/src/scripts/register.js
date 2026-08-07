import { register } from "./register_function.js";
import { login } from "./login_function.js";
import { bannedPasswords } from './config.js';

const passwordInput = document.querySelector('.password-input');
const usernameInput = document.querySelector(".username-input");

async function sign_up() {

  const password = passwordInput.value;
  const username = usernameInput.value;
  const passwordDescription = document.querySelector('.password-desc-text');

  if (password.length < 8) {

    passwordDescription.innerText = 'Password must be at least 8 characters long!';
    passwordDescription.classList.add('password-warn-text');
    return;
  } else if (password.length > 64) {

    passwordDescription.innerText = "Password can't be longer than 64 characters!";
    passwordDescription.classList.add('password-warn-text');
    return;
  } else if (bannedPasswords.includes(password)) {
    passwordDescription.innerText = "Don't even try...";
    passwordDescription.classList.add('password-warn-text');
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
