import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import Home from './components/Home.vue'
import Rays from './components/Rays.vue'
import Vectors from './components/Vectors.vue'
import Materials from './components/Materials.vue'
import Camera from './components/Camera.vue'
import Gallery from './components/Gallery.vue'
import './style.css'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Home },
    { path: '/rays', component: Rays },
    { path: '/vectors', component: Vectors },
    { path: '/materials', component: Materials },
    { path: '/camera', component: Camera },
    { path: '/gallery', component: Gallery },
  ],
  scrollBehavior() {
    return { top: 0 }
  }
})

createApp(App).use(router).mount('#app')
