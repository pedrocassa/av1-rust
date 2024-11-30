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
            let name = args.get_one::<String>("name").unwrap_or_else(|| {
                eprintln!("Error: 'name' argument is required");
                std::process::exit(1);
            }).to_string();

            let birth_date = NaiveDate::parse_from_str(args.get_one::<String>("birth_date").unwrap_or_else(|| {
                eprintln!("Error: 'birth_date' argument is required");
                std::process::exit(1);
            }), "%Y-%m-%d")?;

            let cr: f32 = args.get_one::<String>("cr").unwrap_or_else(|| {
                eprintln!("Error: 'cr' argument is required");
                std::process::exit(1);
            }).parse()?;

            let status = match args.get_one::<String>("status").unwrap_or_else(|| {
                eprintln!("Error: 'status' argument is required");
                std::process::exit(1);
            }).as_str() {
                "Active" => Status::Active,
                "Inactive" => Status::Inactive,
                "Graduated" => Status::Graduated,
                "Suspended" => Status::Suspended,
                _ => Status::Active,
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

            let name = args.get_one::<String>("name").map(|n| n.to_string());
            let birth_date = args.get_one::<String>("birth_date")
                .map(|bd| NaiveDate::parse_from_str(bd, "%Y-%m-%d").expect("Invalid date format"));
            let cr = args.get_one::<String>("cr").map(|c| c.parse().unwrap());
            let status = args.get_one::<String>("status").map(|s| match s.as_str() {
                "Active" => Status::Active,
                "Inactive" => Status::Inactive,
                "Graduated" => Status::Graduated,
                "Suspended" => Status::Suspended,
                _ => Status::Active,
            });

            let mut updated_student = Student::new(
                name.unwrap_or_default(),
                birth_date.unwrap(),
                cr.unwrap(),
                status.unwrap(),
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
