
use std::rc::Rc;

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{qs, QBox, QObject};
use qt_widgets::{QApplication, QWidget, QMainWindow, QVBoxLayout, q_box_layout::Direction, QScrollArea};

use crate::gui::{
    systemTray::SystemTray,
    appMenu::AppMenu,
    opt::optLine::{OptLine},
    text::transText::{TransText},
    result::transResult::{TransResult},
};

#[derive(Debug)]
pub struct MainWindowWidgets {
    pub transText: Rc<TransText>,
    pub optLine: Rc<OptLine>,
    pub transResult: Rc<TransResult>
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
                let sa = QScrollArea::new_1a(&mainWindow);
                let windowWidget = QWidget::new_0a();
        
                let _mw_widgets = initWindowWidgets(&windowWidget);
                let _systemTray = SystemTray::new(mainWindow.as_ptr());
                let _appMenu = AppMenu::new(mainWindow.as_ptr());
        
                sa.set_widget_resizable(true);
                sa.set_widget(&windowWidget);
                mainWindow.set_fixed_size_2a(700, 300);
                sa.resize_1a(&mainWindow.size());
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

    
    ///
    /// 统一插槽初始化
    /// 
    unsafe fn init_slots(self: &Rc<Self>) {
        OptLine::init_slots(&self.mww);
        TransText::init_slots(&self.mww);
    }
}


///
/// 初始化主窗口组件
/// 
unsafe fn initWindowWidgets(windowWidget: &QBox<QWidget>) -> Rc<MainWindowWidgets> {
    let v_box = QVBoxLayout::new_1a(windowWidget);
    v_box.set_contents_margins_4a(0, 0, 0, 0);
    // v_box.set_spacing(2);
    v_box.set_direction(Direction::TopToBottom);

    //初始化操作栏
    let _optLine = OptLine::new(&v_box);

    //初始化输入框
    let _transText = TransText::new(&v_box);

    //翻译结果
    let _transResult = TransResult::new(&v_box);

    let mw = Rc::new(MainWindowWidgets{
        transText: _transText,
        optLine: _optLine,
        transResult: _transResult,
    });
    mw
}