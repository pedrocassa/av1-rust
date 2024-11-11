use std::fs;
use std::io;
use bincode;
use student::Student;

#[derive(Debug)]
pub struct Persistance {
    file_name: String
}

impl Persistance {
    fn read(&self) -> io::Result<Vec<Student>> {
        let mut contents = fs::read_to_string(self.file_name).unwrap_or(Vec::new());
        
        if contents.is_empty() {
            Ok(Vec::new())
        } else {
            let students: Vec<Student> = bincode::deserialize(&contents)?;

            println!("{}", students);
            
            Ok(students)
        }
    }

    fn create(&self, new_student: &Student) -> io::Result<()> {
        let mut students = self.read().unwrap_or_default();

        students.push(new_student.clone());
        
        let updated_content = bincode::serialize(&students)?;

        fs::write(&self.file_name, updated_content)?;
        
        Ok(())
    }

    fn update(&self, updated_student: &Student) -> io::Result<()> {
        let mut students = self.read().unwrap_or_default();

        if let Some(student) = students.iter_mut().find(|s| s.name == updated_student.name) {
            *student = updated_student.clone();
        }

        let updated_content = bincode::serialize(&students)?;

        fs::write(&self.file_name, updated_content)?;

        Ok(())
    }

    fn delete(&self, deleted_student: &Student) -> io::Result<()> {
        let mut students = self.read().unwrap_or_default();

        students.retain(|s| s.name != student_to_delete.name);

        let updated_content = bincode::serialize(&students)?;

        fs::write(&self.file_name, updated_content)?;

        Ok(())
    }
}