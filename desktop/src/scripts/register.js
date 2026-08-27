import { register } from "./register_function.js";
import { login } from "./login_function.js";
import { applyValidationMessage, getValidationErrors } from './validation.js';

const passwordInput = document.querySelector('.password-input');
const usernameInput = document.querySelector(".username-input");

async function signUp() {

  const password = passwordInput.value;
  const username = usernameInput.value;

  const passwordDescription = document.querySelector('.password-desc-text');
  const usernameDescription = document.querySelector('.username-desc-text');
  const validationErrors = getValidationErrors(password, username);

  applyValidationMessage(passwordDescription, validationErrors.password);
  applyValidationMessage(usernameDescription, validationErrors.username);

  if (validationErrors.password || validationErrors.username) {
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

document.querySelector(".sign-up-button").addEventListener("click", signUp);
