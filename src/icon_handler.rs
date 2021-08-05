use winapi::shared::minwindef::PBYTE;
use winapi::um::winuser::{LookupIconIdFromDirectoryEx,LR_DEFAULTCOLOR,CreateIconFromResourceEx};
use winapi::shared::windef::HICON;

pub struct TrayIcon {
    pub hicon: HICON
}

impl TrayIcon {
    pub fn load_icon(buffer: &'static [u8], width: Option<u32>, height: Option<u32>) -> Result<TrayIcon,String> {
        let offset = unsafe { 
            LookupIconIdFromDirectoryEx(buffer.as_ptr() as PBYTE, 1, 
            width.unwrap_or_default() as i32, 
            height.unwrap_or_default() as i32, 
            LR_DEFAULTCOLOR
        )};

        if offset <= 0 {
            return Err(String::from("Icon Loading Failed"));
        }

        let icon_data = &buffer[offset as usize..];

        let hicon = unsafe {
            CreateIconFromResourceEx(
                icon_data.as_ptr() as PBYTE, 
                icon_data.len() as u32, 
                1, 
                0x30000, 
                width.unwrap_or_default() as i32, 
                height.unwrap_or_default() as i32, 
                LR_DEFAULTCOLOR,)
        };

        if hicon.is_null() {
            return Err(String::from("Failed to create icon from resource"));
        }

        Ok(TrayIcon{ hicon: hicon})
    }
}
