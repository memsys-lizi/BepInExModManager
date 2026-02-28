// ===== Game =====
export interface Game {
  id: string
  name: string
  path: string       // 游戏根目录路径
  exeName: string    // 游戏可执行文件名
  addedAt: number    // 时间戳
  coverImage?: string
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

export interface ModInfo {
  id: string
  name: string
  fileName: string   // .dll 文件名
  filePath: string   // 完整路径
  status: ModStatus
  version?: string
  description?: string
  author?: string
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
