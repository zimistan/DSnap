pub mod screenshot;
pub mod utils;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn capture_screenshot() -> Result<String, String> {
    use  crate::screenshot::create_screenshot;
    let screenshot = create_screenshot();
    match screenshot.capture_screen() {
        Ok(image_data) => {
            println!("Screenshot captured successfully, size: {} bytes", image_data.len());
            // 处理图像数据...
        }
        Err(e) => {
            eprintln!("Failed to capture screenshot: {}", e);
        }
    }
    // Return base64 so frontend can display.
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine as _;
    Ok(STANDARD.encode("123"))
}

#[tauri::command]
async fn create_overlay_window(app: tauri::AppHandle) -> Result<(), String> {
    let _window = tauri::webview::WebviewWindowBuilder::new(
        &app,
        "overlay",
        tauri::WebviewUrl::App("/#/overlay".into()),
    )
    // TODO: 添加窗口大小
    .title("Overlay")
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .build()
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            capture_screenshot,
            create_overlay_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
