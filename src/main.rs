use clap::ArgMatches;
use colored::Colorize;
use cosmoxide::Cosmology;

pub mod cli;

use cli::cli;

fn try_parse_string_to_f64(string: &String) -> f64 {
    match string.parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Value {:?} is not a valid number.", string);
            std::process::exit(1)
        }
    }
}

fn parse_cosmo_parameters(matches: &ArgMatches) -> Cosmology {
    let omega_m = match matches.get_one::<String>("omega_matter") {
        Some(om) => try_parse_string_to_f64(om),
        None => 0.3,
    };
    let omega_l = match matches.get_one::<String>("omega_lambda") {
        Some(ol) => try_parse_string_to_f64(ol),
        None => 0.7,
    };
    let omega_k = match matches.get_one::<String>("omega_k") {
        Some(ok) => try_parse_string_to_f64(ok),
        None => 0.,
    };
    let h0 = match matches.get_one::<String>("hubble_constant") {
        Some(h) => try_parse_string_to_f64(h),
        None => 70.,
    };
    if omega_k + omega_l + omega_m != 1. {
        println!("Chosen cosmology is not flat. Non-flat cosmology is not supported yet.");
        std::process::exit(1);
    }
    Cosmology {
        omega_m,
        omega_k,
        omega_l,
        h0,
    }
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("all", sub_matches)) => {
            let z = try_parse_string_to_f64(sub_matches.get_one::<String>("z").unwrap());
            let cosmo = parse_cosmo_parameters(sub_matches);
            println!("Redshift (z): {}", z.to_string().bold().green());
            println!(
                "Expansion factor (a): {}",
                format!("{:.4}", (1. / (1. + z))).bold().green()
            );
            println!();
            println!(
                "Comoving distance: {} Mpc",
                format!("{:.4}", cosmo.comoving_distance(z)).bold().green()
            );
            println!(
                "Luminosity distance: {} Mpc",
                format!("{:.4}", cosmo.luminosity_distance(z))
                    .bold()
                    .green()
            );
            println!(
                "Angular diameter distance: {} Mpc",
                format!("{:.4}", cosmo.angular_diameter_distance(z))
                    .bold()
                    .green()
            );
            println!(
                "Comoving transverse distance: {} Mpc",
                format!("{:.4}", cosmo.comoving_transverse_distance(z))
                    .bold()
                    .green()
            );
            println!(
                "Distance Modulus: {} mag",
                format!("{:.4}", cosmo.distance_modulus(z)).bold().green()
            );
            println!(
                "Physical angular scale: {} kpc/arcsec",
                format!("{:.4}", cosmo.kpc_per_arcsecond_physical(z))
                    .bold()
                    .green()
            );
            println!(
                "Comoving angular scale: {} kpc/arcsec",
                format!("{:.4}", cosmo.kpc_per_arcsecond_comoving(z))
                    .bold()
                    .green()
            );
            println!(
                "Comoving Volume: {} Gpc³",
                format!("{:.4}", (cosmo.comoving_volume(z) / 1e9))
                    .bold()
                    .green()
            );
            println!();
            println!("H(z): {}", format!("{:.4}", cosmo.h_at_z(z)).bold().green());
            println!(
                "Expansion rate: {}",
                format!("{:.4}", cosmo.h_at_z(z) / (1. + z)).bold().green()
            );
            println!();
            println!("Age: {} Gyr", format!("{:.4}", cosmo.age(z)).bold().green());
            println!(
                "Look back time: {} Gyr",
                format!("{:.4}", cosmo.look_back_time(z)).bold().green()
            );
            println!(
                "Universe Age Now: {} Gyr",
                format!("{:.4}", cosmo.age(0.)).bold().green()
            );
            println!(
                "Hubble Time: {} Gyr",
                format!("{:.4}", cosmo.hubble_time()).bold().green()
            );
        }

        Some(("codist", sub_matches)) => {
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            let cosmo = parse_cosmo_parameters(sub_matches);
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {:?}", cosmo.inverse_codist(value));
            } else {
                println!("{:?} Mpc", cosmo.comoving_distance(value));
            }
        }

        Some(("lumdist", sub_matches)) => {
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            let cosmo = parse_cosmo_parameters(sub_matches);
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_codist(value));
            } else {
                println!("{} Mpc", cosmo.luminosity_distance(value))
            }
        }

        Some(("covol", sub_matches)) => {
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            let cosmo = parse_cosmo_parameters(sub_matches);
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_covol(value * 1e9));
            } else {
                println!("{} Gpc³", cosmo.comoving_volume(value) / 1e9);
            }
        }

        Some(("lookback", sub_matches)) => {
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            let cosmo = parse_cosmo_parameters(sub_matches);
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_lookback_time(value));
            } else {
                println!("{} Gyr", cosmo.look_back_time(value));
            }
        }

        Some(("age", sub_matches)) => {
            let value = try_parse_string_to_f64(sub_matches.get_one::<String>("input").unwrap());
            let cosmo = parse_cosmo_parameters(sub_matches);
            if *sub_matches.get_one::<bool>("inverse").unwrap_or(&false) {
                println!("redshift = {}", cosmo.inverse_age(value));
            } else {
                println!("{} Gyr", cosmo.age(value));
            }
        }

        Some(("distmod", sub_matches)) => {
            let redshift = try_parse_string_to_f64(sub_matches.get_one::<String>("z").unwrap());
            let cosmo = parse_cosmo_parameters(sub_matches);
            println!("distance modulus = {}", cosmo.distance_modulus(redshift));
        }

        Some(("angscale_phys", sub_matches)) => {
            let cosmo = parse_cosmo_parameters(sub_matches);
            let redshift: f64 =
                try_parse_string_to_f64(sub_matches.get_one::<String>("z").unwrap());
            if *sub_matches.get_one::<bool>("mpc").unwrap_or(&false) {
                println!(
                    "Angular scale = {} pMpc/arcmin",
                    cosmo.kpc_per_arcsecond_physical(redshift) * 60. / 1e3
                )
            } else {
                println!(
                    "Angular scale = {} pkpc/arcsec",
                    cosmo.kpc_per_arcsecond_physical(redshift)
                )
            }
        }

        Some(("angscale_co", sub_matches)) => {
            let cosmo = parse_cosmo_parameters(sub_matches);
            let redshift: f64 =
                try_parse_string_to_f64(sub_matches.get_one::<String>("z").unwrap());
            if *sub_matches.get_one::<bool>("mpc").unwrap_or(&false) {
                println!(
                    "Angular scale = {} cMpc/arcmin",
                    cosmo.kpc_per_arcsecond_comoving(redshift) * 60. / 1e3
                )
            } else {
                println!(
                    "Angular scale = {} ckpc/arcsec",
                    cosmo.kpc_per_arcsecond_comoving(redshift)
                )
            }
        }

        _ => println!("Command not recognized"),
    }
}
