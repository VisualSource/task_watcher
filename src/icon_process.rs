use winapi::um::shellapi::{NOTIFYICONDATAW, NIF_MESSAGE, NIF_ICON, NIF_TIP, Shell_NotifyIconW, NIM_ADD, NIM_MODIFY, NIM_DELETE};
use winapi::um::winuser::{WM_APP, LoadIconW, IDI_APPLICATION};

use std::mem::{size_of, zeroed}; //get size of stuff and init with zeros
use std::ptr::null_mut; //use a null pointer (I think)
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use crate::icon_handler::TrayIcon;

pub struct TrayIconProcess {
    nid: NOTIFYICONDATAW,
    pid: String,
    pub status: bool
}

impl TrayIconProcess {
    pub fn kill(&mut self){
        unsafe{ Shell_NotifyIconW(NIM_DELETE, &mut self.nid) };
    }
    pub fn set_icon(&mut self, icon_buffer: &'static [u8]){

        let icon = match TrayIcon::load_icon(icon_buffer, None, None) {
            Ok(icon) => icon.hicon,
            Err(err)=>{
                eprintln!("Failed to load icon: {}",err);
                unsafe { LoadIconW(null_mut(),IDI_APPLICATION) }
            }
        };

        self.nid.hIcon = icon;

        unsafe{ Shell_NotifyIconW(NIM_MODIFY, &mut self.nid) }; //updates system tray icon
    }
    pub fn set_tooltip(&mut self, tip: String) {
        let mut tool_tip_int = [0; 128];
        let tool_tip_str_step: &str = &*tip; 
        let tool_tip_step_os = OsStr::new(tool_tip_str_step);
        let tool_tip_step_utf16 = tool_tip_step_os.encode_wide().collect::<Vec<u16>>(); //now actually convert to UTF16 format for the OS
        tool_tip_int[..tool_tip_step_utf16.len()].copy_from_slice(&tool_tip_step_utf16); //record it in that nice integer holder
        self.nid.szTip = tool_tip_int;
        unsafe{ Shell_NotifyIconW(NIM_MODIFY, &mut self.nid) }; //updates system tray icon

    }
    pub fn create(icon_buffer: &'static [u8], id: u32, pid: String) -> Result<TrayIconProcess,String>{
        // to navigate calling with the winapi "crate" use the search function at link
        // https://docs.rs/winapi/*/x86_64-pc-windows-msvc/winapi/um/wincon/fn.GetConsoleWindow.html
        let h_wnd = winapi::um::wincon::GetConsoleWindow;  //gets the current console window handle

        //System Tray Icon support - here it is
        let wm_mymessage = WM_APP + 100; //prep WM_MYMESSAGE
        let tooltip = "Server".to_string(); //record tooltip words for the icon
        let mut tooltip_int: [u16; 128] = [0; 128]; //fill with 0's
        let tooltip_step: &str = &*tooltip; //these two types of strings
        let tooltip_os = OsStr::new(tooltip_step); //convert to OS string format or something
        let tooltip_utf16 = tooltip_os.encode_wide().collect::<Vec<u16>>(); //now actually convert to UTF16 format for the OS
        tooltip_int[..tooltip_utf16.len()].copy_from_slice(&tooltip_utf16); //record it in that nice integer holder

        let icon = match TrayIcon::load_icon(icon_buffer, None, None) {
            Ok(icon) => icon.hicon,
            Err(err)=>{
                eprintln!("Failed to load icon: {}",err);
                unsafe { LoadIconW(null_mut(),IDI_APPLICATION) }
            }
        };

        let mut nid: NOTIFYICONDATAW = unsafe{ zeroed() }; //thing that has info on window and system tray stuff in it 
        unsafe
        {
            nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32; //prep
            nid.hWnd = h_wnd(); //links the console window
            nid.uID = id; //it's a number
            nid.uCallbackMessage = wm_mymessage ; //whoknows should be related to click capture but doesn't so
            nid.hIcon = icon;//winapi::um::winuser::LoadIconW(null_mut(), winapi::um::winuser::IDI_APPLICATION); //icon idk
            nid.szTip = tooltip_int; //tooltip for the icon
            nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP; //who knows
        };

        //let mut nidszTipLength = trayToolTipStepUTF16.len() as u64; //gets the size of nid.szTip (tooltip length) for the UTF-16 format, which is what Windows cares about

        unsafe{ Shell_NotifyIconW(NIM_ADD, &mut nid) }; //shows the icon

        Ok(TrayIconProcess{ nid: nid, pid, status: false })
    }
}