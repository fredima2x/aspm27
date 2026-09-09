import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  { path: '/', name: 'Home', component: () => import('/src/views/Index.vue') },
  { path: '/login', name: 'Login', component: () => import('/src/views/starter/Login.vue') },
  { path: '/register', name: 'Register', component: () => import('/src/views/starter/Register.vue') },
  { path: '/chat', name: 'App', component: () => import('/src/views/chat/Chat.vue') },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
