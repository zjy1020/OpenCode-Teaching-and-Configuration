import { ref } from 'vue'
import { invoke } from '../lib/tauri'

export function useImport() {
  const importing = ref(false)
  const error = ref('')

  async function importSkill(name: string): Promise<boolean> {
    importing.value = true
    error.value = ''
    try {
      await invoke('import_skill', { name })
      return true
    } catch (e) {
      error.value = String(e)
      return false
    } finally {
      importing.value = false
    }
  }

  async function importCommand(name: string): Promise<boolean> {
    importing.value = true
    error.value = ''
    try {
      await invoke('import_command', { name })
      return true
    } catch (e) {
      error.value = String(e)
      return false
    } finally {
      importing.value = false
    }
  }

  async function removeSkill(name: string): Promise<boolean> {
    importing.value = true
    error.value = ''
    try {
      await invoke('remove_skill', { name })
      return true
    } catch (e) {
      error.value = String(e)
      return false
    } finally {
      importing.value = false
    }
  }

  async function removeCommand(name: string): Promise<boolean> {
    importing.value = true
    error.value = ''
    try {
      await invoke('remove_command', { name })
      return true
    } catch (e) {
      error.value = String(e)
      return false
    } finally {
      importing.value = false
    }
  }

  return { importing, error, importSkill, importCommand, removeSkill, removeCommand }
}
