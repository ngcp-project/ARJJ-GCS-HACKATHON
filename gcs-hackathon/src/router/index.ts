import { createRouter, createWebHistory } from 'vue-router'
import CatalogView from '../views/CatalogView.vue'
import LoginView from '../views/LoginView.vue'
import BarcodesView from '../views/BarcodesView.vue'


const routes = [
  { path: '/', name: 'Catalog', component: CatalogView }, // ✅ default route
  { path: '/login', name: 'Login', component: LoginView },
  { path: '/barcodes', name: 'barcodes', component: BarcodesView },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
