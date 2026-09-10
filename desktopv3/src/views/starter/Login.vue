<script setup>
import { login } from '../../api/requests/login';
import { save_token } from '../../stores/token';
import { ref } from 'vue';

import { useRouter } from 'vue-router';
const router = useRouter();

const usernameInput = ref('');
const passwordInput = ref('');

async function handle_login() {
  const res = await login(
    usernameInput.value,
    passwordInput.value,
  );
  console.log("Saved Auth_token", res);
  save_token(res.token_string);

  router.push("/chat")
}

</script>

<template>
    <div class="app-background">
        <div class="login-page">
            <h2 class="heading">Welcome Back!</h2>
            <p class="sign-in-text">Sign in:</p>

            <input type="text" placeholder="Username or id" class="username-input" v-model="usernameInput" @keydown.enter="handle_login">
            <input type="password" placeholder="Password" class="password-input" v-model="passwordInput" @keydown.enter="handle_login">
            <button class="sign-in-button" @click="handle_login">Sign in</button>

            <p class="sign-up-text">
                Dont have an account yet?
                <a class="sign-up-link" @click="router.push('/register')">Sign up</a>
            </p>
        </div>
    </div>
</template>

<style scoped>
@import '/src/assets/styles/general.css';
@import '/src/assets/styles/utils/animations.css';

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
</style>
