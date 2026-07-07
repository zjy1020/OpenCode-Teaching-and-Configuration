import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'welcome',
      component: () => import('../pages/WelcomePage.vue'),
    },
    {
      path: '/teaching/:slug?',
      name: 'teaching',
      component: () => import('../pages/TeachingPage.vue'),
    },
    {
      path: '/skills',
      name: 'skills',
      component: () => import('../pages/SkillsPage.vue'),
    },
    {
      path: '/commands',
      name: 'commands',
      component: () => import('../pages/CommandsPage.vue'),
    },
    {
      path: '/themes',
      name: 'themes',
      component: () => import('../pages/ThemesPage.vue'),
    },
  ],
})

export default router
