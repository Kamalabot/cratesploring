#![allow(unused_imports)]
#![allow(warnings)]

use clap::{Args, Parser, Subcommand};
use diesel_prac1::*;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct CRUDI {
    ///Crud Sub commands are executed
    #[command(subcommand)]
    crud: CrudCommand,
}

#[derive(Subcommand)]
enum CrudCommand {
    ///Employee creation
    CreateEmployee {
        /// Name of the employee
        name: String,
        /// Age of the employee
        age: i32,
        /// Department he is working in
        department: String,
        /// Current is at work
        #[arg(short, long)]
        working: bool,
    },
    /// Show Employee
    ShowEmployees,
    /// Get Employee
    GetEmployee {
        id: i32,
    },
    /// Delete Employee
    DeleteEmployee {
        id: i32,
    },
    UnworkEmployee {
        id: i32,
    },
}

fn main() {
    // following simply connects to db
    let cli = CRUDI::parse();
    let mut conn = connect();
    match cli.crud {
        CrudCommand::CreateEmployee {
            name,
            age,
            department,
            working,
        } => create_employee(&mut conn, name.as_str(), age, department.as_str(), working),
        CrudCommand::ShowEmployees => show_employees(),
        CrudCommand::UnworkEmployee { id } => unwork_employee(&mut conn, id),
        CrudCommand::GetEmployee { id } => show_employee(&mut conn, id),
        CrudCommand::DeleteEmployee { id } => show_employee(&mut conn, id),
    }
}
