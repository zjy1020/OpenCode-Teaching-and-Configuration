class MockWindow {
  minimize() { return Promise.resolve() }
  toggleMaximize() { return Promise.resolve() }
  close() { return Promise.resolve() }
  startDragging() { return Promise.resolve() }
}

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window

export async function getWindow() {
  if (!isTauri) return new MockWindow()
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    return getCurrentWindow()
  } catch {
    return new MockWindow()
  }
}

export async function openUrl(url: string) {
  if (!isTauri) {
    window.open(url, '_blank')
    return
  }
  try {
    const { open } = await import('@tauri-apps/plugin-shell')
    await open(url)
  } catch {
    window.open(url, '_blank')
  }
}

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri) {
    const { invoke: ti } = await import('@tauri-apps/api/core')
    return ti<T>(cmd, args)
  }
  switch (cmd) {
    case 'get_opencode_dir': return '~/.opencode' as T
    case 'scan_skills': return [] as T
    case 'scan_commands': return [] as T
    case 'import_skill': return undefined as T
    case 'import_command': return undefined as T
    case 'remove_skill': return undefined as T
    case 'remove_command': return undefined as T
    case 'open_folder': return undefined as T
    case 'read_tui_config': return { theme: 'ember-glow', diff_style: 'auto', attention: { enabled: true, notifications: true, sound: true, volume: 0.4 } } as T
    case 'write_tui_config': return undefined as T
    case 'list_embedded_themes': return ['ember-glow'] as T
    case 'check_theme_installed': return true as T
    case 'import_theme': return undefined as T
    case 'remove_theme': return undefined as T
    case 'sync_author_config': return undefined as T
    case 'reset_tui_config': return undefined as T
    default: throw new Error(`Unknown command: ${cmd}`)
  }
}