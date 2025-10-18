use clap::{Arg, ArgAction, Command};
use cosmoxide::Cosmology;

fn cli() -> Command {
    Command::new("cosmo")
        .about("Cosmology Calculator CLI")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(
            Command::new("codist")
                .about("Calculate comoving distance in Mpc")
                .arg(
                    Arg::new("input")
                        .required(true)
                        .index(1)
                        .help("Either redshift or comoving distance"),
                )
                .arg(
                    Arg::new("inverse")
                        .long("inverse")
                        .short('i')
                        .help("Inverse. Redshift at a given comoving distance in Mpc.")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("lumdist")
                .about("Calculate the luminosity distance in Mpc")
                .arg(
                    Arg::new("input")
                        .required(true)
                        .index(1)
                        .help("Either redshift or luminosity distance in Mpc"),
                )
                .arg(
                    Arg::new("inverse")
                        .long("inverse")
                        .short('i')
                        .help("Inverse. Redshift at a given comoving distance in Mpc.")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("covol")
                .about("Calculate the co-moving volume in Gpc^3.")
                .arg(
                    Arg::new("input")
                        .required(true)
                        .index(1)
                        .help("Either redshift or comoving volume in Gpc^3."),
                )
                .arg(
                    Arg::new("inverse")
                        .long("inverse")
                        .short('i')
                        .help("Inverse. Redshift at a given comoving volume in Gpc^3.")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("lookback")
                .about("Calculate the lookback time in Gyr")
                .arg(
                    Arg::new("input")
                        .required(true)
                        .index(1)
                        .help("Either redshift or lookback time in Gyr"),
                )
                .arg(
                    Arg::new("inverse")
                        .long("inverse")
                        .short('i')
                        .help("Inverse. Redshift at a given lookback time in Gyr")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("age")
                .about("Calculate the age of the universe in Gyr at a given redshift.")
                .arg(
                    Arg::new("input")
                        .required(true)
                        .index(1)
                        .help("Either redshift or age in Gyr"),
                )
                .arg(
                    Arg::new("inverse")
                        .long("inverse")
                        .short('i')
                        .help("Inverse. Redshift at a given age in Gyr")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("distmod")
                .about("Distance modulus at a given redshift")
                .arg(Arg::new("z").required(true).index(1).help("redshift")),
        )
}

fn main() {
    let matches = cli().get_matches();
    let cosmo = Cosmology {
        omega_m: 0.3,
        omega_k: 0.0,
        omega_l: 0.7,
        h0: 70.,
    };

    match matches.subcommand() {
        Some(("codist", sub_matches)) => {
            let value = sub_matches.get_one::<String>("input").unwrap();
            let value = value.parse::<f64>().expect("Value must be a number.");
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {:?}", cosmo.inverse_codist(value));
            } else {
                println!("{:?} Mpc", cosmo.comoving_distance(value));
            }
        }

        Some(("lumdist", sub_matches)) => {
            let value = sub_matches.get_one::<String>("input").unwrap();
            let value = value.parse::<f64>().expect("Value must be a number");
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_codist(value));
            } else {
                println!("{} Mpc", cosmo.comoving_distance(value))
            }
        }

        Some(("covol", sub_matches)) => {
            let value = sub_matches.get_one::<String>("input").unwrap();
            let value = value.parse::<f64>().expect("Value must be a number");
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_covol(value * 1e9));
            } else {
                println!("{} Gpc^3", cosmo.comoving_volume(value) / 1e9);
            }
        }

        Some(("lookback", sub_matches)) => {
            let value = sub_matches.get_one::<String>("input").unwrap();
            let value = value.parse::<f64>().expect("Value must be a number");
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_lookback_time(value));
            } else {
                println!("{} Gyr", cosmo.look_back_time(value));
            }
        }

        Some(("age", sub_matches)) => {
            let value = sub_matches.get_one::<String>("input").unwrap();
            let value = value.parse::<f64>().expect("Value must be a number");
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_age(value));
            } else {
                println!("{} Gyr", cosmo.age(value));
            }
        }

        Some(("distmod", sub_matches)) => {
            let redshift = sub_matches.get_one::<String>("z").unwrap();
            let redshift = redshift.parse::<f64>().expect("Value must be a number");
            println!("distance modulus = {}", cosmo.distance_modulus(redshift));
        }

        _ => println!("Command not recognized"),
    }
}
