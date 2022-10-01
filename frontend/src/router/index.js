import Vue from 'vue'
import VueRouter from 'vue-router'
import LandingPage from '../views/LandingPageView.vue'
import BattleMode from '../views/BattleModeView.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/', component: LandingPage,
  },
  {
  path: '/battle', component: BattleMode,
  },
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

export default router
