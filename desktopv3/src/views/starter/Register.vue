<script setup>
import { useRouter } from 'vue-router';
import { ref } from 'vue';
const router = useRouter();

const showUsernameShort = ref(false);
const showPasswordShort = ref(false);
const showUsernameLong = ref(false);
const showPasswordLong = ref(false);

function reset_password_warn_messages() {
  showPasswordLong.value = false;
  showPasswordShort.value = false;
}
function reset_username_warn_messages() {
  showUsernameLong.value = false;
  showUsernameShort.value = false;
}

function check_password(password) {
  if (password.length < 8) { return -1 }
  if (password.length > 64) { return 1 }
  return 0
}
function check_username(username) {
  if (username.length < 3) { return -1 }
  if (username.length > 24) { return 1 }
  return 0;
}

function username_update(event) {
  const username = event.target.value;
  if (!username) { reset_username_warn_messages(); return }
  const username_status = check_username(username);
  reset_username_warn_messages();
  if (username_status === -1) { showUsernameShort.value = true }
  if (username_status === 1) { showUsernameLong.value = true }
}

function password_update(event) {
  const password = event.target.value;
  if (!password) { reset_password_warn_messages(); return }
  const password_status = check_password(password);
  reset_password_warn_messages();
  if (password_status === -1) { showPasswordShort.value = true }
  if (password_status === 1) { showPasswordLong.value = true }
}

</script>

<template>
    <div class="background">
        <div class="register-page">
            <h2 class="heading">Hello there!</h2>
            <p class="sign-up-text">Sign up:</p>

            <input type="text" placeholder="Username" class="username-input" @input="username_update">

            <Transition name="warning">
                <p v-if="showUsernameShort" class="password-warn-text">Username is too short!</p>
            </Transition>
            <Transition name="warning">
                <p v-if="showUsernameLong" class="password-warn-text">Username is too long!</p>
            </Transition>

            <input type="text" placeholder="Password" class="password-input" @input="password_update">

            <Transition name="warning">
                <p v-if="showPasswordShort" class="password-warn-text">Password is too short!</p>
            </Transition>
            <Transition name="warning">
                <p v-if="showPasswordLong" class="password-warn-text">Password is too long!</p>
            </Transition>

            <button class="sign-up-button">Sign up</button>

            <p class="sign-in-text">Already have an account?
                <a @click="router.push('/login')" class="sign-in-link">Sign in</a>
            </p>
        </div>
    </div>
</template>

<style scoped>
@import '/src/assets/styles/starters/general.css';
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
  color: white;
  margin-top: 8px;
}

.sign-up-text {
  font-size: 14px;
}

.password-warn-text {
  color: red;
}

.sign-up-button:hover {
  opacity: 0.8;
}

.sign-up-button:active {
  opacity: 0.6;
}

.sign-in-text {
  color: rgb(90, 90, 90);
}

.sign-in-link {
  cursor: pointer;
}
</style>
