<template>
  <router-view />
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useAuthStore } from './stores/auth'

onMounted(() => {
  // This will initialize the store from cookies when the app loads.
  // The store itself is already written to do this, but this ensures it happens at app startup.
  useAuthStore()
})
</script>
