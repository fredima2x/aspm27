import { login } from "./loginFunction.js";

const passwordInput = document.querySelector('.password-input');
const usernameInput = document.querySelector(".username-input");

document.querySelector(".js-sign-in-button").addEventListener("click", signIn);

async function signIn() {
  console.log("Sign In was Attempted");


  if (passwordInput.value.length < 8) {
    alert('Password must be at least 8 characters long');
    return;
  }
  if (passwordInput.value.length > 64) {
    alert("Password can't be longer than 64 charaters");
    return;
  }

  console.log("Logging in!", usernameInput.value, passwordInput.value);
  const login_data = await login(usernameInput.value, passwordInput.value);

  if (!login_data) {
    console.log("Invalid Login Data");
    return;
  }

  localStorage.setItem("auth_token", login_data.token_string);



  document.startViewTransition(() => {
    window.location.href = "/chat.html";
  });
}
