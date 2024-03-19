  const {     Avatar,
    Button,
    TrashIcon,
    Card,
    CheckIcon,
    SearchIcon,
    UpdatedIcon,
    AnimatedLogo,
    FolderOpenIcon,
    Checkbox,
    formatProjectType,
    DropdownButton,
    Modal,
    XIcon,
    ShareIcon,
    DropdownIcon,
    GlobeIcon,
    FileIcon,
    EyeIcon,
    EyeOffIcon,
    ShareModal,
    CodeIcon,
    Pagination,
    DropdownSelect } = require('omorphia');

    const { list } = require('@/helpers/profile.js');
    const { handleError } = require('@/store/notifications.js');

  async function myFunction(query) {
      const profiles = await list(true).catch(handleError)
      const instances = shallowRef(Object.values(profiles))
  }

  module.exports = {
    myFunction,
};

  const profiles = await list(true).catch(handleError)
  const instances = shallowRef(Object.values(profiles))

  const projects = ref([])

  const initProjects = (initInstance) => {
    projects.value = []
    if (!initInstance || !initInstance.projects) return
    for (const [path, project] of Object.entries(initInstance.projects)) {
      if (project.metadata.type === 'modrinth' && !props.offline) {
        let owner = project.metadata.members.find((x) => x.role === 'Owner')
        projects.value.push({
          path,
          name: project.metadata.project.title,
          slug: project.metadata.project.slug,
          author: owner ? owner.user.username : null,
          version: project.metadata.version.version_number,
          file_name: project.file_name,
          icon: project.metadata.project.icon_url,
          disabled: project.disabled,
          updateVersion: project.metadata.update_version,
          outdated: !!project.metadata.update_version,
          project_type: project.metadata.project.project_type,
          id: project.metadata.project.id,
        })
      } else if (project.metadata.type === 'inferred') {
        projects.value.push({
          path,
          name: project.metadata.title ?? project.file_name,
          author: project.metadata.authors[0],
          version: project.metadata.version,
          file_name: project.file_name,
          icon: project.metadata.icon ? convertFileSrc(project.metadata.icon) : null,
          disabled: project.disabled,
          outdated: false,
          project_type: project.metadata.project_type,
        })
      } else {
        projects.value.push({
          path,
          name: project.file_name,
          author: '',
          version: null,
          file_name: project.file_name,
          icon: null,
          disabled: project.disabled,
          outdated: false,
          project_type: null,
        })
      }
    }
  
    const newSelectionMap = new Map()
    for (const project of projects.value) {
      newSelectionMap.set(
        project.path,
        selectionMap.value.get(project.path) ??
          selectionMap.value.get(project.path.slice(0, -9)) ??
          selectionMap.value.get(project.path + '.disabled') ??
          false
      )
    }
    selectionMap.value = newSelectionMap
  }

initProjects(props.instance)

const selectableProjectTypes = computed(() => {
    const obj = { All: 'all' }
  
    for (const project of projects.value) {
      obj[project.project_type ? formatProjectType(project.project_type) + 's' : 'Other'] =
        project.project_type
    }
  
    return obj
  })

const search = computed(() => {
    const projectType = selectableProjectTypes.value[selectedProjectType.value]
    const filtered = projects.value
      .filter((mod) => {
        return (
          mod.name.toLowerCase().includes(searchFilter.value.toLowerCase()) &&
          (projectType === 'all' || mod.project_type === projectType)
        )
      })
      .filter((mod) => {
        if (hideNonSelected.value) {
          return !mod.disabled
        }
        return true
      })
  
    return updateSort(filtered)
  })