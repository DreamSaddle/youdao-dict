
use std::rc::Rc;

use cpp_core::{NullPtr, Ptr, StaticUpcast};
use qt_core::{QCoreApplication, QPtr,qs, SlotNoArgs, QBox, QObject, slot};
use qt_widgets::{QMainWindow, QMenuBar, QMenu, QAction};

use crate::gui::{
    about::About,
    usage::Usage
};

pub struct AppMenu {
    mw: Ptr<QMainWindow>,
    pub helpAction: QBox<QAction>,
    pub helpMenu: QBox<QMenu>,
    pub exitAction: QPtr<QAction>,
    pub aboutAction: QPtr<QAction>,
    pub usageAction: QPtr<QAction>,
}


impl StaticUpcast<QObject> for AppMenu {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.helpMenu.as_ptr().static_upcast()
    }
}


impl AppMenu {
    pub fn new(mw: Ptr<QMainWindow>) -> Rc<Self> {
        unsafe {
            let menu_bar = QMenuBar::new_0a();

            let help_action = QAction::from_q_string(&qs("帮助"));
            let help_menu = QMenu::new();

            let about_action = help_menu.add_action_q_string(&qs("关于"));
            let usage_action = help_menu.add_action_q_string(&qs("如何使用"));
            help_menu.add_separator();
            let exit_action = help_menu.add_action_q_string(&qs("退出"));

            help_action.set_menu(&help_menu);
            menu_bar.add_menu_q_menu(&help_menu);
            mw.set_menu_bar(&menu_bar);

            let this = Rc::new(AppMenu {
                mw: mw,
                helpAction: help_action,
                helpMenu: help_menu,
                aboutAction: about_action,
                exitAction: exit_action,
                usageAction: usage_action
            });
            this.init();
            this
        }
    }


    unsafe fn init(self: &Rc<Self>) {
        //关于
        self.aboutAction.triggered().connect(&self.slot_on_help_action_triggered());

        //使用
        self.usageAction.triggered().connect(&self.slot_on_usage_action_triggered());

        //退出程序
        self.exitAction.triggered().connect(&self.slot_on_exit_action_triggered());
    }


    #[slot(SlotNoArgs)]
    unsafe fn on_help_action_triggered(self: &Rc<Self>) {
        About::show(self.mw);
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_usage_action_triggered(self: &Rc<Self>) {
        Usage::show(self.mw);
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_exit_action_triggered(self: &Rc<Self>) {
        QCoreApplication::exit_0a();
    }
}