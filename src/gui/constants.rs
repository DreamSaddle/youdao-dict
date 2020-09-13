
pub struct Constants {
}

impl Constants {

    pub fn application_width() -> i32 {
        600
    }

    pub fn application_height() -> i32 {
        400
    }

    pub fn application_name() -> String {
        String::from("YDict")
    }
    
    pub fn application_version() -> String {
        String::from("0.3.1")
    }

    pub fn application_icon() -> String {
        String::from("/usr/share/icons/hicolor/scalable/apps/YDict/desktop.png")
    }


    ///
    /// 系统托盘图标路径
    /// 
    pub fn system_tray_icon_path() -> String {
        String::from("/usr/share/icons/hicolor/scalable/apps/YDict/tray.png")
    }
    
    ///
    /// 喇叭图标路径
    /// 
    pub fn born_icon_path() -> String {
        String::from("/usr/share/icons/hicolor/scalable/apps/YDict/born.png")
    }

    pub fn clipboard_close_icon_path() -> String {
        String::from("/usr/share/icons/hicolor/scalable/apps/YDict/close.png")
    }
}