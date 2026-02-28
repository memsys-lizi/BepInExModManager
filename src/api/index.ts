import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// ===== Game =====
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
export interface BepInExStatusRaw {
  installed: boolean
  version?: string
  path?: string
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
export interface ModEntryRaw {
  id: string
  name: string
  file_name: string
  file_path: string
  status: 'enabled' | 'disabled'
}

export function scanMods(gamePath: string): Promise<ModEntryRaw[]> {
  return invoke('scan_mods', { gamePath })
}

export function enableMod(filePath: string): Promise<string> {
  return invoke('enable_mod', { filePath })
}

export function disableMod(filePath: string): Promise<string> {
  return invoke('disable_mod', { filePath })
}

export function deleteMod(filePath: string): Promise<void> {
  return invoke('delete_mod', { filePath })
}

export function openPluginsDir(gamePath: string): Promise<void> {
  return invoke('open_plugins_dir', { gamePath })
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
