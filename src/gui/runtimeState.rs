///
/// 运行时状态
/// 

use crate::utils::datetime::{current_timestamp_millis};

#[derive(Debug)]
pub struct RunTimeState {
    // 英式音标发音最后一次触发时间
    pub lastUkBornTime: i64,
    // 美式音标发音最后一次触发时间
    pub lastUsBornTime: i64,
}

impl RunTimeState {
    pub fn new() -> RunTimeState {
        RunTimeState {
            lastUkBornTime: -1,
            lastUsBornTime: -1,
        }
    }

    ///
    /// 刷新英式音标发音最后一次触发时间为当前时间
    /// 
    pub fn refresh_last_uk_born_time(state: &mut RunTimeState) {
        state.lastUkBornTime = current_timestamp_millis();
    }

    ///
    /// 刷新美式音标发音最后一次触发时间为当前时间
    /// 
    pub fn refresh_last_us_born_time(state: &mut RunTimeState) {
        state.lastUsBornTime = current_timestamp_millis();
    }
}


