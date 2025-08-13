#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        match role {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let new_worker = Worker {
            role: role.to_string(),
            name: name.to_string(),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|worker| {
            let name = worker.name;
            self.grade = worker.next;
            name
        })
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        self.grade.as_ref().map(|worker| {
            (worker.name.clone(), Role::from(worker.role.as_str()))
        })
    }
}