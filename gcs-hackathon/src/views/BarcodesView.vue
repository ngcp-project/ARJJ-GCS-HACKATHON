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
        <button
          @click="goToCatalog"
          class="bg-white text-green-800 px-4 py-2 rounded-md font-semibold hover:bg-gray-200 transition"
        >
          Back to Catalog
        </button>
      </div>
    </header>

    <!-- Barcode Section -->
    <main class="flex-1 max-w-4xl mx-auto px-6 py-12 text-center">
      <h2 class="text-3xl font-bold text-green-800 mb-4">Barcodes</h2>
      <p class="text-gray-600 mb-8">
        This page will display your membership barcodes.
      </p>

      <!-- User barcode area -->
      <div class="bg-white rounded-xl shadow-md p-8 inline-block">
        <div v-if="auth.loggedIn">
          <img :src="barcodeUrl" alt="User Barcode" class="mx-auto" />
          <p class="mt-4 text-sm text-gray-700">{{ auth.barcode }}</p>
        </div>
        <div v-else>
          <p class="text-gray-600">Please sign in to view your barcode.</p>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import { computed } from 'vue'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const auth = useAuthStore()

function goToCatalog() {
  router.push('/')
}

const barcodeUrl = computed(() => {
  if (!auth.barcode) return ''
  // Use an online barcode generator that accepts data via query param. For example, barcode.tec-it.com supports a URL-based generator.
  // We'll use CODE128 symbology.
  return `https://barcode.tec-it.com/barcode.ashx?data=${encodeURIComponent(String(auth.barcode))}&code=Code128&translate-esc=off`
})
</script>
