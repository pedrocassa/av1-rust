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
    name: String,
    birth_date: NaiveDate,
    cr: f32,
    status: AcademicStatus,
}