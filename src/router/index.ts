import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('@/pages/HomePage.vue'),
    },
    {
      path: '/game/:id',
      name: 'game-detail',
      component: () => import('@/pages/GameDetailPage.vue'),
    },
    {
      path: '/game/:id/bepinex',
      name: 'bepinex-installer',
      component: () => import('@/pages/BepInExPage.vue'),
    },
    {
      path: '/game/:id/mod/:modId/config',
      name: 'mod-config',
      component: () => import('@/pages/ModConfigPage.vue'),
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('@/pages/SettingsPage.vue'),
    },
  ],
})

export default router
