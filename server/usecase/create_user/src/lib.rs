use working_status::WorkingStatus;

trait UserTrait {
    fn new(username: String) -> Self;
    fn get_username(&self) -> &str;
    fn get_working_status(&self) -> &WorkingStatus;
}

struct User {
    username: String,
    working_status: WorkingStatus,
}

impl UserTrait for User {
    fn new(username: String) -> User {
        User {
            username,
            working_status: WorkingStatus::NotWorking,
        }
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_working_status(&self) -> &WorkingStatus {
        &self.working_status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
