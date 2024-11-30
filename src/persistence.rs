use rusqlite::{params, Connection, OptionalExtension, Result, Error};
use chrono::NaiveDate;

use crate::student::{Student, Status};

#[derive(Debug)]
pub struct Persistence {
    conn: Connection
}

impl Persistence {
    pub fn new(db_name: &str) -> Result<Self, Error> {
        let conn = Connection::open(db_name)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS students (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                birth_date TEXT NOT NULL,
                cr REAL NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Persistence { conn })
    }

    pub fn read_all(&self) -> Result<Vec<Student>, Error> {
        let mut all_students = self.conn.prepare("SELECT id, name, birth_date, cr, status FROM students")?;

        let student_iter = all_students.query_map([], |row| {
            let status_str: String = row.get(4)?;

            let mut student_instance = Student::new(
                row.get(1)?,
                NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d").unwrap(),
                row.get(3)?,
                match status_str.as_str() {
                    "Active" => Status::Active,
                    "Inactive" => Status::Inactive,
                    "Graduated" => Status::Graduated,
                    "Suspended" => Status::Suspended,
                    _ => Status::Inactive,
                },
            );

            student_instance.set_id(row.get(0)?);

            Ok(student_instance)
        })?;

        student_iter.collect()
    }

    pub fn read_one(&self, student_id: i32) -> Result<Option<Student>, Error> {
        let mut student = self.conn.prepare("SELECT id, name, birth_date, cr, status FROM students WHERE id = ?1")?;

        let student = student.query_row(params![student_id], |row| {
            let status_str: String = row.get(4)?;
            
            let mut student_instance = Student::new(
                row.get(1)?,
                NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d").unwrap(),
                row.get(3)?,
                match status_str.as_str() {
                    "Active" => Status::Active,
                    "Inactive" => Status::Inactive,
                    "Graduated" => Status::Graduated,
                    "Suspended" => Status::Suspended,
                    _ => Status::Inactive,
                },
            );

            student_instance.set_id(row.get(0)?);

            Ok(student_instance)
        }).optional()?;

        Ok(student)
    }

    pub fn create(&self, new_student: &Student) -> Result<(), Error> {
        self.conn.execute(
            "INSERT INTO students (name, birth_date, cr, status) VALUES (?1, ?2, ?3, ?4)",
            params![
                new_student.name,
                new_student.birth_date.to_string(),
                new_student.cr,
                format!("{:?}", new_student.status)
            ],
        )?;

        Ok(())
    }

    pub fn update(&self, updated_student: &Student) -> Result<(), Error> {
        if let Some(id) = updated_student.get_id() {
            self.conn.execute(
                "UPDATE students SET name = ?1, birth_date = ?2, cr = ?3, status = ?4 WHERE id = ?5",
                params![
                    updated_student.name,
                    updated_student.birth_date.to_string(),
                    updated_student.cr,
                    format!("{:?}", updated_student.status),
                    id
                ],
            )?;
        }

        Ok(())
    }

    pub fn delete(&self, student_id: i32) -> Result<()> {
        self.conn.execute("DELETE FROM students WHERE id = ?1", params![student_id])?;

        Ok(())
    }
}