export default {
  // ── 通用 ──
  common: {
    save: '保存',
    saving: '保存中...',
    saved: '已保存',
    cancel: '取消',
    confirm: '确认',
    delete: '删除',
    refresh: '刷新',
    loading: '加载中...',
    empty: '暂无数据',
    reset: '恢复默认',
    close: '关闭',
    unknown: '未知',
    yes: '是',
    no: '否',
  },

  // ── 侧栏 ──
  sidebar: {
    games: '游戏',
    noGames: '暂无游戏',
    addGame: '添加游戏',
    home: '首页',
    settings: '设置',
    language: '语言',
    theme: '主题',
    themeDark: '深色',
    themeLight: '浅色',
    themeSystem: '跟随系统',
  },

  // ── 首页 ──
  home: {
    title: '游戏列表',
    addGame: '添加游戏',
    addGameDesc: '选择游戏可执行文件（.exe）来添加游戏',
    pickExe: '选择游戏 .exe',
    gameName: '游戏名称',
    gameNamePlaceholder: '自动从文件夹名获取',
    exeName: '可执行文件名',
    validating: '正在验证...',
    deleteGame: '移除游戏',
    deleteGameConfirm: '确定要从列表中移除',
    deleteGameConfirmSuffix: '吗？（不会删除游戏文件）',
    modsCount: '个 Mod',
    bepInstalled: 'BepInEx 已安装',
    bepNotInstalled: '未安装 BepInEx',
  },

  // ── BepInEx 安装页 ──
  bepinex: {
    title: 'BepInEx',
    currentStatus: '当前状态',
    installed: '已安装',
    notInstalled: '未安装',
    version: '当前版本',
    gameDir: '游戏目录',
    integrity: '完整性',
    integrityPass: '必须项通过',
    integrityDesc: '（≥4 已安装）',
    selectVersion: '选择版本',
    versionHint: '大多数游戏选 x64；若游戏为 32 位进程则选 x86。不确定时优先选 x64。',
    winOnlyNote: '仅 Windows 正式版',
    install: '安装',
    reinstall: '重新安装',
    uninstall: '卸载',
    refreshList: '刷新',
    fetching: '正在获取...',
    fetchFailed: '获取失败，请检查网络或在设置中配置代理',
    installSuccess: '安装成功',
  },

  // ── Mod 列表页 ──
  mods: {
    title: 'Mod 列表',
    bepNotInstalled: '未安装 BepInEx',
    bepNotInstalledHint: '请先安装 BepInEx 才能管理 Mod',
    goInstall: '去安装',
    openConfig: '配置文件',
    noMods: '暂无 Mod',
    noModsHint: '将 Mod 文件夹放入游戏的 BepInEx/plugins 目录后重新扫描',
    rescan: '重新扫描',
    enabled: '已启用',
    disabled: '已禁用',
    deleteMod: '删除 Mod',
    deleteModConfirm: '确定要删除',
    deleteModConfirmSuffix: '吗？此操作不可恢复',
    folder: '文件夹',
    file: '文件',
    expandDlls: '展开 DLL',
  },

  // ── 配置文件页 ──
  config: {
    title: '配置文件',
    noFiles: '未找到配置文件',
    noFilesHint: '运行一次游戏后，BepInEx 会自动生成各 Mod 的配置文件',
    empty: '配置文件为空',
    reading: '读取中...',
    // BepInEx.cfg 各字段说明
    fieldDesc: {
      // [Caching]
      EnableAssemblyCache: '启用程序集缓存。开启后 BepInEx 会缓存已加载的程序集，加快后续启动速度；关闭则每次启动都重新加载。',
      // [Chainloader]
      HideManagerGameObject: '隐藏 BepInEx 管理器对象。开启后游戏内看不到 BepInEx 的管理 GameObject，可避免某些游戏因检测到它而崩溃。',
      // [Harmony.Logger]
      LogChannels: '控制 Harmony 补丁库的日志输出级别。可填写 None / Warn / Error / Info / All，多个用逗号分隔。',
      // [Logging]
      UnityLogListening: '监听 Unity 引擎自身的日志。开启后 Unity 内部日志也会被 BepInEx 捕获并写入日志文件。',
      LogConsoleToUnityLog: '将 BepInEx 控制台输出同步写入 Unity 日志。通常关闭，避免日志重复。',
      // [Logging.Console]
      'Logging.Console.Enabled': '开启独立的 BepInEx 控制台窗口。关闭后不会弹出黑色命令行窗口（适合正式游玩时关闭）。',
      'Logging.Console.PreventClose': '防止用户关闭控制台窗口时同时关闭游戏。',
      ShiftJisEncoding: '使用 Shift-JIS 编码显示控制台文字。仅在运行日语游戏且控制台出现乱码时开启。',
      StandardOutType: '控制台输出重定向模式。Auto = 自动，ConsoleOut = 强制控制台，LogFile = 仅写文件。',
      'Logging.Console.LogLevels': '控制台显示的日志级别。可填 Fatal / Error / Warning / Message / Info / Debug / All，多个用逗号分隔。',
      // [Logging.Disk]
      WriteUnityLog: '将 Unity 引擎日志同步写入 BepInEx 日志文件。',
      AppendLog: '追加写入日志文件，而非每次启动时覆盖。关闭则每次启动清空旧日志。',
      'Logging.Disk.Enabled': '是否将日志写入磁盘文件（BepInEx/LogOutput.log）。建议保持开启，方便排查问题。',
      'Logging.Disk.LogLevels': '写入磁盘的日志级别。可填 Fatal / Error / Warning / Message / Info / Debug / All，多个用逗号分隔。',
      // [Preloader]
      ApplyRuntimePatches: '在运行时应用 BepInEx 的运行时补丁，提高兼容性。一般保持开启。',
      HarmonyBackend: 'Harmony 补丁后端。auto = 自动选择，MonoMod = 使用 MonoMod，NativeDetour = 使用原生钩子。',
      DumpAssemblies: '将游戏程序集转储到磁盘（调试用）。正常游玩请关闭，否则会降低启动速度。',
      LoadDumpedAssemblies: '启动时从磁盘加载已转储的程序集，而非从内存加载（高级调试选项）。',
      BreakBeforeLoadAssemblies: '在加载程序集前中断，等待调试器连接（仅开发者使用）。',
      // [Preloader.Entrypoint]
      Assembly: 'BepInEx 注入点所在的程序集文件名。通常为 UnityEngine.CoreModule.dll，无需修改。',
      Type: '注入点所在的类名（位于上述程序集中）。',
      Method: '注入点方法名。.cctor 表示静态构造函数，是最早执行的时机。',
    },
  },

  // ── 设置页 ──
  settings: {
    title: '设置',
    download: '下载',
    source: 'BepInEx 下载源',
    sourceDesc: '影响安装时的下载速度',
    sourceGithub: 'GitHub（官方）',
    sourceMirror: '镜像源',
    proxy: 'HTTP 代理',
    proxyPlaceholder: '留空则不使用代理（例如 http://127.0.0.1:7890）',
    general: '通用',
    language: '界面语言',
    about: '关于',
    version: '版本',
    stack: '技术栈',
    reset: '恢复默认',
  },
}
