use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

#[derive(Clone, Debug)]
pub struct MyData {
    pub id: u64,
    pub time: SystemTime,
}

impl MyData {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            time: SystemTime::now(),
        }
    }

    /// hh:mm:ss 形式で文字列取得
    pub fn formatted_time(&self) -> String {
        let datetime = self.time
            .duration_since(UNIX_EPOCH)
            .unwrap();
        let secs = datetime.as_secs() % 86400; // 1日の秒数
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

impl fmt::Display for MyData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyData {{ id: {}, time: {} }}", self.id, self.formatted_time())
    }
}
