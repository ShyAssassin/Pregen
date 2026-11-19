// FIXME: This sucks fix it!!!!!!!!!!

use log::Level;

static mut SINK_ID_COUNTER: usize = usize::MIN + 1;
pub type SinkCallback = Box<dyn Fn(&log::Record) + Send + Sync>;

pub struct SinkEntry {
    minimum_level: Level,
    pub id: Option<usize>,
    callback: SinkCallback,
}

impl SinkEntry {
    pub fn new(level: Level, callback: SinkCallback) -> Self {
        // Potential race condition :)
        let sink_id = unsafe {
            let id = SINK_ID_COUNTER;
            SINK_ID_COUNTER += 1;
            id
        };

        return Self {
            id: Some(sink_id),
            callback: callback,
            minimum_level: level,
        }
    }

    pub fn callback(&self, record: &log::Record) {
        if record.level() <= self.minimum_level {
            (self.callback)(record);
        }
    }
}
