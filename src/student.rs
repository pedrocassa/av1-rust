use chrono::NaiveDate;

// Enum que representa os possíveis status de matrícula de estudante
#[derive(Debug)]
#[derive(Clone)]
pub enum Status {
    Active,
    Inactive,
    Graduated,
    Suspended,
}

// Struct que representa um estudante
// Campos são opcionais para habilitar a funcionalidade de patch (update parcial)
#[derive(Debug)]
pub struct Student {
    id: Option<i32>,
    pub name: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub cr: Option<f32>,
    pub status: Option<Status>,
}

impl Student {
    // Getter para id que é private
    pub fn get_id(&self) -> Option<i32> {
        self.id
    }

    // Setter para id que é private
    pub fn set_id(&mut self, id: i32) {
        self.id = Some(id);
    }

    // Construtor de estudante
    pub fn new(name: Option<String>, birth_date: Option<NaiveDate>, cr: Option<f32>, status: Option<Status>) -> Self {
        Student {
            id: None,
            name,
            birth_date,
            cr,
            status,
        }
    }

    // Método para mostrar as informações do estudante no console
    pub fn display(&self) {
        println!("\nID: {:?}", self.id.unwrap_or(-1));
        
        match &self.name {
            Some(name) => println!("Nome: {}", name),
            None => println!("Nome: Não informado"),
        }

        match &self.birth_date {
            Some(birth_date) => println!("Data de Nascimento (dd-mm-yy): {}", birth_date),
            None => println!("Data de Nascimento: Não informado"),
        }

        match &self.cr {
            Some(cr) => println!("CR: {}", cr),
            None => println!("CR: Não informado"),
        }

        match &self.status {
            Some(status) => match status {
                Status::Active => println!("Status: Ativo"),
                Status::Inactive => println!("Status: Inativo"),
                Status::Graduated => println!("Status: Graduado"),
                Status::Suspended => println!("Status: Suspenso"),
            },
            None => println!("Status: Não informado"),
        }
    }
}