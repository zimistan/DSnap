// 定义截图错误类型
#[derive(Debug)]
pub struct ScreenshotError(String);

impl std::fmt::Display for ScreenshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Screenshot error: {}", self.0)
    }
}

// 定义截图 trait
pub trait Screenshot {
    fn capture_screen(&self) -> Result<Vec<u8>, ScreenshotError>;
    fn capture_area(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, ScreenshotError>;
}

// 声明linux模块
#[cfg(target_os = "linux")]
pub mod linux;

// 平台特定的实现选择
pub fn create_screenshot() -> Box<dyn Screenshot> {
    #[cfg(target_os = "linux")]
    {
        use crate::utils::displayServer::{detect_display_server, DisplayServer};

        match detect_display_server() {
            Ok(DisplayServer::Wayland) => {
                use crate::screenshot::linux::wayland::WaylandScreenshot;
                return Box::new(WaylandScreenshot);
            }
            Ok(DisplayServer::X11) => {
                use crate::screenshot::linux::x11::X11Screenshot;
                return Box::new(X11Screenshot);
            }
            Err(e) => {
                panic!("无法检测显示服务器: {}", e);
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        panic!("当前平台不支持截图功能");
    }
}
