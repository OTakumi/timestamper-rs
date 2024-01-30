use chrono::prelude::*;

pub fn get_start_work_time() -> DateTime<Utc> {
    Utc::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_start_work_time();
        assert_eq!(result, Utc::now());
    }
}
