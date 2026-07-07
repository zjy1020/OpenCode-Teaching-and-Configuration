import { ref } from 'vue'
import { invoke } from '../lib/tauri'

export interface ScanResult {
  installed: string[]
}

export function useScanner() {
  const scanning = ref(false)
  const error = ref('')

  async function scanSkills(): Promise<ScanResult> {
    scanning.value = true
    error.value = ''
    try {
      const installed = await invoke<string[]>('scan_skills')
      return { installed }
    } catch (e) {
      error.value = String(e)
      return { installed: [] }
    } finally {
      scanning.value = false
    }
  }

  async function scanCommands(): Promise<ScanResult> {
    scanning.value = true
    error.value = ''
    try {
      const installed = await invoke<string[]>('scan_commands')
      return { installed }
    } catch (e) {
      error.value = String(e)
      return { installed: [] }
    } finally {
      scanning.value = false
    }
  }

  return { scanning, error, scanSkills, scanCommands }
}
