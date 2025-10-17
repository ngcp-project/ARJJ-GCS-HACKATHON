import { defineStore } from 'pinia'
import { ref } from 'vue'
import Cookies from 'js-cookie'

export const useAuthStore = defineStore('auth', () => {
  const username = ref<string | null>(Cookies.get('username') || null)
  const barcode = ref<number | null>(Number(Cookies.get('barcode')) || null)
  const loggedIn = ref<boolean>(!!Cookies.get('loggedIn'))

  function setAuth(user: string, code: number) {
    username.value = user
    barcode.value = code
    loggedIn.value = true
    Cookies.set('username', user, { expires: 7 })
    Cookies.set('barcode', String(code), { expires: 7 })
    Cookies.set('loggedIn', 'true', { expires: 7 })
  }

  function clearAuth() {
    username.value = null
    barcode.value = null
    loggedIn.value = false
    Cookies.remove('username')
    Cookies.remove('barcode')
    Cookies.remove('loggedIn')
  }

  return { username, barcode, loggedIn, setAuth, clearAuth }
})
