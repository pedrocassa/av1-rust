use chrono::NaiveDate;

#[derive(Debug)]
pub enum Status {
    Active,
    Inactive,
    Graduated,
    Suspended,
}

#[derive(Debug)]
pub struct Student {
    id: Option<i32>,
    pub name: String,
    pub birth_date: NaiveDate,
    pub cr: f32,
    pub status: Status,
}

impl Student {
    pub fn get_id(&self) -> Option<i32> {
        self.id
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    pub fn new(name: String, birth_date: NaiveDate, cr: f32, status: Status) -> Self {
        Student {
            id: None,
            name,
            birth_date,
            cr,
            status,
        }
    }

    pub fn display(&self) {
        println!("\nID: {:?}", self.id.unwrap_or(-1));
        println!("Nome: {}", self.name);
        println!("Data de Nascimento: {}", self.birth_date);
        println!("CR: {}", self.cr);
        println!("Status: {:?}", self.status);
    }
}