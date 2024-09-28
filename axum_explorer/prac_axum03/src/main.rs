#![allow(unused_imports)]

use clap::{Args, Parser, Subcommand};
use prac_axum03::*;

#[derive(Parser)]
#[command(about, version, long_about=None)]
struct Cli {
    /// show Sessions till now
    #[arg(short, long)]
    display_sessions: bool,
    /// Commands to CRUD database
    #[command(subcommand)]
    crudcmd: Option<CrudCommand>,
}

#[derive(Subcommand)]
enum CrudCommand {
    Create(CreateTable),
    ShowRow(RowId),
    UpdtRow(RowId),
    DeleteRow(RowId),
}

#[derive(Args)]
struct RowId {
    pid: i32,
}

#[derive(Args)]
struct CreateTable {
    /// Name of the session
    sessionname: String,
    /// Current practice session number
    practice: i32,
    /// Package you are practicing
    package: String,
}

fn main() {
    let cli = Cli::parse();

    if cli.display_sessions {
        show_all()
    }
    match cli.crudcmd {
        Some(CrudCommand::Create(CreateTable {
            sessionname,
            practice,
            package,
        })) => {
            let completed = true;
            create_prac(sessionname, practice, package, completed)
        }
        Some(CrudCommand::ShowRow(RowId { pid })) => show_prac(pid),
        Some(CrudCommand::UpdtRow(RowId { pid })) => uncomplete(pid),
        Some(CrudCommand::DeleteRow(RowId { pid })) => remove_sess(pid),
        None => println!("skipped"),
    }
}
