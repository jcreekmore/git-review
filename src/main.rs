#[macro_use]
extern crate clap;
extern crate git2;

use clap::{Arg, App, SubCommand};
use git2::Repository;

fn list() -> Result<(), git2::Error> {
    let repo = try!(Repository::open_from_env());
    let head = try!(repo.head());
    let target = head.target().expect("head was not a direct reference");
    let mut commit = try!(repo.find_commit(target));

    println!("{}", commit.summary().unwrap());
    Ok(())
}

pub fn main() {
    let matches = App::new("git-review")
        .version(crate_version!())
        .author("Jonathan Creekmore <jonathan@thecreekmores.org>")
        .about("Short review listing of commits")
        .subcommand(SubCommand::with_name("list")
                        .about("Lists the commits since the parent branch for easy review."))
        .subcommand(SubCommand::with_name("show")
                        .about("Show the diff of an individual commit in the review branch.")
                        .arg(Arg::with_name("COMMIT")
                                .help("Which commit in the list to display")
                                .required(true)
                                .index(1)))
        .get_matches();

    if let Some(show_options) = matches.subcommand_matches("show") {
        // grab the commit from to show

    } else {
        // by default, we will just do a list

        if let Err(err) = list() {
            println!("Failed to list commits: {}", err);
        }
    }
}
