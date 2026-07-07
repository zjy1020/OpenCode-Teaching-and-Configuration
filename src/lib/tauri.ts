class MockWindow {
  minimize() { return Promise.resolve() }
  toggleMaximize() { return Promise.resolve() }
  close() { return Promise.resolve() }
  startDragging() { return Promise.resolve() }
}

export async function getWindow() {
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    return getCurrentWindow()
  } catch {
    return new MockWindow()
  }
}

export async function openUrl(url: string) {
  try {
    const { open } = await import('@tauri-apps/plugin-shell')
    await open(url)
  } catch {
    window.open(url, '_blank')
  }
}

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    const { invoke: ti } = await import('@tauri-apps/api/core')
    return ti<T>(cmd, args)
  } catch {
    switch (cmd) {
      case 'get_opencode_dir': return '~/.opencode' as T
      case 'scan_skills': return [] as T
      case 'scan_commands': return [] as T
      case 'import_skill': return undefined as T
      case 'import_command': return undefined as T
      case 'remove_skill': return undefined as T
      case 'remove_command': return undefined as T
      case 'open_folder': return undefined as T
      default: throw new Error(`Unknown command: ${cmd}`)
    }
  }
}
