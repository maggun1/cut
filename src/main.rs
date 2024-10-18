use clap::{Arg, Command, ArgAction};
use std::io::{stdin, BufRead, Error};

fn main() -> Result<(), Error> {
    let matches = Command::new("cut")
        .arg(Arg::new("fields")
            .short('f')
            .long("fields")
            .num_args(1))

        .arg(Arg::new("delimiter")
            .short('d')
            .long("delimiter")
            .num_args(1)
            .default_value("\t"))

        .arg(Arg::new("separated")
            .short('s')
            .long("separated")
            .action(ArgAction::SetTrue))
        .get_matches();

    let fields = matches.get_one::<String>("fields");
    let delimiter = matches.get_one::<String>("delimiter").unwrap();
    let separated = matches.get_flag("separated");

    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let parts = line.split(delimiter).collect::<Vec<&str>>();

        if separated && parts.len() == 1 {
            continue;
        }

        match fields {
            Some(fields) => {
                let fields = fields.split(delimiter).filter_map(|s| s.parse::<usize>().ok()).collect::<Vec<usize>>();
                let selected_fields= fields.iter()
                    .filter_map(|&f| parts.get(f - 1))
                    .cloned()
                    .collect::<Vec<&str>>();
                println!("{}", selected_fields.join(delimiter));
            },
            None => {println!("{}", line);},
        }
    }
    Ok(())
}
