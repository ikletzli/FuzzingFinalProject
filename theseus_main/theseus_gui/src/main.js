// import { shallowRef, computed, ref } from 'vue'
// import {
//   remove_project,
//   toggle_disable_project,
//   update_project,
//   list
// } from '@/helpers/profile.js'

// import { mixpanel_track } from '@/helpers/mixpanel'
// import { handleError } from '@/store/notifications.js'
// import dayjs from 'dayjs'
// import 'omorphia/dist/style.css'
// import '@/assets/stylesheets/global.scss'
// import 'floating-vue/dist/style.css'
// import { initialize_state } from '@/helpers/state'
// import { get } from '@/helpers/settings'
// import { convertFileSrc } from '@tauri-apps/api/tauri'
// import {
//   formatProjectType,
// } from 'omorphia'
// import { highlightModInProfile } from '@/helpers/utils.js'

// const selectionMap = ref(new Map())
// let locks = {}

// const updateProject = async (mod, instance) => {
//   mod.updating = true
//   await new Promise((resolve) => setTimeout(resolve, 0))
//   mod.path = await update_project(instance.value.path, mod.path).catch(handleError)
//   mod.updating = false

//   mod.outdated = false
//   mod.version = mod.updateVersion.version_number
//   mod.updateVersion = null

//   mixpanel_track('InstanceProjectUpdate', {
//     loader: instance.value.metadata.loader,
//     game_version: instance.value.metadata.game_version,
//     id: mod.id,
//     name: mod.name,
//     project_type: mod.project_type,
//   })
// }

// const toggleDisableMod = async (mod, instance) => {
//   // Use mod's id as the key for the lock. If mod doesn't have a unique id, replace `mod.id` with some unique property.
//   if (!locks[mod.id]) {
//     locks[mod.id] = ref(null)
//   }

//   let lock = locks[mod.id]

//   while (lock.value) {
//     await lock.value
//   }

//   lock.value = toggle_disable_project(instance.value.path, mod.path)
//     .then((newPath) => {
//       mod.path = newPath
//       mod.disabled = !mod.disabled
//       mixpanel_track('InstanceProjectDisable', {
//         loader: instance.value.metadata.loader,
//         game_version: instance.value.metadata.game_version,
//         id: mod.id,
//         name: mod.name,
//         project_type: mod.project_type,
//         disabled: mod.disabled,
//       })
//     })
//     .catch(handleError)
//     .finally(() => {
//       lock.value = null
//     })

//   await lock.value
// }

// const removeMod = async (mod, instance, projects) => {
//   await remove_project(instance.value.path, mod.path).catch(handleError)
//   projects.value = projects.value.filter((x) => mod.path !== x.path)

//   mixpanel_track('InstanceProjectRemove', {
//     loader: instance.value.metadata.loader,
//     game_version: instance.value.metadata.game_version,
//     id: mod.id,
//     name: mod.name,
//     project_type: mod.project_type,
//   })
// }

// initialize_state()
//   .then(() => {
//     // First, redirect to other landing page if we have that setting
//     get()
//       .then(() => {
//         const recentInstances = shallowRef([])

//         const getInstances = async () => {
//           const profiles = await list(true).catch(handleError)
//           recentInstances.value = Object.values(profiles).sort((a, b) => {
//             return dayjs(b.metadata.last_played ?? 0).diff(dayjs(a.metadata.last_played ?? 0))
//           })
//         }
        
//         const instance = computed(() => {
//           return recentInstances.value[0]
//         })
        
//         getInstances().then(() => {
//           const projects = ref([])

//           const obj = { All: 'all' }

//           projects.value = []
//           if (!instance.value || !instance.value.projects) return
//           for (const [path, project] of Object.entries(instance.value.projects)) {
//             if (project.metadata.type === 'modrinth') {
//               let owner = project.metadata.members.find((x) => x.role === 'Owner')
//               projects.value.push({
//                 path,
//                 name: project.metadata.project.title,
//                 slug: project.metadata.project.slug,
//                 author: owner ? owner.user.username : null,
//                 version: project.metadata.version.version_number,
//                 file_name: project.file_name,
//                 icon: project.metadata.project.icon_url,
//                 disabled: project.disabled,
//                 updateVersion: project.metadata.update_version,
//                 outdated: !!project.metadata.update_version,
//                 project_type: project.metadata.project.project_type,
//                 id: project.metadata.project.id,
//               })
//             } else if (project.metadata.type === 'inferred') {
//               projects.value.push({
//                 path,
//                 name: project.metadata.title ?? project.file_name,
//                 author: project.metadata.authors[0],
//                 version: project.metadata.version,
//                 file_name: project.file_name,
//                 icon: project.metadata.icon ? convertFileSrc(project.metadata.icon) : null,
//                 disabled: project.disabled,
//                 outdated: false,
//                 project_type: project.metadata.project_type,
//               })
//             } else {
//               projects.value.push({
//                 path,
//                 name: project.file_name,
//                 author: '',
//                 version: null,
//                 file_name: project.file_name,
//                 icon: null,
//                 disabled: project.disabled,
//                 outdated: false,
//                 project_type: null,
//               })
//             }
//           }
        
//           const newSelectionMap = new Map()
//           for (const project of projects.value) {
//             newSelectionMap.set(
//               project.path,
//               selectionMap.value.get(project.path) ??
//                 selectionMap.value.get(project.path.slice(0, -9)) ??
//                 selectionMap.value.get(project.path + '.disabled') ??
//                 false
//             )
//           }
//           selectionMap.value = newSelectionMap

//           for (const project of projects.value) {
//             obj[project.project_type ? formatProjectType(project.project_type) + 's' : 'Other'] =
//               project.project_type
//           }
        
//           const selectableProjectTypes = obj
//           const selectedProjectType = ref('All')
//           const searchFilter = ref('')
//           const hideNonSelected = ref(false)

//           const projectType = selectableProjectTypes.value[selectedProjectType.value]
//           const filtered = projects.value
//             .filter((mod) => {
//               return (
//                 mod.name.toLowerCase().includes(searchFilter.value.toLowerCase()) &&
//                 (projectType === 'all' || mod.project_type === projectType)
//               )
//             })
//             .filter((mod) => {
//               if (hideNonSelected.value) {
//                 return !mod.disabled
//               }
//               return true
//             })
        
//           const search = filtered
//           const currentPage = ref(1)

//           search.slice((currentPage.value - 1) * 20, currentPage.value * 20).forEach( (mod) => {
//             console.log(mod.file_name)
//             console.log(mod.slug)
//             console.log(mod.icon)
//             console.log(mod.name)
//             console.log(mod.author)
//             console.log(mod.version)
//             console.log(mod.updating)
//             console.log(mod.outdated)
//             console.log(mod.disabled)

//             selectionMap.value.get(mod.path)
//             selectionMap.value.set(mod.path, false)

//             updateProject(mod, instance)
//             highlightModInProfile(instance.value.path, mod.path)
//             toggleDisableMod(mod, instance)
//             removeMod(mod, instance, projects)
//           })


//         })
//       })
//   })
