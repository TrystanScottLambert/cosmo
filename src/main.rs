use clap::{Arg, ArgAction, Command};
use cosmoxide::Cosmology;

fn cli() -> Command {
    Command::new("cosmo")
        .about("Cosmology Calculator CLI")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(
            Command::new("codist")
                .aliases(["co_dist", "comoving_distance", "CoDist"])
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
                .aliases(["lum_dist", "luminosity_distance", "LumDist"])
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
                .aliases(["co_vol", "comoving_volume", "CoVol"])
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
                .aliases(["look_back", "look_back_time", "lookback_time", "TravelTime"])
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
                .aliases(["Age", "UniAge"])
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
                .aliases(["DistanceMod", "DistMod", "Distmod", "distance_modulus", "dist_mod"])
                .about("Distance modulus at a given redshift")
                .arg(Arg::new("z").required(true).index(1).help("redshift")),
        )
}

fn try_parse_string_to_f64(string: &String) -> f64 {
    match string.parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Value {:?} is not a valid number.", string);
            std::process::exit(1)
        }
    }
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
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {:?}", cosmo.inverse_codist(value));
            } else {
                println!("{:?} Mpc", cosmo.comoving_distance(value));
            }
        }

        Some(("lumdist", sub_matches)) => {
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_codist(value));
            } else {
                println!("{} Mpc", cosmo.comoving_distance(value))
            }
        }

        Some(("covol", sub_matches)) => {
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_covol(value * 1e9));
            } else {
                println!("{} Gpc^3", cosmo.comoving_volume(value) / 1e9);
            }
        }

        Some(("lookback", sub_matches)) => {
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_lookback_time(value));
            } else {
                println!("{} Gyr", cosmo.look_back_time(value));
            }
        }

        Some(("age", sub_matches)) => {
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_age(value));
            } else {
                println!("{} Gyr", cosmo.age(value));
            }
        }

        Some(("distmod", sub_matches)) => {
            let redshift = try_parse_string_to_f64(sub_matches.get_one::<String>("z").unwrap());
            println!("distance modulus = {}", cosmo.distance_modulus(redshift));
        }

        _ => println!("Command not recognized"),
    }
}
