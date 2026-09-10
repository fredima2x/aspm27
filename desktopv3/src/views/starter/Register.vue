<script setup>
import { login } from '../../api/requests/login';
import { register } from '../../api/requests/register';
import { save_token } from '../../stores/token';

import { useRouter } from 'vue-router';
import { ref } from 'vue';
const router = useRouter();

const usernameWarning = ref('');
const passwordWarning = ref('');
const warning = ref('');

const usernameInput = ref('');
const passwordInput = ref('');

function check_password(password) {
  if (password.length < 8) { return -1 }
  if (password.length > 64) { return 1 }
  return 0
}
function check_username(username) {
  if (username.length < 3) { return -1 }
  if (username.length > 24) { return 1 }
  return 0
}

function username_update(event) {
  warning.value = '';
  const username = event.target.value;
  if (!username) { usernameWarning.value = ''; return }
  const username_status = check_username(username);

  if (username_status === -1) {
    usernameWarning.value = 'Username is too short!';
  }
  if (username_status === 1) {
    usernameWarning.value = 'Username is too long!';
  }
  if (username_status === 0) {
    usernameWarning.value = '';
  }
}

function password_update(event) {
  warning.value = '';
  const password = event.target.value;
  if (!password) { passwordWarning.value = ''; return }
  const password_status = check_password(password);

  if (password_status === -1) {
    passwordWarning.value = 'Password is too short!';
  }
  if (password_status === 1) {
    passwordWarning.value = 'Password is too long!';
  }
  if (password_status === 0) {
    passwordWarning.value = '';
  }
}

async function sign_up_handler() {
  const username = usernameInput.value;
  const password = passwordInput.value;

  const data = await register(username, password);
  if (data.ok) {
    const res = await data.json();
    save_token(res.auth_token);
    router.push("/chat");
  } else {
    if (data.status === 400) {
      warning.value = 'Invalid Username or Password!';
    } else if (data.status === 409) {
      warning.value = 'User already Exists!';
    } else if (data.status === 500) {
      warning.value = 'Internal Server Error, Please try again later.';
    } else {
      warning.value = 'An unknown Error occured. View console for more Information!'
      console.error("Invalid Server Response", data);
    }
  }
}

</script>

<template>
    <div class="app-background">
        <div class="register-page">
            <h2 class="heading">Hello there!</h2>
            <p class="sign-up-text">Sign up:</p>

            <input type="text" placeholder="Username" class="username-input" @input="username_update" @keydown.enter="sign_up_handler" v-model="usernameInput">
            <Transition name="warning">
                <p v-show="usernameWarning" class="password-warn-text">{{ usernameWarning }}</p>
            </Transition>

            <input type="password" placeholder="Password" class="password-input" @input="password_update" @keydown.enter="sign_up_handler" v-model="passwordInput">
            <Transition name="warning">
                <p v-show="passwordWarning" class="password-warn-text">{{ passwordWarning }}</p>
            </Transition>
            <Transition name="warning">
                <p v-show="warning" class="password-warn-text">{{ warning }}</p>
            </Transition>

            <button class="sign-up-button" @click="sign_up_handler">Sign up</button>

            <p class="sign-in-text">Already have an account?
                <a @click="router.push('/login')" class="sign-in-link">Sign in</a>
            </p>
        </div>
    </div>
</template>

<style scoped>
@import '/src/assets/styles/general.css';
@import '/src/assets/styles/utils/animations.css';


.register-page {
  background-color: rgb(15, 15, 15);
  border-radius: 10px;
  padding: 20px;
  box-shadow: 0px 0px 15px rgb(15, 15, 15);
  width: min(100%, 360px);
  box-sizing: border-box;
  margin: 0;
  animation: fadeIn 0.5s ease-in-out forwards;
}

.heading {
  color: var(--theme-white);
  margin-top: 8px;
}

.sign-up-text {
  font-size: 14px;
}

.password-warn-text {
  color: var(--theme-red);
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

.sign-up-button:hover {
  opacity: 0.8;
}

.sign-up-button:active {
  opacity: 0.6;
}

.sign-in-text {
  color: var(--theme-light-gray);
}

.sign-in-link {
  cursor: pointer;
}
</style>
