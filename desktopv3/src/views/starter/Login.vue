<script setup>
import { login } from "../../api/requests/login";
import { save_token } from "../../stores/token";
import { ref } from "vue";

import { useRouter } from "vue-router";
const router = useRouter();

const usernameInput = ref("");
const passwordInput = ref("");
const warning = ref('');

async function handle_login() {
  const data = await login(usernameInput.value, passwordInput.value);
  if (data.ok) {
    const res = await data.json()
    console.log("Saved Auth_token", res);
    save_token(res.token_string);
    router.push("/chat");
  } else {
    if (data.status === 400) {
      warning.value = 'Invalid Username or Password!';
    } else if (data.status === 404) {
      warning.value = 'User does not exist!';
    } else if (data.status === 401) {
      warning.value = 'Wrong Password!';
    } else if (data.status === 500) {
      warning.value = 'Internal Server Error, Please try again later.';
    } else {
      warning.value = 'An unknown Error occured. View console for more Information!'
      console.error("Invalid Server Response", data);
    }
  }
}

function handleInput() {
  warning.value = '';
}
</script>

<template>
  <div class="app-background">
    <div class="login-page">
      <h2 class="heading">Welcome Back!</h2>
      <p class="sign-in-text">Sign in:</p>

      <input
        type="text"
        placeholder="Username or id"
        class="username-input"
        v-model="usernameInput"
        @keydown.enter="handle_login"
        @input="handleInput"
      />
      <input
        type="password"
        placeholder="Password"
        class="password-input"
        v-model="passwordInput"
        @keydown.enter="handle_login"
        @input="handleInput"
      />
      <Transition name="warning">
        <p v-show="warning" class="password-warn-text">{{ warning }}</p>
      </Transition>

      <button class="sign-in-button" @click="handle_login">Sign in</button>

      <p class="sign-up-text">
        Dont have an account yet?
        <a class="sign-up-link" @click="router.push('/register')">Sign up</a>
      </p>
    </div>
  </div>
</template>

<style scoped>
@import "/src/assets/styles/general.css";
@import "/src/assets/styles/utils/animations.css";

.login-page {
  background-color: var(--theme-gray);
  border-radius: var(--theme-round-edges);
  padding: 20px;
  box-shadow: 0px 0px 15px var(--theme-gray);
  width: min(100%, 360px);
  box-sizing: border-box;
  margin: 0;
  animation: fadeIn 0.5s ease-in-out forwards;
}

.heading {
  color: var(--theme-white);
  margin-top: 8px;
}

.sign-in-text {
  font-size: 14px;
}

.sign-in-button:hover {
  opacity: 0.8;
}

.sign-in-button:active {
  opacity: 0.6;
}

.sign-up-link {
  cursor: pointer;
}

.warning-enter-active,
.warning-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}

.warning-enter-from,
.warning-leave-to {
  opacity: 0;
  transform: translateY(-5px);
}

.warning-enter-to,
.warning-leave-from {
  opacity: 1;
  transform: translateY(0);
}

.password-warn-text {
  color: var(--theme-red);
}
</style>
