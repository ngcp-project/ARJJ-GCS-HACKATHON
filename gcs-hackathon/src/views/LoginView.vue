<template>
  <div class="min-h-screen flex flex-col bg-gray-50 font-sans text-gray-800">
    <header class="bg-green-800 text-white shadow-lg">
      <div class="max-w-7xl mx-auto flex justify-between items-center px-6 py-4">
        <div class="flex items-center gap-3">
          <img
            src="https://asiportal.cpp.edu/Content/Images/calpolypomona-logo-main.png"
            alt="ASI Logo"
            class="h-10"
          />
          <h1 class="text-2xl font-bold tracking-wide">ASI Portal</h1>
        </div>

        <div class="flex items-center gap-4 relative">
          <div v-if="auth.loggedIn" class="text-white font-medium mr-4">
            {{ auth.username }}
          </div>
          <div class="relative" ref="menuButtonRef">
            <button
              @click="toggleMenu"
              class="bg-white text-green-800 px-4 py-2 rounded-md font-semibold hover:bg-gray-200 transition flex items-center gap-2"
            >
              Menu
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-4 w-4"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </button>

            <div
              v-if="isMenuOpen"
              class="absolute right-0 mt-2 w-40 bg-white text-green-800 rounded-md shadow-lg z-50"
            >
              <button
                @click="goToBarcodes"
                class="block w-full text-left px-4 py-2 hover:bg-gray-100"
              >
                Barcodes
              </button>
              <button
                @click="goToPrograms"
                class="block w-full text-left px-4 py-2 hover:bg-gray-100"
              >
                Programs
              </button>
              <button
                @click="goToLogin"
                class="block w-full text-left px-4 py-2 hover:bg-gray-100"
              >
                Login
              </button>
            </div>
          </div>
        </div>
      </div>
    </header>

    <main class="flex-1 w-full flex items-center justify-center p-6">
      <div v-if="auth.loggedIn" class="bg-white p-8 rounded-xl shadow-md w-full max-w-md text-center">
        <h2 class="text-2xl font-bold mb-6 text-gray-800">You are signed in as {{ auth.username }}</h2>
        <button @click="handleSignOut" class="w-full bg-red-600 text-white py-2 rounded-md hover:bg-red-700 transition font-semibold">
          Sign Out
        </button>
      </div>
      <div v-else class="bg-white p-8 rounded-xl shadow-md w-full max-w-md">
        <h2 class="text-2xl font-bold mb-6 text-center text-gray-800">
          Sign in to your ASI Portal
        </h2>
        <form @submit.prevent="handleLogin">
          <div class="mb-4">
            <label for="username" class="block text-sm font-medium text-gray-700 mb-1">Username</label>
            <input
              id="username"
              type="text"
              v-model="form.username"
              placeholder="Enter your username"
              class="w-full p-2 border border-gray-300 rounded-md focus:ring-green-500 focus:border-green-500"
            />
          </div>
          <div class="mb-6">
            <label for="password" class="block text-sm font-medium text-gray-700 mb-1">Password</label>
            <input
              id="password"
              type="password"
              v-model="form.password"
              placeholder="Enter your password"
              class="w-full p-2 border border-gray-300 rounded-md focus:ring-green-500 focus:border-green-500"
            />
          </div>
          <button
            type="submit"
            class="w-full bg-green-700 text-white py-2 rounded-md hover:bg-green-800 transition font-semibold"
          >
            Log In
          </button>
        </form>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const auth = useAuthStore()

const router = useRouter()
const isMenuOpen = ref(false)
const menuButtonRef = ref(null)

// --- Functions for the Header ---
const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value
}

const goToLogin = () => {
  if (auth.loggedIn) {
    // Sign out
    auth.clearAuth()
    router.push('/')
  } else {
    router.push('/login')
  }
  isMenuOpen.value = false
}

const goToBarcodes = () => {
  router.push('/barcodes')
  isMenuOpen.value = false
}

const goToPrograms = () => {
  window.location.href = 'http://localhost:5173/'
  isMenuOpen.value = false
}

const goToCatalog = () => {
  router.push('/')
  isMenuOpen.value = false
}

const handleClickOutside = (event) => {
  if (menuButtonRef.value && !menuButtonRef.value.contains(event.target)) {
    isMenuOpen.value = false
  }
}

const form = ref({ username: '', password: '' })

const handleLogin = async () => {
  try {
    const res = await fetch('http://127.0.0.1:3000/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username: form.value.username, password: form.value.password }),
    })
    const data = await res.json()
    if (data && data.barcode) {
      auth.setAuth(form.value.username, data.barcode)
      router.push('/barcodes')
    } else {
      alert('Login failed')
    }
  } catch (error) {
    console.error('An error occurred:', error)
  }
}

const handleSignOut = () => {
  auth.clearAuth()
  router.push({ name: 'login' })
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>