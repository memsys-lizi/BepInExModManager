import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// ===== Game =====
export function extractExeIcon(exePath: string): Promise<string> {
  return invoke('extract_exe_icon', { exePath })
}

export interface GameValidation {
  valid: boolean
  has_data_folder: boolean
  has_exe: boolean
  exe_name?: string
  message: string
}

export function validateGameDir(path: string): Promise<GameValidation> {
  return invoke('validate_game_dir', { path })
}

// ===== BepInEx =====
export interface BepInExIntegrity {
  winhttp_dll: boolean
  doorstop_cfg: boolean
  doorstop_version: boolean
  core_dir: boolean
  core_bepinex_dll: boolean
  changelog: boolean
  score: number   // 必须项满分 5，>=4 视为已安装
}

export interface BepInExStatusRaw {
  installed: boolean
  version?: string
  path?: string
  integrity: BepInExIntegrity
}

export function checkBepInExStatus(gamePath: string): Promise<BepInExStatusRaw> {
  return invoke('check_bepinex_status', { gamePath })
}

export interface ReleaseAsset {
  version: string
  tag_name: string
  download_url: string
  arch: 'x64' | 'x86' | 'unix'
  published_at: string
}

export function fetchBepInExReleases(proxy?: string): Promise<ReleaseAsset[]> {
  return invoke('fetch_bepinex_releases', { proxy: proxy || null })
}

export interface InstallProgress {
  stage: 'downloading' | 'extracting' | 'done'
  percent: number
  message: string
}

export async function installBepInEx(
  gamePath: string,
  downloadUrl: string,
  version: string,
  onProgress?: (p: InstallProgress) => void,
  proxy?: string,
): Promise<void> {
  let unlisten: (() => void) | undefined
  if (onProgress) {
    unlisten = await listen<InstallProgress>('bepinex://progress', (e) => {
      onProgress(e.payload)
    })
  }
  try {
    await invoke('install_bepinex', { gamePath, downloadUrl, version, proxy: proxy || null })
  } finally {
    unlisten?.()
  }
}

export function uninstallBepInEx(gamePath: string): Promise<void> {
  return invoke('uninstall_bepinex', { gamePath })
}

// ===== Mods =====
export interface DllEntryRaw {
  name: string
  file_name: string
  file_path: string
  status: 'enabled' | 'disabled'
}

export interface ModEntryRaw {
  id: string
  name: string
  mod_path: string
  is_folder: boolean
  status: 'enabled' | 'disabled'
  dlls: DllEntryRaw[]
}

export function scanMods(gamePath: string): Promise<ModEntryRaw[]> {
  return invoke('scan_mods', { gamePath })
}

export function enableMod(modPath: string, isFolder: boolean): Promise<string> {
  return invoke('enable_mod', { modPath, isFolder })
}

export function disableMod(modPath: string, isFolder: boolean): Promise<string> {
  return invoke('disable_mod', { modPath, isFolder })
}

export function deleteMod(modPath: string, isFolder: boolean): Promise<void> {
  return invoke('delete_mod', { modPath, isFolder })
}

export function openPluginsDir(gamePath: string): Promise<void> {
  return invoke('open_plugins_dir', { gamePath })
}

export type ConflictStrategy = 'rename' | 'overwrite' | 'cancel'

export interface InstallModResult {
  mod_name: string
  mod_path: string
  is_folder: boolean
  conflict: boolean
  conflict_path: string | null
}

export function installMod(
  gamePath: string,
  sourcePath: string,
  conflictStrategy?: ConflictStrategy,
): Promise<InstallModResult> {
  return invoke('install_mod', { gamePath, sourcePath, conflictStrategy: conflictStrategy ?? null })
}

export function installModFromUrl(
  gamePath: string,
  url: string,
  conflictStrategy?: ConflictStrategy,
): Promise<InstallModResult> {
  return invoke('install_mod_from_url', { gamePath, url, conflictStrategy: conflictStrategy ?? null })
}

export function launchGame(gamePath: string, exeName: string): Promise<void> {
  return invoke('launch_game', { gamePath, exeName })
}

// ===== Config =====
export interface ConfigEntryRaw {
  key: string
  value: string
  description?: string
  entry_type: string
  options?: string[]
}

export interface ConfigSectionRaw {
  name: string
  entries: ConfigEntryRaw[]
}

export function listConfigFiles(gamePath: string): Promise<string[]> {
  return invoke('list_config_files', { gamePath })
}

export function readConfigFile(filePath: string): Promise<ConfigSectionRaw[]> {
  return invoke('read_config_file', { filePath })
}

export function writeConfigFile(
  filePath: string,
  sections: ConfigSectionRaw[],
): Promise<void> {
  return invoke('write_config_file', { filePath, sections })
}
