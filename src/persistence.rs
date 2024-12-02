use rusqlite::{params, Connection, OptionalExtension, Result, Error};
use chrono::NaiveDate;

use crate::student::{Student, Status};


// Struct que representa a conexão com o banco sqlite
// Possui métodos para cada uma das operações CRUD
#[derive(Debug)]
pub struct Persistence {
    conn: Connection
}

impl Persistence {
    // Método que cria uma instância do banco e cria uma tabela para guardar os estudantes
    pub fn new(db_name: &str) -> Result<Self, Error> {
        let conn = Connection::open(db_name)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS students (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                birth_date TEXT,
                cr REAL,
                status TEXT
            )",
            [],
        )?;

        Ok(Persistence { conn })
    }

    pub fn read_all(&self) -> Result<Vec<Student>, Error> {
        let mut all_students = self.conn.prepare("SELECT id, name, birth_date, cr, status FROM students")?;

        let student_iter = all_students.query_map([], |row| {
            let name = row.get::<_, String>(1).ok();

            let birth_date = match row.get::<_, String>(2) {
                Ok(bd) => NaiveDate::parse_from_str(&bd, "%Y-%m-%d").ok(),
                Err(_) => None,
            };

            let cr: Option<f32> = match row.get::<_, f32>(3) {
                Ok(c) => Some(c),
                Err(_) => None,
            };

            let status: Option<Status> = match row.get::<_, String>(4) {
                Ok(s) => match s.as_str() {
                    "Active" => Some(Status::Active),
                    "Inactive" => Some(Status::Inactive),
                    "Graduated" => Some(Status::Graduated),
                    "Suspended" => Some(Status::Suspended),
                    _ => None,
                },
                Err(_) => None,
            };

            let mut student_instance = Student::new(
                name,
                birth_date,
                cr,
                status,
            );

            student_instance.set_id(row.get::<_, i32>(0)?);

            Ok(student_instance)
        })?;

        student_iter.collect()
    }

    pub fn read_one(&self, student_id: i32) -> Result<Option<Student>, Error> {
        let mut student = self.conn.prepare("SELECT id, name, birth_date, cr, status FROM students WHERE id = ?1")?;

        let student = student.query_row(params![student_id], |row| {
            let name = row.get::<_, String>(1).ok();

            let birth_date = match row.get::<_, String>(2) {
                Ok(bd) => NaiveDate::parse_from_str(&bd, "%Y-%m-%d").ok(),
                Err(_) => None,
            };

            let cr: Option<f32> = match row.get::<_, f32>(3) {
                Ok(c) => Some(c),
                Err(_) => None,
            };

            let status: Option<Status> = match row.get::<_, String>(4) {
                Ok(s) => match s.as_str() {
                    "Active" => Some(Status::Active),
                    "Inactive" => Some(Status::Inactive),
                    "Graduated" => Some(Status::Graduated),
                    "Suspended" => Some(Status::Suspended),
                    _ => None,
                },
                Err(_) => None,
            };
            
            let mut student_instance = Student::new(
                name,
                birth_date,
                cr,
                status,
            );

            student_instance.set_id(row.get::<_, i32>(0)?);

            Ok(student_instance)
        }).optional()?;

        Ok(student)
    }

    pub fn create(&self, new_student: &Student) -> Result<(), Error> {
        let birth_date = new_student.birth_date.map(|bd| bd.to_string());

        println!("{} {:?}", new_student.cr.unwrap_or(0.0), new_student.status.clone().expect("Nao tem"));

        self.conn.execute(
            "INSERT INTO students (name, birth_date, cr, status) VALUES (?1, ?2, ?3, ?4)",
            params![
                new_student.name,
                birth_date,
                new_student.cr,
                match new_student.status {
                    Some(Status::Active) => "Active",
                    Some(Status::Inactive) => "Inactive",
                    Some(Status::Graduated) => "Graduated",
                    Some(Status::Suspended) => "Suspended",
                    None => "None",
                }
            ],
        )?;

        Ok(())
    }

    pub fn update(&self, updated_student: &Student) -> Result<(), Error> {
        let birth_date = updated_student.birth_date.map(|bd| bd.to_string());

        if let Some(id) = updated_student.get_id() {
            self.conn.execute(
                "UPDATE students SET name = ?1, birth_date = ?2, cr = ?3, status = ?4 WHERE id = ?5",
                params![
                    updated_student.name,
                    birth_date,
                    updated_student.cr,
                    match updated_student.status {
                        Some(Status::Active) => "Active",
                        Some(Status::Inactive) => "Inactive",
                        Some(Status::Graduated) => "Graduated",
                        Some(Status::Suspended) => "Suspended",
                        None => "None",
                    },
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