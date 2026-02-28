use std::path::PathBuf;
use serde::Serialize;

#[derive(Serialize)]
pub struct GameValidation {
    pub valid: bool,
    pub has_data_folder: bool,
    pub has_exe: bool,
    pub message: String,
}

/// 验证游戏目录是否为 Unity 游戏
#[tauri::command]
pub fn validate_game_dir(path: String) -> GameValidation {
    let dir = PathBuf::from(&path);

    if !dir.is_dir() {
        return GameValidation {
            valid: false, has_data_folder: false, has_exe: false,
            message: "路径不存在或不是目录".into(),
        };
    }

    let has_exe = std::fs::read_dir(&dir)
        .ok()
        .map(|rd| rd.flatten().any(|e| {
            e.path().extension().map(|x| x == "exe").unwrap_or(false)
        }))
        .unwrap_or(false);

    let has_data_folder = std::fs::read_dir(&dir)
        .ok()
        .map(|rd| rd.flatten().any(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            e.path().is_dir() && name.ends_with("_Data")
        }))
        .unwrap_or(false);

    let valid = has_exe;
    let message = if !has_exe {
        "未找到可执行文件（.exe），请确认目录正确".into()
    } else if !has_data_folder {
        "未找到 Unity _Data 文件夹，可能不是 Unity 游戏，但仍可继续".into()
    } else {
        "验证通过，检测到 Unity 游戏目录".into()
    };

    GameValidation { valid, has_data_folder, has_exe, message }
}

/// 从 exe 文件提取图标，返回 PNG base64 字符串
/// 仅在 Windows 上可用
#[tauri::command]
pub fn extract_exe_icon(exe_path: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        extract_icon_windows(&exe_path)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = exe_path;
        Err("仅 Windows 支持图标提取".into())
    }
}

#[cfg(target_os = "windows")]
fn extract_icon_windows(exe_path: &str) -> Result<String, String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    // 将路径转为 UTF-16
    let wide: Vec<u16> = OsStr::new(exe_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        // 加载 shell32 / user32
        let hicon = extract_icon_handle(&wide)?;
        let png_bytes = icon_to_png(hicon)?;

        // 销毁 HICON
        windows_destroy_icon(hicon);

        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &png_bytes,
        );
        Ok(format!("data:image/png;base64,{}", encoded))
    }
}

#[cfg(target_os = "windows")]
unsafe fn extract_icon_handle(wide_path: &[u16]) -> Result<*mut std::ffi::c_void, String> {
    #[link(name = "shell32")]
    extern "system" {
        fn ExtractIconExW(
            lpszFile: *const u16,
            nIconIndex: i32,
            phiconLarge: *mut *mut std::ffi::c_void,
            phiconSmall: *mut *mut std::ffi::c_void,
            nIcons: u32,
        ) -> u32;
    }

    let mut hicon_large: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut hicon_small: *mut std::ffi::c_void = std::ptr::null_mut();

    let count = ExtractIconExW(
        wide_path.as_ptr(),
        0,
        &mut hicon_large,
        &mut hicon_small,
        1,
    );

    if count == 0 || hicon_large.is_null() {
        // 尝试小图标
        if !hicon_small.is_null() {
            return Ok(hicon_small);
        }
        return Err("无法从 exe 提取图标".into());
    }

    // 释放小图标
    if !hicon_small.is_null() {
        windows_destroy_icon(hicon_small);
    }

    Ok(hicon_large)
}

#[cfg(target_os = "windows")]
unsafe fn icon_to_png(hicon: *mut std::ffi::c_void) -> Result<Vec<u8>, String> {
    #[repr(C)]
    struct ICONINFO {
        f_icon: i32,
        x_hotspot: u32,
        y_hotspot: u32,
        hbm_mask: *mut std::ffi::c_void,
        hbm_color: *mut std::ffi::c_void,
    }

    #[repr(C)]
    struct BITMAP {
        bm_type: i32,
        bm_width: i32,
        bm_height: i32,
        bm_width_bytes: i32,
        bm_planes: u16,
        bm_bits_pixel: u16,
        bm_bits: *mut std::ffi::c_void,
    }

    #[repr(C)]
    struct BITMAPINFOHEADER {
        bi_size: u32,
        bi_width: i32,
        bi_height: i32,
        bi_planes: u16,
        bi_bit_count: u16,
        bi_compression: u32,
        bi_size_image: u32,
        bi_x_pels_per_meter: i32,
        bi_y_pels_per_meter: i32,
        bi_clr_used: u32,
        bi_clr_important: u32,
    }

    #[link(name = "user32")]
    extern "system" {
        fn GetIconInfo(hIcon: *mut std::ffi::c_void, pIconInfo: *mut ICONINFO) -> i32;
    }
    #[link(name = "gdi32")]
    extern "system" {
        fn GetObjectW(h: *mut std::ffi::c_void, c: i32, pv: *mut std::ffi::c_void) -> i32;
        fn GetDC(hwnd: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
        fn ReleaseDC(hwnd: *mut std::ffi::c_void, hdc: *mut std::ffi::c_void) -> i32;
        fn GetDIBits(
            hdc: *mut std::ffi::c_void,
            hbm: *mut std::ffi::c_void,
            start: u32,
            lines: u32,
            bits: *mut std::ffi::c_void,
            bmi: *mut BITMAPINFOHEADER,
            usage: u32,
        ) -> i32;
        fn DeleteObject(ho: *mut std::ffi::c_void) -> i32;
    }

    let mut icon_info = ICONINFO {
        f_icon: 0, x_hotspot: 0, y_hotspot: 0,
        hbm_mask: std::ptr::null_mut(),
        hbm_color: std::ptr::null_mut(),
    };
    if GetIconInfo(hicon, &mut icon_info) == 0 {
        return Err("GetIconInfo 失败".into());
    }

    // 获取位图尺寸
    let mut bmp: BITMAP = std::mem::zeroed();
    let bm_ptr = if !icon_info.hbm_color.is_null() {
        icon_info.hbm_color
    } else {
        icon_info.hbm_mask
    };
    GetObjectW(bm_ptr, std::mem::size_of::<BITMAP>() as i32, &mut bmp as *mut _ as _);

    let w = bmp.bm_width.unsigned_abs();
    let h = bmp.bm_height.unsigned_abs();

    // 读取 BGRA 像素数据
    let mut bmi = BITMAPINFOHEADER {
        bi_size: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        bi_width: w as i32,
        bi_height: -(h as i32), // 负值=从上到下
        bi_planes: 1,
        bi_bit_count: 32,
        bi_compression: 0, // BI_RGB
        bi_size_image: 0,
        bi_x_pels_per_meter: 0,
        bi_y_pels_per_meter: 0,
        bi_clr_used: 0,
        bi_clr_important: 0,
    };

    let hdc = GetDC(std::ptr::null_mut());
    let mut pixels = vec![0u8; (w * h * 4) as usize];
    GetDIBits(hdc, bm_ptr, 0, h, pixels.as_mut_ptr() as _, &mut bmi, 0);
    ReleaseDC(std::ptr::null_mut(), hdc);

    // 清理 GDI 对象
    if !icon_info.hbm_color.is_null() { DeleteObject(icon_info.hbm_color); }
    if !icon_info.hbm_mask.is_null()  { DeleteObject(icon_info.hbm_mask); }

    // BGRA → RGBA
    for chunk in pixels.chunks_mut(4) {
        chunk.swap(0, 2); // B <-> R
    }

    // 编码为 PNG
    let img = image::RgbaImage::from_raw(w, h, pixels)
        .ok_or("图像数据无效")?;
    let mut buf = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut buf),
        image::ImageFormat::Png,
    ).map_err(|e| e.to_string())?;

    Ok(buf)
}

#[cfg(target_os = "windows")]
unsafe fn windows_destroy_icon(hicon: *mut std::ffi::c_void) {
    #[link(name = "user32")]
    extern "system" {
        fn DestroyIcon(hIcon: *mut std::ffi::c_void) -> i32;
    }
    DestroyIcon(hicon);
}
