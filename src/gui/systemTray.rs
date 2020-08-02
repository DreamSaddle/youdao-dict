
use std::rc::Rc;

use cpp_core::{NullPtr, Ptr, StaticUpcast};
use qt_core::{QCoreApplication, qs, SlotNoArgs, QBox, QPtr, slot, QObject};
use qt_widgets::{QMainWindow, QAction, QMenu, QSystemTrayIcon, SlotOfActivationReason, q_system_tray_icon::ActivationReason};
use qt_gui::{QIcon};

use crate::gui::about::About;


pub struct SystemTray {
    mw: Ptr<QMainWindow>,
    pub systemTrayIcon: QBox<QSystemTrayIcon>,
    pub helpMenuAction: QBox<QAction>,
    pub trayMenu: QBox<QMenu>,
    pub helpAction: QPtr<QAction>,
    pub exitAction: QPtr<QAction>,
}

impl StaticUpcast<QObject> for SystemTray {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.systemTrayIcon.as_ptr().static_upcast()
    }
}


impl SystemTray {
    pub fn new(mw: Ptr<QMainWindow>) -> Rc<Self> {
        unsafe {
            let icon = QIcon::from_q_string(&qs("./media/tray_icon.png"));
            let trayIcon = QSystemTrayIcon::new();
            trayIcon.set_icon(&icon);
            trayIcon.set_tool_tip(&qs("Youdao Dict"));
            trayIcon.show();

            let action = QAction::from_q_string(&qs("关于"));

            let menu = QMenu::new();
            let help_action = menu.add_action_q_string(&qs("关于"));
            menu.add_separator();
            let exit_action = menu.add_action_q_string(&qs("退出"));

            action.set_menu(&menu);

            trayIcon.set_context_menu(&menu);

            let this = Rc::new(SystemTray {
                mw: mw,
                systemTrayIcon: trayIcon,
                helpMenuAction: action,
                trayMenu: menu,
                helpAction: help_action,
                exitAction: exit_action,
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        //托盘图标点击
        self.systemTrayIcon.activated().connect(&self.slot_on_tray_icon_activated());

        //帮助触发
        self.helpAction.triggered().connect(&self.slot_on_help_action_triggered());

        //退出程序
        self.exitAction.triggered().connect(&self.slot_on_exit_action_triggered());
    }
    

    ///
    /// 系统托盘图标激活事件
    /// ActivationReason#to_int()
    ///     0: 未知事件
    ///     1: context
    ///     2: 鼠标左键双击
    ///     3: 鼠标左键单击
    ///     4: 滚轮单击
    /// 
    #[slot(SlotOfActivationReason)]
    unsafe fn on_tray_icon_activated(
        self: &Rc<Self>,
        reason: ActivationReason
    ) {
        if reason.to_int() == 3 {
            //单击还原窗口
            self.mw.show();
        }
    }


    #[slot(SlotNoArgs)]
    unsafe fn on_help_action_triggered(self: &Rc<Self>) {
        About::show(self.mw);
    }


    #[slot(SlotNoArgs)]
    unsafe fn on_exit_action_triggered(self: &Rc<Self>) {
        QCoreApplication::exit_0a();
    }
}
