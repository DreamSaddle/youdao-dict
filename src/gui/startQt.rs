
use std::rc::Rc;

use cpp_core::{NullPtr, Ptr, StaticUpcast};
use qt_core::{qs, QBox, slot, SlotNoArgs, QObject};
use qt_widgets::{QApplication, QWidget, QMainWindow, QVBoxLayout, q_box_layout::Direction};

use crate::gui::{
    systemTray::SystemTray,
    appMenu::AppMenu,
    opt::optLine::{OptLine},
    text::transText::{TransText}
};

#[derive(Debug)]
struct MainWindowWidgets {
    transText: Rc<TransText>,
    optLine: Rc<OptLine>,
}

#[derive(Debug)]
pub struct StartQt {
    mw: QBox<QMainWindow>,
    mww: Rc<MainWindowWidgets>,
}


impl StaticUpcast<QObject> for StartQt {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.mw.as_ptr().static_upcast()
    }
}

impl StartQt {
    pub fn start() {
        
        unsafe {
            QApplication::init(|_| {
                let mainWindow: QBox<QMainWindow> = QMainWindow::new_0a();
                let windowWidget = QWidget::new_1a(&mainWindow);
        
                let _mw_widgets = initWindowWidgets(&windowWidget);
                let _systemTray = SystemTray::new(mainWindow.as_ptr());
                let _appMenu = AppMenu::new(mainWindow.as_ptr());
        
                mainWindow.set_fixed_size_2a(700, 200);
                windowWidget.resize_1a(&mainWindow.size());
                mainWindow.set_window_title(&qs("Youdao Dict"));
                mainWindow.show();
                let this = Rc::new(StartQt{
                    mw: mainWindow,
                    mww: _mw_widgets
                });
                this.init_slots();
                QApplication::exec()
            });
        }
    }

    

    unsafe fn init_slots(self: &Rc<Self>) {
        //根据翻译词切换对于的翻译类型
        self.mww.transText.sourceEdit.text_changed().connect(&self.slot_on_source_edit_text_changed());

    }



    #[slot(SlotNoArgs)]
    pub unsafe fn on_source_edit_text_changed(self: &Rc<Self>) {
        OptLine::slot_change_lang_select_actived(&self.mww.optLine);
    }
}

///
/// 初始化主窗口组件
/// 
unsafe fn initWindowWidgets(windowWidget: &QBox<QWidget>) -> Rc<MainWindowWidgets> {
    let vBox = QVBoxLayout::new_1a(windowWidget);
    vBox.set_contents_margins_4a(0, 0, 0, 0);
    vBox.set_spacing(2);
    vBox.set_direction(Direction::Down);

    //初始化输入框
    let transText = TransText::new(&vBox);
    
    //初始化操作栏
    let _optLine = OptLine::new(&vBox, &transText);

    let mw = Rc::new(MainWindowWidgets{
        transText: transText,
        optLine: _optLine
    });
    mw
}