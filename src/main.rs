use std::fmt::format;

use clap::{Arg, ArgAction, Command};
use cosmoxide::Cosmology;
use colored::Colorize;

fn cli() -> Command {
    Command::new("cosmo")
        .about("Cosmology Calculator CLI")
        .version("0.1.0")
        .author("Trytan Lambert")

        .subcommand(
            Command::new("summary")
         .about("Print out a summary of values for the given redshift.")
         .aliases(["sum", "all"])
         .arg(
            Arg::new("z")
            .required(true)
            .index(1)
            .help("Redshift.")
         )   
        )
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
        .subcommand(
            Command::new("angscale_phys")
                .aliases(["angscale", "angular_scale", "angular_scale_physical", "angscale_physical"])
                .about("The physical angular scale on sky in units of kpc/arcsec. (Default angular scale)")
                .arg(Arg::new("z").required(true).index(1).help("redshift"))
                .arg(Arg::new("mpc").long("mpc-per-arcmin").short('m').help("Return the angular scale in units of Mpc/arcmin").action(ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("angscale_co")
                .aliases(["angular_scale_comoving", "angscale_comoving"])
                .about("The co-moving angular scale on sky in units of kpc/arcsec.")
                .arg(Arg::new("z").required(true).index(1).help("redshift"))
                .arg(Arg::new("mpc").long("mpc-per-arcmin").short('m').help("Return the angular scale in units of Mpc/arcmin").action(ArgAction::SetTrue))
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

        Some(("summary", sub_matches)) => {
            let z = try_parse_string_to_f64(sub_matches.get_one::<String>("z").unwrap());
            println!("Redshift (z): {}", z.to_string().bold().blue());
            println!("Expansion factor (a): {}", format!("{:.4}",(1./(1. + z))).bold().blue());
            println!();
            println!("Comoving distance: {} Mpc", format!("{:.4}",cosmo.comoving_distance(z)).bold().blue());
            println!("Luminosity distance: {} Mpc", format!("{:.4}",cosmo.luminosity_distance(z)).bold().blue());
            println!("Angular diameter distance: {} Mpc", format!("{:.4}",cosmo.angular_diameter_distance(z)).bold().blue());
            println!("Comoving transverse distance: {} Mpc", format!("{:.4}",cosmo.comoving_transverse_distance(z)).bold().blue());
            println!("Distance Modulus: {} mag", format!("{:.4}",cosmo.distance_modulus(z)).bold().blue());
            println!("Physical angular scale: {} kpc/arcsec", format!("{:.4}",cosmo.kpc_per_arcsecond_physical(z)).bold().blue());
            println!("Comoving angular scale: {} kpc/arcsec", format!("{:.4}",cosmo.kpc_per_arcsecond_comoving(z)).bold().blue());
            println!("Comoving Volume: {} Gpc^3", format!("{:.4}",(cosmo.comoving_volume(z)/1e9)).bold().blue());
            println!();
            println!("H(z): {}", format!("{:.4}",cosmo.h_at_z(z)).bold().blue());
            println!("Expansion rate: {}", format!("{:.4}", cosmo.h_at_z(z)/(1.+z)).bold().blue());
            println!();
            println!("Age: {} Gyr", format!("{:.4}", cosmo.age(z)).bold().blue());
            println!("Look back time: {} Gyr", format!("{:.4}", cosmo.look_back_time(z)).bold().blue());
            println!("Universe Age Now: {} Gyr", format!("{:.4}", cosmo.age(0.)).bold().blue());
            println!("Hubble Time: {} Gyr", format!("{:.4}", cosmo.hubble_time()).bold().blue());
            
        }

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
                println!("{} Mpc", cosmo.luminosity_distance(value))
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

        Some(("angscale_phys", sub_matches)) => {
            let redshift: f64 = try_parse_string_to_f64(sub_matches.get_one::<String>("z").unwrap());
            if *sub_matches.get_one::<bool>("mpc").unwrap_or(&false) {
                println!("Angular scale = {} pMpc/arcmin", cosmo.kpc_per_arcsecond_physical(redshift) * 60./1e3)
            } else {
                println!("Angular scale = {} pkpc/arcsec",cosmo.kpc_per_arcsecond_physical(redshift))
            }
        }

        Some(("angscale_co", sub_matches)) => {
            let redshift: f64 = try_parse_string_to_f64(sub_matches.get_one::<String>("z").unwrap());
            if *sub_matches.get_one::<bool>("mpc").unwrap_or(&false) {
                println!("Angular scale = {} cMpc/arcmin", cosmo.kpc_per_arcsecond_comoving(redshift) * 60./1e3)
            } else {
                println!("Angular scale = {} ckpc/arcsec",cosmo.kpc_per_arcsecond_comoving(redshift))
            }
        }

        _ => println!("Command not recognized"),
    }
}
