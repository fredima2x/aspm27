<script setup>
import { onMounted } from "vue";
import { useRouter } from "vue-router";
import { get_token } from "../stores/token.js";
import get_chats from "../api/requests/get_chats.js";
const router = useRouter();

onMounted(async () => {
  const token = get_token();

  if (!token) {
    router.push("/login");
    return;
  }

  const data = await get_chats();

  if (data?.raw?.ok) {
    router.push("/chat");
  } else if (data?.raw?.status == 401) {
    router.push("/login");
  }
});
</script>

<template>
  <div class="background"></div>
</template>
<style scoped>
.background {
  width: 100%;
  height: 100%;
  background-color: #000000;
}
</style>
