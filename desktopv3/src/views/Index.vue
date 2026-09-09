<script setup>
import { useRouter } from 'vue-router'
const router = useRouter()

router.push('/login')
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
