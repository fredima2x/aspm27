<script setup>
import { login } from "../../api/requests/login";
import { save_token } from "../../stores/token";
import { ref } from "vue";
import { useRouter } from "vue-router";
import loadingIcon from "../../assets/loading-spinner.svg"
import { onMounted } from "vue";

const router = useRouter();

const loadingStatus = ref(false)
const usernameInput = ref("");
const passwordInput = ref("");
const warning = ref('');

onMounted(() => {
  loadingStatus.value = false;
});

async function handle_login() {
  const password = passwordInput.value;
  const username = usernameInput.value;

  loadingStatus.value = true;

  if (!username || !password) {
    warning.value = 'Username and Password cannot be empty!';
    loadingStatus.value = false;
    return;
  }

  const data = await login(username, password);

  if (data.ok) {
    const res = await data.json()
    console.log("Saved Auth_token", res);
    save_token(res.token_string);
    loadingStatus.value = false;
    router.push("/chat");
  } else {
    loadingStatus.value = false;
    if (data.status === 400) {
      warning.value = 'Invalid Username or Password! (400)';
    } else if (data.status === 404) {
      warning.value = 'User does not exist! (404)';
    } else if (data.status === 401) {
      warning.value = 'Wrong Password! (401)';
    } else if (data.status === 500) {
      warning.value = 'Internal Server Error, Please try again later. (500)';
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
    <form class="login-page">
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

      <button class="sign-in-button" @click="handle_login">
        <template v-if="loadingStatus"><img :src="loadingIcon" class="loading-icon"></template>
        <template v-else>Sign in</template>
      </button>

      <p class="sign-up-text">
        Dont have an account yet?
        <a class="sign-up-link" @click="router.push('/register')">Sign up</a>
      </p>
    </form>
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
  animation: fadeIn 0.5s ease-in-out;
}

.heading {
  color: var(--theme-white);
  margin-top: 8px;
}

.sign-in-text {
  font-size: 14px;
}

.sign-in-button {
  height: 32px;
  width: 62px;
}

.sign-in-button:hover {
  opacity: 0.8;
}

.sign-in-button:active {
  opacity: 0.6;
}

.sign-up-text, .sign-up-link {
  color: var(--theme-light-gray);
}

.sign-up-link {
  cursor: pointer;
  text-decoration: underline;
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

.loading-icon {
  width: 20px;
  height: 20px;
  margin: 0;
  animation: drehen 1s linear infinite;
}

@keyframes drehen {
  to {
    transform: rotate(360deg);
  }
}
</style>
