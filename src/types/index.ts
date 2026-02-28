// ===== Game =====
export interface Game {
  id: string
  name: string
  path: string
  exeName: string
  addedAt: number
  iconBase64?: string   // exe 图标的 PNG base64（data:image/png;base64,...）
}

// ===== BepInEx =====
export type BepInExArch = 'x64' | 'x86'
export type BepInExStatus = 'not_installed' | 'installed' | 'outdated'

export interface BepInExInfo {
  installed: boolean
  version?: string
  arch?: BepInExArch
  path?: string
}

export interface BepInExRelease {
  version: string
  downloadUrl: string
  arch: BepInExArch
  publishedAt: string
}

// ===== Mod =====
export type ModStatus = 'enabled' | 'disabled'

export interface DllInfo {
  name: string
  fileName: string
  filePath: string
  status: ModStatus
}

export interface ModInfo {
  id: string
  name: string
  modPath: string    // 文件夹路径（或散装 dll 路径）
  isFolder: boolean
  status: ModStatus
  dlls: DllInfo[]
  gameId: string
}

// ===== Config =====
export interface ConfigSection {
  name: string
  entries: ConfigEntry[]
}

export interface ConfigEntry {
  key: string
  value: string
  description?: string
  type: 'string' | 'number' | 'boolean' | 'enum'
  options?: string[]  // enum 类型的可选值
}

// ===== Settings =====
export type ThemeMode = 'dark' | 'light' | 'system'

export interface AppSettings {
  theme: 'dark' | 'light'
  themeMode: ThemeMode
  downloadProxy?: string
  bepinexSource: 'github' | 'mirror'
  language: 'zh-CN' | 'en-US'
  windowWidth: number
  windowHeight: number
}
