//! Module pour la capture de fenêtres Windows
//! Permet de lister les fenêtres ouvertes et de capturer leur contenu

use base64::{engine::general_purpose::STANDARD, Engine};
use image::{ImageBuffer, Rgba};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use tauri::command;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, RECT, TRUE},
    Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject,
        GetDC, GetDIBits, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER,
        BI_RGB, DIB_RGB_COLORS, SRCCOPY,
    },
    UI::WindowsAndMessaging::{
        EnumWindows, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
        IsWindowVisible,
    },
};

/// Liste toutes les fenêtres visibles avec leur titre
#[command]
pub fn list_windows() -> Result<Vec<String>, String> {
    #[cfg(target_os = "windows")]
    {
        let mut windows: Vec<String> = Vec::new();

        unsafe {
            let _ = EnumWindows(
                Some(enum_windows_callback),
                LPARAM(&mut windows as *mut Vec<String> as isize),
            );
        }

        // Filtrer les fenêtres sans titre et trier
        windows.retain(|s| !s.is_empty() && s.len() > 1);
        windows.sort();
        windows.dedup();

        Ok(windows)
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Cette fonctionnalité n'est disponible que sur Windows".to_string())
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // Vérifier si la fenêtre est visible
    if !IsWindowVisible(hwnd).as_bool() {
        return TRUE;
    }

    // Obtenir le titre de la fenêtre
    let length = GetWindowTextLengthW(hwnd);
    if length == 0 {
        return TRUE;
    }

    let mut buffer: Vec<u16> = vec![0; (length + 1) as usize];
    let copied = GetWindowTextW(hwnd, &mut buffer);

    if copied > 0 {
        buffer.truncate(copied as usize);
        let title = OsString::from_wide(&buffer)
            .to_string_lossy()
            .to_string();

        if !title.is_empty() {
            let windows = &mut *(lparam.0 as *mut Vec<String>);
            windows.push(title);
        }
    }

    TRUE
}

/// Capture une fenêtre par son titre et retourne l'image en base64
#[command]
pub fn capture_window(window_title: String) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            // Trouver la fenêtre par son titre
            let hwnd = find_window_by_title(&window_title)
                .ok_or_else(|| format!("Fenêtre '{}' non trouvée", window_title))?;

            // Obtenir les dimensions de la fenêtre
            let mut rect = RECT::default();
            if GetWindowRect(hwnd, &mut rect).is_err() {
                return Err("Impossible d'obtenir les dimensions de la fenêtre".to_string());
            }

            let width = (rect.right - rect.left) as i32;
            let height = (rect.bottom - rect.top) as i32;

            if width <= 0 || height <= 0 {
                return Err("Dimensions de fenêtre invalides".to_string());
            }

            // Créer un DC compatible
            let hdc_screen = GetDC(hwnd);
            if hdc_screen.is_invalid() {
                return Err("Impossible d'obtenir le DC de la fenêtre".to_string());
            }

            let hdc_mem = CreateCompatibleDC(hdc_screen);
            if hdc_mem.is_invalid() {
                ReleaseDC(hwnd, hdc_screen);
                return Err("Impossible de créer le DC mémoire".to_string());
            }

            // Créer un bitmap compatible
            let hbitmap = CreateCompatibleBitmap(hdc_screen, width, height);
            if hbitmap.is_invalid() {
                let _ = DeleteDC(hdc_mem);
                ReleaseDC(hwnd, hdc_screen);
                return Err("Impossible de créer le bitmap".to_string());
            }

            let old_bitmap = SelectObject(hdc_mem, hbitmap);

            // Capturer le contenu de la fenêtre via BitBlt
            let blt_result = BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, 0, 0, SRCCOPY);
            if blt_result.is_err() {
                SelectObject(hdc_mem, old_bitmap);
                let _ = DeleteObject(hbitmap);
                let _ = DeleteDC(hdc_mem);
                ReleaseDC(hwnd, hdc_screen);
                return Err("Impossible de capturer la fenêtre".to_string());
            }

            // Préparer la structure BITMAPINFO
            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height, // Négatif pour un bitmap top-down
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [Default::default()],
            };

            // Allouer le buffer pour les pixels
            let mut pixels: Vec<u8> = vec![0; (width * height * 4) as usize];

            let result = GetDIBits(
                hdc_mem,
                hbitmap,
                0,
                height as u32,
                Some(pixels.as_mut_ptr() as *mut _),
                &mut bmi,
                DIB_RGB_COLORS,
            );

            // Nettoyer les ressources GDI
            SelectObject(hdc_mem, old_bitmap);
            let _ = DeleteObject(hbitmap);
            let _ = DeleteDC(hdc_mem);
            ReleaseDC(hwnd, hdc_screen);

            if result == 0 {
                return Err("Impossible de récupérer les pixels".to_string());
            }

            // Convertir BGRA en RGBA
            for chunk in pixels.chunks_exact_mut(4) {
                chunk.swap(0, 2); // Swap B et R
            }

            // Créer l'image
            let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
                ImageBuffer::from_raw(width as u32, height as u32, pixels)
                    .ok_or("Impossible de créer l'image")?;

            // Encoder en PNG puis en base64
            let mut png_data: Vec<u8> = Vec::new();
            let encoder = image::codecs::png::PngEncoder::new(&mut png_data);

            img.write_with_encoder(encoder)
                .map_err(|e| format!("Erreur encodage PNG: {}", e))?;

            let base64_str = STANDARD.encode(&png_data);

            Ok(base64_str)
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Cette fonctionnalité n'est disponible que sur Windows".to_string())
    }
}

#[cfg(target_os = "windows")]
unsafe fn find_window_by_title(title: &str) -> Option<HWND> {
    struct SearchData {
        title: String,
        hwnd: Option<HWND>,
    }

    let mut data = SearchData {
        title: title.to_lowercase(),
        hwnd: None,
    };

    unsafe extern "system" fn search_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let data = &mut *(lparam.0 as *mut SearchData);

        if !IsWindowVisible(hwnd).as_bool() {
            return TRUE;
        }

        let length = GetWindowTextLengthW(hwnd);
        if length == 0 {
            return TRUE;
        }

        let mut buffer: Vec<u16> = vec![0; (length + 1) as usize];
        let copied = GetWindowTextW(hwnd, &mut buffer);

        if copied > 0 {
            buffer.truncate(copied as usize);
            let window_title = OsString::from_wide(&buffer)
                .to_string_lossy()
                .to_lowercase();

            if window_title.contains(&data.title) {
                data.hwnd = Some(hwnd);
                return BOOL(0); // Arrêter l'énumération
            }
        }

        TRUE
    }

    let _ = EnumWindows(
        Some(search_callback),
        LPARAM(&mut data as *mut SearchData as isize),
    );

    data.hwnd
}
