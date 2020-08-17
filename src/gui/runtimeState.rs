///
/// 运行时状态
/// 

use std::sync::Arc;
use std::sync::Mutex;

use crate::utils::datetime::{current_timestamp_millis};

#[derive(Debug)]
pub struct RunTimeState {
    
}

#[derive(Copy, Clone, Debug)]
pub struct LastOptTime {
    // 翻译按钮最后一次点击时间
    pub lastTranOptTime: i64,
    // 英式音标发音最后一次触发时间
    pub lastUkLastOptTime: i64,
    // 美式音标发音最后一次触发时间
    pub lastUsLastOptTime: i64,
}


impl LastOptTime {

    ///
    /// 单例维护
    /// 使用 Mutex 一定要注意可能会造成死锁问题
    /// Rust 在一个 {}代码段中使用 lock()后, 退出该代码段时会释放锁
    /// 如果在一个 {}代码段中使用多次 lock() 就会造成死锁
    /// 
    pub fn get_instance() -> Arc<Mutex<LastOptTime>> {
		static mut POINT: Option<Arc<Mutex<LastOptTime>>> = None;

        unsafe {
            POINT.get_or_insert_with(|| {
                Arc::new(Mutex::new(LastOptTime::new()))
            }).clone()
        }
    }
    
    fn new() -> LastOptTime {
        LastOptTime {
            lastUkLastOptTime: -1,
            lastUsLastOptTime: -1,
            lastTranOptTime: -1
        }
    }

    ///
    /// 刷新最后一次执行翻译时间为当前时间
    /// 
    pub fn refresh_last_tran_time() {
        LastOptTime::get_instance().lock().unwrap().lastTranOptTime = current_timestamp_millis();
    }

    ///
    /// 刷新英式音标发音最后一次触发时间为当前时间
    /// 
    pub fn refresh_last_uk_born_time() {
        LastOptTime::get_instance().lock().unwrap().lastUkLastOptTime = current_timestamp_millis();
    }

    ///
    /// 刷新美式音标发音最后一次触发时间为当前时间
    /// 
    pub fn refresh_last_us_born_time() {
        LastOptTime::get_instance().lock().unwrap().lastUsLastOptTime = current_timestamp_millis();
    }
}


