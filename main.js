import { shallowRef, computed } from 'vue'
import { list } from '@/helpers/profile.js'
import { handleError } from '@/store/notifications.js'
import dayjs from 'dayjs'
import 'omorphia/dist/style.css'
import '@/assets/stylesheets/global.scss'
import 'floating-vue/dist/style.css'
import { initialize_state } from '@/helpers/state'
import { get } from '@/helpers/settings'
console.log("instance.value");

initialize_state()
  .then(() => {
    // First, redirect to other landing page if we have that setting
    get()
      .then(() => {
        const recentInstances = shallowRef([])

        const getInstances = async () => {
          const profiles = await list(true).catch(handleError)
          recentInstances.value = Object.values(profiles).sort((a, b) => {
            return dayjs(b.metadata.last_played ?? 0).diff(dayjs(a.metadata.last_played ?? 0))
          })
        }
        
        const instance = computed(() => {
          return recentInstances.value[0]
        })
        
        getInstances().then(() => {
            console.log("instance.value");
            console.log(instance.value);
        })
      })
  })
