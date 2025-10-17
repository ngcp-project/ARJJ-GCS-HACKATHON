<template>
  <div class="min-h-screen flex flex-col bg-gray-50 font-sans text-gray-800">
    <!-- Header -->
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
          <!-- Dropdown Menu -->
          <div class="relative">
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
                @click="goToCatalog"
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

    <!-- Hero Section -->
    <section class="bg-gradient-to-r from-green-800 to-green-600 text-white py-16 shadow-inner">
      <div class="max-w-7xl mx-auto px-6 text-center">
        <h2 class="text-4xl font-bold mb-3">Programs Catalog</h2>
        <p class="text-lg text-green-100 max-w-2xl mx-auto">
          Explore all ASI programs and activities offered by Cal Poly Pomona’s
          Associated Students, Inc.
        </p>
      </div>
    </section>

    <!-- Main Content -->
    <main class="flex-1 max-w-7xl mx-auto w-full px-6 py-12">
      <div
        class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8"
      >
        <div
          v-for="program in programs"
          :key="program.id"
          class="bg-white rounded-xl shadow-md hover:shadow-2xl transition transform hover:-translate-y-1"
        >
          <img
            :src="program.image_url"
            alt="Program image"
            class="h-48 w-full object-cover rounded-t-xl"
          />
          <div class="p-5">
            <h3 class="text-lg font-semibold mb-2 text-gray-900">
              {{ program.name }}
            </h3>
            <p class="text-gray-600 text-sm mb-4 line-clamp-2">
              {{ program.description }}
            </p>
          </div>
        </div>
      </div>
    </main>

    
  </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const isMenuOpen = ref(false)

function toggleMenu() {
  isMenuOpen.value = !isMenuOpen.value
}

function goToLogin() {
  router.push('/login')
  isMenuOpen.value = false
}

function goToBarcodes() {
  router.push('/barcodes')
  isMenuOpen.value = false
}

function goToCatalog() {
  router.push('/')
  isMenuOpen.value = false
}

function handleClickOutside(event) {
  const menu = document.querySelector('.relative')
  if (menu && !menu.contains(event.target)) {
    isMenuOpen.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})
onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})


const programs = ref([]);

async function fetchPrograms() {
  try {
    const response = await fetch('http://localhost:3000/programs'); // Replace with your Rust API endpoint
    if (!response.ok) {
      throw new Error('Failed to fetch programs');
    }
    programs.value = await response.json();
  } catch (error) {
    console.error('Error fetching programs:', error);
  }
}

onMounted(() => {
  fetchPrograms();
});
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-clamp: 2; /* Added standard property for compatibility */
}
</style>
