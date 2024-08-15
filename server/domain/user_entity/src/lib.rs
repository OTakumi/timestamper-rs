use ulid::Ulid;

#[derive(Debug, PartialEq)]
pub enum WorkingStatus {
    Working,
    NotWorking,
    OnBreak,
}

/// Employee ID
struct ID {
    /// ID is a struct that contains a ULID value.
    value: Ulid,
}

impl ID {
    /// Create a new ID
    pub fn new() -> Self {
        ID { value: Ulid::new() }
    }

    /// Get the value of the ID
    pub fn get_value(&self) -> Ulid {
        self.value
    }
}

/// Employee
/// Employee is a struct that contains an ID, name, and working status.
pub struct Employee {
    /// Employee must have an ID
    id: ID,
    /// Employee must have a name
    name: String,
    /// Employee must have a working status
    /// default is NotWorking
    status: WorkingStatus,
}

impl Employee {
    /// Create a new Employee
    pub fn new(name: String) -> Self {
        Employee {
            id: ID::new(),
            name,
            status: WorkingStatus::NotWorking,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// check the id length
    /// ULID is a 26 character string
    fn test_id() {
        let id = ID::new();
        assert_eq!(id.get_value().to_string().len(), 26);
    }

    #[test]
    /// check the employee name and status
    fn test_new_employee() {
        let employee = Employee::new("John Doe".to_string());
        assert_eq!(employee.name, "John Doe");
        assert_eq!(employee.status, WorkingStatus::NotWorking);
    }
}
