
///
/// 基础翻译结果, 发音栏
/// 

use std::rc::Rc;

use cpp_core::{NullPtr, Ptr, StaticUpcast};
use qt_core::{qs, SlotNoArgs, QBox, slot, QObject, QFlags, AlignmentFlag, TextInteractionFlag};
use qt_widgets::{QWidget, QVBoxLayout, QHBoxLayout, QLabel, q_box_layout::Direction, QLayoutItem, QPushButton};
use qt_gui::{QIcon};

use crate::gui::{
    startQt::{MainWindowWidgets},
    result::transResult::{TransResult},
    constants::Constants,
    runtimeState::LastOptTime,
};

use crate::utils::{
    resultParse::{
        request_result_parse_en_to_zh,
        request_result_parse_zh_to_en
    },
    audio::{play_phonogram},
    datetime::{current_timestamp_millis}
};
use crate::structs::zhConciseInfo::ZhConciseInfo;
use crate::structs::engConciseInfo::EngConciseInfo;


#[derive(Debug)]
pub struct Pronounce {
    pub widget: QBox<QWidget>,
    //当前翻译词
    transWordLabel: QBox<QLabel>,
    //翻译结果行
    reusltLineBox: QBox<QVBoxLayout>,
    //中译英 发音行
    hornWidget: QBox<QWidget>,
    ukPWidget: QBox<QWidget>,
    ukPButton: QBox<QPushButton>,
    //英式音标
    ukPLabel: QBox<QLabel>,
    usPWidget: QBox<QWidget>,
    usPButton: QBox<QPushButton>,
    //美式音标
    usPLabel: QBox<QLabel>,
    //英语其他时态对应词汇
    wfsLabel: QBox<QLabel>,
}


impl StaticUpcast<QObject> for Pronounce {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl Pronounce {
    pub fn new() -> Rc<Self> {
        unsafe {
            let widget = QWidget::new_0a();
            let v_box = QVBoxLayout::new_1a(&widget);

            let trans_word_label: QBox<QLabel> = QLabel::new();
            trans_word_label.set_text(&qs(""));
            trans_word_label.set_style_sheet(&qs("font-size:20px;font-weight:bold;color:#D81159"));
            trans_word_label.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
            v_box.add_widget_3a(&trans_word_label, 0, QFlags::from(AlignmentFlag::AlignTop));

            //发音喇叭图标
            let icon = QIcon::from_q_string(&qs(Constants::born_icon_path()));

            let horn_widget = QWidget::new_0a();
            horn_widget.resize_2a(300, 30);
            let horn_hbox = QHBoxLayout::new_1a(&horn_widget);
            horn_hbox.set_direction(Direction::LeftToRight);

            //英式发音
            let uk_p_widget = QWidget::new_0a();
            let uk_p_hbox = QHBoxLayout::new_1a(&uk_p_widget);
            let uk_p_label = QLabel::new();
            uk_p_label.set_text(&qs("英 []"));
            uk_p_label.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
            uk_p_hbox.add_widget(&uk_p_label);
            //发音喇叭按钮
            let uk_p_button = QPushButton::new();
            uk_p_button.set_icon(&icon);
            uk_p_button.resize_2a(10, 10);
            uk_p_button.set_style_sheet(&qs("QPushButton{border-radius:10px;border-width:0px;}"));
            uk_p_hbox.add_widget(&uk_p_button);

            //美式发音
            let us_p_widget = QWidget::new_0a();
            let us_p_hbox = QHBoxLayout::new_1a(&us_p_widget);
            let us_p_label = QLabel::new();
            us_p_label.set_text(&qs("美 []"));
            us_p_label.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
            us_p_hbox.add_widget(&us_p_label);
            //发音喇叭按钮
            let us_p_button = QPushButton::new();
            us_p_button.set_style_sheet(&qs("QPushButton{border-radius:10px;border-width:0px;}"));
            us_p_button.set_icon(&icon);
            us_p_hbox.add_widget(&us_p_button);

            horn_hbox.add_widget(&uk_p_widget);
            horn_hbox.add_spacing(10);
            horn_hbox.add_widget(&us_p_widget);
            horn_hbox.add_stretch_0a();

            //默认隐藏
            horn_widget.hide();
            v_box.add_widget_3a(&horn_widget, 0, QFlags::from(AlignmentFlag::AlignTop));

            //翻译结果行区域
            let result_widget = QWidget::new_0a();
            // result_widget.set_minimum_height(10);
            let result_v_box = QVBoxLayout::new_1a(&result_widget);
            result_v_box.set_direction(Direction::TopToBottom);
            result_v_box.set_spacing(1);

            v_box.add_widget(&result_widget);

            let wfs_label: QBox<QLabel> = QLabel::new();
            wfs_label.set_text(&qs(""));
            wfs_label.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
            wfs_label.set_style_sheet(&qs("font-size:12px;"));
            v_box.add_widget(&wfs_label);

            let this = Rc::new(Pronounce{
                widget: widget,
                transWordLabel: trans_word_label,
                reusltLineBox: result_v_box,
                hornWidget: horn_widget,
                ukPWidget: uk_p_widget,
                ukPLabel: uk_p_label,
                ukPButton: uk_p_button,
                usPWidget: us_p_widget,
                usPLabel: us_p_label,
                usPButton: us_p_button,
                wfsLabel: wfs_label,
            });
            this.init();
            this
        }
    }
    
    ///
    /// init
    /// 
    unsafe fn init(self: &Rc<Self>) {
        self.ukPButton.clicked().connect(&self.slot_on_uk_pronounce_button_clicked());
        self.usPButton.clicked().connect(&self.slot_on_us_pronounce_button_clicked());
        
    }

    ///
    /// 英式音标发音执行
    /// 
    #[slot(SlotNoArgs)]
    unsafe fn on_uk_pronounce_button_clicked(self: &Rc<Self>) {
        {
            let b = LastOptTime::get_instance();
            let _lastBornTime = b.clone().lock().unwrap().lastUkLastOptTime;
            if _lastBornTime == -1 || current_timestamp_millis() - _lastBornTime > 1000 {
                play_phonogram(&self.transWordLabel.text().to_std_string(), 2);
            }
        }
        //上面需要提入到 {}代码段中, 否则刷新最后一次发音时间会造成死锁问题
        LastOptTime::refresh_last_uk_born_time();
    }


    ///
    /// 美式音标发音执行
    /// 
    #[slot(SlotNoArgs)]
    unsafe fn on_us_pronounce_button_clicked(self: &Rc<Self>) {
        {
            let b = LastOptTime::get_instance();
            let _lastBornTime = b.clone().lock().unwrap().lastUsLastOptTime;
            if _lastBornTime == -1 || current_timestamp_millis() - _lastBornTime > 1000 {
                play_phonogram(&self.transWordLabel.text().to_std_string(), 1);
            }
        }
        LastOptTime::refresh_last_us_born_time();
    }


    ///
    /// 英译中结果填充
    /// 
    pub unsafe fn full_en_to_zh_trans_result(self: &Rc<Self>, obj: &EngConciseInfo, mww: &Rc<MainWindowWidgets>) {
        self.clear_all_result_content();
        
        //解析结果
        let parse_result = request_result_parse_en_to_zh(obj);

        self.hornWidget.show();
        if !parse_result.ukphone.is_empty() {
            self.ukPLabel.set_text(&qs(format!("英 [{}]", parse_result.ukphone)));
            self.ukPWidget.show();
        } else {
            self.ukPWidget.hide()
        }
        if !parse_result.usphone.is_empty() {
            self.usPLabel.set_text(&qs(format!("美 [{}]", parse_result.usphone)));
            self.usPWidget.show();
        } else {
            self.usPWidget.hide();
        }

        let zh_lines: Vec<String> = parse_result.zhLines;
        if !zh_lines.is_empty() {
            if obj.input.is_some() {
                self.transWordLabel.set_text(&qs(obj.input.as_ref().unwrap()));
            }

            for zh_line in zh_lines.iter() {
                let label: QBox<QLabel> = QLabel::new();
                label.set_text(&qs(zh_line));
                label.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
                label.set_style_sheet(&qs("font-size:13px;"));
                self.reusltLineBox.add_widget_3a(&label, 0, QFlags::from(AlignmentFlag::AlignTop));
                
            }
            self.wfsLabel.set_text(&qs(parse_result.wfs));
            TransResult::show(&mww.transResult);
        } else {
            TransResult::hide(&mww.transResult);
        }
    }

    ///
    /// 中译英结果填充
    /// 
    pub unsafe fn full_zh_to_en_trans_result(self: &Rc<Self>, obj: &ZhConciseInfo, mww: &Rc<MainWindowWidgets>) {
        self.clear_all_result_content();
        let zh_lines: Vec<String> = request_result_parse_zh_to_en(obj);

        //隐藏发音栏
        self.hornWidget.hide();
        
        if !zh_lines.is_empty() {
            if obj.input.is_some() {
                self.transWordLabel.set_text(&qs(obj.input.as_ref().unwrap()));
            }

            for zh_line in zh_lines.iter() {
                let label: QBox<QLabel> = QLabel::new();
                label.set_text(&qs(zh_line));
                label.set_text_interaction_flags(QFlags::from(TextInteractionFlag::TextSelectableByMouse));
                label.set_style_sheet(&qs("font-size:13px;"));
                self.reusltLineBox.add_widget_3a(&label, 0, QFlags::from(AlignmentFlag::AlignTop));
                
            }
            TransResult::show(&mww.transResult);
        } else {
            TransResult::hide(&mww.transResult);
        }
    }


    ///
    /// 清空上一次翻译结果
    /// 
    unsafe fn clear_all_result_content(self: &Rc<Self>) {
        self.wfsLabel.set_text(&qs(""));
        self.ukPLabel.set_text(&qs(""));
        self.usPLabel.set_text(&qs(""));
        self.remove_reuslt_lines();
    }
    
    ///
    /// 原内容清空
    /// 
    unsafe fn remove_reuslt_lines(self: &Rc<Self>) {
        let count = self.reusltLineBox.count();
        if count == 0 {
            return
        }
        loop {
            let item: Ptr<QLayoutItem> = self.reusltLineBox.take_at(0);
            if item.is_null() {
                break;
            }
            item.to_box().unwrap().widget().set_parent(NullPtr);
            // self.reusltLineBox.remove_widget(item.to_box().unwrap().widget());
        }
    }
}