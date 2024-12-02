use std::error::Error;
use clap::{Arg, Command};
use chrono::NaiveDate;

mod student;
mod persistence;

use student::{Student, Status};
use persistence::Persistence;

fn main() -> Result<(), Box<dyn Error>> {
    let commands = Command::new("Student CRUD")
        .version("1.0")
        .author("Pedro Cassa Dias")
        .about("Student CRUD")
        .subcommand(
            Command::new("create")
                .about("Creates a new student")
                .arg(Arg::new("name").short('n').long("name").value_name("NAME").required(true))
                .arg(Arg::new("birth_date").short('b').long("birth_date").value_name("BIRTH_DATE").required(true))
                .arg(Arg::new("cr").short('c').long("cr").value_name("CR").required(true))
                .arg(Arg::new("status").short('s').long("status").value_name("STATUS").required(true)),
        )
        .subcommand(Command::new("read_all").about("Returns a list of all students"))
        .subcommand(
            Command::new("read_one")
                .about("Returns a student")
                .arg(Arg::new("id").short('i').long("id").value_name("ID").required(true)),
        )
        .subcommand(
            Command::new("update")
                .about("Updates a student")
                .arg(Arg::new("id").short('i').long("id").value_name("ID").required(true))
                .arg(Arg::new("name").short('n').long("name").value_name("NAME"))
                .arg(Arg::new("birth_date").short('b').long("birth_date").value_name("BIRTH_DATE"))
                .arg(Arg::new("cr").short('c').long("cr").value_name("CR"))
                .arg(Arg::new("status").short('s').long("status").value_name("STATUS")),
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes a student")
                .arg(Arg::new("id").short('i').long("id").value_name("ID").required(true)),
        )
        .get_matches();

    let persistence = Persistence::new("students.db")?;

    match commands.subcommand() {
        Some(("create", args)) => {
            let name = match args.get_one::<String>("name") {
                Some(n) => Some(n.to_string()),
                None => None,
            };

            let birth_date = match args.get_one::<String>("birth_date") {
                Some(bd) => Some(NaiveDate::parse_from_str(bd, "%Y-%m-%d").expect("Invalid date format")),
                None => None,
            };

            let cr = match args.get_one::<String>("cr") {
                Some(c) => match c.parse::<f32>() {
                    Ok(val) => Some(val),
                    Err(_) => {
                        println!("Failed to parse 'cr' to an integer.");
                        Some(0.0)
                    }
                },
                None => Some(0.0),
            };

            let status = match args.get_one::<String>("status") {
                Some(s) => match s.as_str() {
                    "Active" => Some(Status::Active),
                    "Inactive" => Some(Status::Inactive),
                    "Graduated" => Some(Status::Graduated),
                    "Suspended" => Some(Status::Suspended),
                    _ => None,
                },
                None => None,
            };

            let new_student = Student::new(name, birth_date, cr, status);

            persistence.create(&new_student)?;
        }
        Some(("read_one", args)) => {
            let id_str = args.get_one::<String>("id").unwrap();

            let id: i32 = id_str.parse().unwrap_or_else(|_| {
                eprintln!("Error: invalid ID format");
                std::process::exit(1);
            });

            match persistence.read_one(id)? {
                Some(student) => student.display(),
                None => println!("Student not found"),
            }
        }
        Some(("read_all", _)) => {
            let students = persistence.read_all()?;

            for student in students {
                student.display()
            }
        }
        Some(("update", args)) => {
            let id_str = args.get_one::<String>("id").unwrap();

            let id: i32 = id_str.parse().unwrap_or_else(|_| {
                eprintln!("Error: invalid ID format");
                std::process::exit(1);
            });

            let student_option = persistence.read_one(id).unwrap_or_else(|_| {
                eprintln!("Error: no student found with the provided id");
                std::process::exit(1);
            });

            let student = match student_option {
                Some(student) => student,
                None => {
                    eprintln!("No student found with the provided ID");
                    std::process::exit(1);
                }
            };

            let name = match args.get_one::<String>("name") {
                Some(n) => Some(n.to_string()),
                None => student.name,
            };

            let birth_date = match args.get_one::<String>("birth_date") {
                Some(bd) => Some(NaiveDate::parse_from_str(bd, "%Y-%m-%d").expect("Invalid date format")),
                None => student.birth_date,
            };

            let cr = match args.get_one::<String>("cr") {
                Some(c) => match c.parse::<f32>() {
                    Ok(val) => Some(val),
                    Err(_) => {
                        println!("Failed to parse 'cr' to an integer.");
                        student.cr
                    }
                },
                None => student.cr,
            };

            let status = match args.get_one::<String>("status") {
                Some(s) => match s.as_str() {
                    "Active" => Some(Status::Active),
                    "Inactive" => Some(Status::Inactive),
                    "Graduated" => Some(Status::Graduated),
                    "Suspended" => Some(Status::Suspended),
                    _ => None,
                },
                None => student.status,
            };
                
            let mut updated_student = Student::new(
                name,
                birth_date,
                cr,
                status,
            );

            updated_student.set_id(id);
            
            persistence.update(&updated_student)?;
        }
        Some(("delete", args)) => {
            let id_str = args.get_one::<String>("id").unwrap();
            let id: i32 = id_str.parse().unwrap_or_else(|_| {
                eprintln!("Error: invalid ID format");
                std::process::exit(1);
            });
            persistence.delete(id)?;
        }
        _ => {
            println!("Invalid Command");
        }
    }

    Ok(())
}
