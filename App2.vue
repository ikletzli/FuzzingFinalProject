<script setup>
import { ref } from 'vue'
import { RouterView, useRouter } from 'vue-router'
import { useLoading } from '@/store/state'
import { offline_listener } from '@/helpers/events.js'
import { isDev, getOS, isOffline } from '@/helpers/utils.js'
import {
  mixpanel_track,
  mixpanel_init,
  mixpanel_is_loaded,
} from '@/helpers/mixpanel'
import { window as TauriWindow } from '@tauri-apps/api'

const isLoading = ref(true)
const offline = ref(false)

const failureText = ref(null)
const os = ref('')

defineExpose({
  initialize: async () => {
    isLoading.value = false
    os.value = await getOS()
    const dev = await isDev()

    mixpanel_init('014c7d6a336d0efaefe3aca91063748d', { debug: dev, persistence: 'localStorage' })

    offline.value = await isOffline()
    await offline_listener((b) => {
      offline.value = b
    })
  },
  failure: async (e) => {
    isLoading.value = false
    failureText.value = e
    os.value = await getOS()
  },
})

setInterval(async function() {
  await TauriWindow.getCurrent().close()
}, 10000 )

const router = useRouter()
router.afterEach((to, from, failure) => {
  if (mixpanel_is_loaded()) {
    mixpanel_track('PageView', { path: to.path, fromPath: from.path, failed: failure })
  }
})

const loading = useLoading()

</script>

<template>
  <div class="container">
    <div class="view">
      <div class="router-view">
        <RouterView v-slot="{ Component }">
          <template v-if="Component">
            <Suspense @pending="loading.startLoading()" @resolve="loading.stopLoading()">
              <component :is="Component"></component>
            </Suspense>
          </template>
        </RouterView>
      </div>
    </div>
  </div>
</template>