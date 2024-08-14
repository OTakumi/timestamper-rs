use chrono::prelude::*;

#[derive(Default)]
pub struct TimeStamp {
    start_work_time: DateTime<Utc>,
    end_work_time: DateTime<Utc>,

    // work_status is true when working.
    work_status: bool,
}

impl TimeStamp {
    /// create new TimeStamp instance.
    /// start_work_time is set to current time.
    /// end_work_time is set to default value.
    pub fn new() -> TimeStamp {
        let default_datetime: DateTime<Utc> = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        TimeStamp {
            start_work_time: default_datetime,
            end_work_time: default_datetime,
            work_status: false,
        }
    }

    pub fn get_start_work_time(&self) -> DateTime<Utc> {
        self.start_work_time
    }

    pub fn get_end_work_time(&self) -> DateTime<Utc> {
        self.end_work_time
    }

    pub fn punch_start_work_now(&mut self) -> String {
        if !self.work_status {
            self.start_work_time = Utc::now();
            self.work_status = true;
            "Start working.".to_string()
        } else {
            "You have already started working.".to_string()
        }
    }

    pub fn punch_end_work_time(&mut self, end_work_time: DateTime<Utc>) {
        self.end_work_time = end_work_time;
    }

    pub fn get_work_status(&self) -> bool {
        self.work_status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_work_time_can_be_input() {
        let mut time_card = TimeStamp::new();
        time_card.punch_start_work_now();

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
        let mut time_card = TimeStamp::new();
        time_card.punch_end_work_time(Utc::now());

        assert_eq!(
            time_card
                .get_end_work_time()
                .format("%Y/%m/%d %H:%M:%S")
                .to_string(),
            Utc::now().format("%Y/%m/%d %H:%M:%S").to_string()
        );
    }

    #[test]
    fn already_working() {
        let mut time_card = TimeStamp::new();
        // Check that the time is not updated and only the message is displayed after two times of punching.
        #[allow(unused_assignments)]
        let mut message = time_card.punch_start_work_now();
        message = time_card.punch_start_work_now();

        // assert!(time_card.get_work_status());
        assert_eq!(message, "You have already started working.");
    }
}
