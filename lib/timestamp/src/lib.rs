use chrono::prelude::*;

#[derive(Default)]
pub struct TimeStamp {
    start_work_time: DateTime<Utc>,
    end_work_time: DateTime<Utc>,
}

impl TimeStamp {
    pub fn new() -> TimeStamp {
        TimeStamp {
            start_work_time: Utc::now(),
            end_work_time: Utc::now(),
        }
    }

    pub fn get_start_work_time(&self) -> DateTime<Utc> {
        self.start_work_time
    }

    pub fn get_end_work_time(&self) -> DateTime<Utc> {
        self.end_work_time
    }

    pub fn punch_end_work_time(&mut self, end_work_time: DateTime<Utc>) {
        self.end_work_time = end_work_time;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_work_time_can_be_input() {
        let time_card = TimeStamp::new();

        assert_eq!(
            time_card
                .get_start_work_time()
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
            Utc::now().format("%Y/%m/%d %H:%M:%S").to_string()
        );
    }

    #[test]
    fn end_work_time_can_be_input() {
        let time_card = TimeStamp::new();

        assert_eq!(
            time_card
                .get_end_work_time()
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
            Utc::now().format("%Y/%m/%d %H:%M:%S").to_string()
        );
    }
}
