<template>
    <div class="page-background">
        <div class="register-page">
            <h2 class="heading">Hello there!</h2>
            <p class="sign-up-text">Sign up:</p>
            <input type="text" placeholder="Username" class="username-input">
            <input type="text" placeholder="Password" class="password-input">
            <button class="sign-up-button" onclick="">Sign up</button>
            <p class="sign-in-text">Already have an account?
              <router-link to="/login">Sign in</router-link>
            </p>
        </div>
    </div>
</template>

<style scoped>
.page-background {
  min-height: 100vh;
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: black;
  background-image: linear-gradient(to left, rgb(15, 15, 15), rgba(0, 219, 124, 0.25));
}
p,
h2,
a {
  font-family: Arial;
  color: rgb(90, 90, 90);
}
p {
  font-size: 12px;
}
input {
  width: 100%;
  box-sizing: border-box;
  border: solid 3px rgb(0, 219, 124);
  border-radius: 10px;
  background-color: rgb(27, 27, 27);
  background-image: linear-gradient(to right, rgb(15, 15, 15), rgba(0, 219, 124, 0.25));
  margin-bottom: 10px;
  padding: 8px;
  color: white;
}
input:focus {
  outline: none;
}
button {
  border: none;
  border-radius: 10px;
  background-color: rgb(0, 219, 124);
  padding: 8px;
  color: rgb(27, 27, 27);
  margin-bottom: 15px;
  transition: opacity 0.135s;
}
.register-page {
  background-color: rgb(15, 15, 15);
  border-radius: 10px;
  padding: 20px;
  box-shadow: 0px 0px 15px rgb(15, 15, 15);
  width: min(100%, 360px);
  box-sizing: border-box;
}
.heading {
  color: white;
  margin-top: 8px;
}
.sign-up-text {
  font-size: 14px;
}
.sign-up-button:hover {
  opacity: 0.8;
}
.sign-up-button:active {
  opacity: 0.6;
}
</style>
