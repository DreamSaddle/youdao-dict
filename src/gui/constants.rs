
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
        String::from("Youdao Dict")
    }

    pub fn application_icon() -> String {
        String::from("/usr/share/icons/hicolor/scalable/apps/youdao-dict-desktop.png")
    }


    ///
    /// 系统托盘图标路径
    /// 
    pub fn system_tray_icon_path() -> String {
        String::from("/usr/share/icons/hicolor/scalable/apps/youdao-dict-tray.png")
    }
    
    ///
    /// 喇叭图标路径
    /// 
    pub fn born_icon_path() -> String {
        String::from("/usr/share/icons/hicolor/scalable/apps/youdao-dict-born.png")
    }    
}