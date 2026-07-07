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
    case 'read_theme_content': return JSON.stringify({
  $schema: "https://opencode.ai/theme.json",
  defs: { amber: "#FF6B1A", amberDark: "#C94F0A", amberDeep: "#A03A00", golden: "#FFC800", goldenDark: "#C99900", teal: "#2DD4BF", tealDark: "#0D9488", green: "#34D399", greenDark: "#059669", red: "#FF4D4D", redDark: "#DC2626", tan: "#E8B88A", bgLight: "#FFF5E6", bgPanelLight: "#FFFBF5", bgElemLight: "#FFFFFF", borderLight: "#E8D8C8", textLight: "#3D2B1F", mutedLight: "#8B7A6A", diffAddedBgLight: "#E8F8EE", diffRemovedBgLight: "#F8E8E8" },
  theme: { primary: { dark: "amberDark", light: "amberDark" }, secondary: { dark: "tealDark", light: "tealDark" }, accent: { dark: "amber", light: "amber" }, error: { dark: "redDark", light: "redDark" }, warning: { dark: "goldenDark", light: "goldenDark" }, success: { dark: "greenDark", light: "greenDark" }, info: { dark: "tealDark", light: "tealDark" }, text: { dark: "textLight", light: "textLight" }, textMuted: { dark: "mutedLight", light: "mutedLight" }, background: { dark: "bgLight", light: "bgLight" }, backgroundPanel: { dark: "bgPanelLight", light: "bgPanelLight" }, backgroundElement: { dark: "bgElemLight", light: "bgElemLight" }, border: { dark: "borderLight", light: "borderLight" }, borderActive: { dark: "amberDark", light: "amberDark" }, borderSubtle: { dark: "borderLight", light: "borderLight" }, diffAdded: { dark: "greenDark", light: "greenDark" }, diffRemoved: { dark: "redDark", light: "redDark" }, diffContext: { dark: "mutedLight", light: "mutedLight" }, diffHunkHeader: { dark: "amberDark", light: "amberDark" }, diffHighlightAdded: { dark: "green", light: "green" }, diffHighlightRemoved: { dark: "red", light: "red" }, diffAddedBg: { dark: "diffAddedBgLight", light: "diffAddedBgLight" }, diffRemovedBg: { dark: "diffRemovedBgLight", light: "diffRemovedBgLight" }, diffContextBg: { dark: "bgPanelLight", light: "bgPanelLight" }, diffLineNumber: { dark: "mutedLight", light: "mutedLight" }, diffAddedLineNumberBg: { dark: "diffAddedBgLight", light: "diffAddedBgLight" }, diffRemovedLineNumberBg: { dark: "diffRemovedBgLight", light: "diffRemovedBgLight" }, selectedListItemText: { dark: "#FFFFFF", light: "#FFFFFF" }, backgroundMenu: { dark: "#FFF0D8", light: "#FFF0D8" }, selection: { dark: "#2563EB", light: "#2563EB" }, selectionForeground: { dark: "#FFFFFF", light: "#FFFFFF" }, markdownText: { dark: "textLight", light: "textLight" }, markdownHeading: { dark: "amberDark", light: "amberDark" }, markdownLink: { dark: "tealDark", light: "tealDark" }, markdownLinkText: { dark: "teal", light: "teal" }, markdownCode: { dark: "greenDark", light: "greenDark" }, markdownBlockQuote: { dark: "mutedLight", light: "mutedLight" }, markdownEmph: { dark: "goldenDark", light: "goldenDark" }, markdownStrong: { dark: "amberDark", light: "amberDark" }, markdownHorizontalRule: { dark: "borderLight", light: "borderLight" }, markdownListItem: { dark: "amberDark", light: "amberDark" }, markdownListEnumeration: { dark: "tealDark", light: "tealDark" }, markdownImage: { dark: "tealDark", light: "tealDark" }, markdownImageText: { dark: "amberDark", light: "amberDark" }, markdownCodeBlock: { dark: "textLight", light: "textLight" }, syntaxComment: { dark: "#9CA3AF", light: "#9CA3AF" }, syntaxKeyword: { dark: "#C94F0A", light: "#C94F0A" }, syntaxFunction: { dark: "#F59E0B", light: "#F59E0B" }, syntaxVariable: { dark: "#3B82F6", light: "#3B82F6" }, syntaxString: { dark: "#059669", light: "#059669" }, syntaxNumber: { dark: "#D97706", light: "#D97706" }, syntaxType: { dark: "#8B5CF6", light: "#8B5CF6" }, syntaxOperator: { dark: "#B45309", light: "#B45309" }, syntaxPunctuation: { dark: "#8B7A6A", light: "#8B7A6A" } }
}) as T
    default: throw new Error(`Unknown command: ${cmd}`)
  }
}