
use clap::{Arg, Command, ArgAction};

pub fn cli() -> Command {
    Command::new("cosmo")
        .about("Cosmology Calculator CLI")
        .version("0.1.0")
        .author("Trytan Lambert")
        .subcommand_required(true)
        .subcommand(
    Command::new("all")
                .about("Print out a summary of values for the given redshift.")
                .aliases(["sum", "summary"])
                .arg(
                    Arg::new("z")
                    .required(true)
                    .index(1)
                    .help("Redshift.")
                )
                .arg(
                    Arg::new("omega_matter")
                    .long("omega-m")
                    .short('m')
                    .help("Omega matter, mass density of the universe. [default 0.3]")
                )
                .arg(
                    Arg::new("omega_lambda")
                    .long("omega-l")
                    .short('l')
                    .help("Omega lambda, Effective mass density of dark energy. [default 0.7]")
                )
                .arg(
                    Arg::new("omega_k")
                    .long("omega-k")
                    .short('k')
                    .help("Omega k, Effective mass density of relativistic particles. [default 0.0]")
                )
                .arg(
                    Arg::new("hubble_constant")
                    .long("hubble-const")
                    .short('H')
                    .help("Hubble constant, default is 70 km/s/Mpc")
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
                )
                .arg(
                    Arg::new("omega_matter")
                    .long("omega-m")
                    .short('m')
                    .help("Omega matter, mass density of the universe. [default 0.3]")
                )
                .arg(
                    Arg::new("omega_lambda")
                    .long("omega-l")
                    .short('l')
                    .help("Omega lambda, Effective mass density of dark energy. [default 0.7]")
                )
                .arg(
                    Arg::new("omega_k")
                    .long("omega-k")
                    .short('k')
                    .help("Omega k, Effective mass density of relativistic particles. [default 0.0]")
                )
                .arg(
                    Arg::new("hubble_constant")
                    .long("hubble-const")
                    .short('H')
                    .help("Hubble constant, default is 70 km/s/Mpc")
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
                )
                .arg(
                    Arg::new("omega_matter")
                    .long("omega-m")
                    .short('m')
                    .help("Omega matter, mass density of the universe. [default 0.3]")
                )
                .arg(
                    Arg::new("omega_lambda")
                    .long("omega-l")
                    .short('l')
                    .help("Omega lambda, Effective mass density of dark energy. [default 0.7]")
                )
                .arg(
                    Arg::new("omega_k")
                    .long("omega-k")
                    .short('k')
                    .help("Omega k, Effective mass density of relativistic particles. [default 0.0]")
                )
                .arg(
                    Arg::new("hubble_constant")
                    .long("hubble-const")
                    .short('H')
                    .help("Hubble constant, default is 70 km/s/Mpc")
                ),
        )
        .subcommand(
            Command::new("covol")
                .aliases(["co_vol", "comoving_volume", "CoVol"])
                .about("Calculate the co-moving volume in Gpc³.")
                .arg(
                    Arg::new("input")
                        .required(true)
                        .index(1)
                        .help("Either redshift or comoving volume in Gpc³."),
                )
                .arg(
                    Arg::new("inverse")
                        .long("inverse")
                        .short('i')
                        .help("Inverse. Redshift at a given comoving volume in Gpc³.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("omega_matter")
                    .long("omega-m")
                    .short('m')
                    .help("Omega matter, mass density of the universe. [default 0.3]")
                )
                .arg(
                    Arg::new("omega_lambda")
                    .long("omega-l")
                    .short('l')
                    .help("Omega lambda, Effective mass density of dark energy. [default 0.7]")
                )
                .arg(
                    Arg::new("omega_k")
                    .long("omega-k")
                    .short('k')
                    .help("Omega k, Effective mass density of relativistic particles. [default 0.0]")
                )
                .arg(
                    Arg::new("hubble_constant")
                    .long("hubble-const")
                    .short('H')
                    .help("Hubble constant, default is 70 km/s/Mpc")
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
                )
                .arg(
                    Arg::new("omega_matter")
                    .long("omega-m")
                    .short('m')
                    .help("Omega matter, mass density of the universe. [default 0.3]")
                )
                .arg(
                    Arg::new("omega_lambda")
                    .long("omega-l")
                    .short('l')
                    .help("Omega lambda, Effective mass density of dark energy. [default 0.7]")
                )
                .arg(
                    Arg::new("omega_k")
                    .long("omega-k")
                    .short('k')
                    .help("Omega k, Effective mass density of relativistic particles. [default 0.0]")
                )
                .arg(
                    Arg::new("hubble_constant")
                    .long("hubble-const")
                    .short('H')
                    .help("Hubble constant, default is 70 km/s/Mpc")
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
                )
                .arg(
                    Arg::new("omega_matter")
                    .long("omega-m")
                    .short('m')
                    .help("Omega matter, mass density of the universe. [default 0.3]")
                )
                .arg(
                    Arg::new("omega_lambda")
                    .long("omega-l")
                    .short('l')
                    .help("Omega lambda, Effective mass density of dark energy. [default 0.7]")
                )
                .arg(
                    Arg::new("omega_k")
                    .long("omega-k")
                    .short('k')
                    .help("Omega k, Effective mass density of relativistic particles. [default 0.0]")
                )
                .arg(
                    Arg::new("hubble_constant")
                    .long("hubble-const")
                    .short('H')
                    .help("Hubble constant, default is 70 km/s/Mpc")
                ),
        )
        .subcommand(
            Command::new("distmod")
                .aliases(["DistanceMod", "DistMod", "Distmod", "distance_modulus", "dist_mod"])
                .about("Distance modulus at a given redshift")
                .arg(Arg::new("z").required(true).index(1).help("redshift"))
                .arg(
                    Arg::new("omega_matter")
                    .long("omega-m")
                    .short('m')
                    .help("Omega matter, mass density of the universe. [default 0.3]")
                )
                .arg(
                    Arg::new("omega_lambda")
                    .long("omega-l")
                    .short('l')
                    .help("Omega lambda, Effective mass density of dark energy. [default 0.7]")
                )
                .arg(
                    Arg::new("omega_k")
                    .long("omega-k")
                    .short('k')
                    .help("Omega k, Effective mass density of relativistic particles. [default 0.0]")
                )
                .arg(
                    Arg::new("hubble_constant")
                    .long("hubble-const")
                    .short('H')
                    .help("Hubble constant, default is 70 km/s/Mpc")
                ),
        )
        .subcommand(
            Command::new("angscale_phys")
                .aliases(["angscale", "angular_scale", "angular_scale_physical", "angscale_physical"])
                .about("The physical angular scale on sky in units of kpc/arcsec. (Default angular scale)")
                .arg(Arg::new("z").required(true).index(1).help("redshift"))
                .arg(Arg::new("mpc").long("mpc-per-arcmin").short('M').help("Return the angular scale in units of Mpc/arcmin").action(ArgAction::SetTrue))
                                .arg(
                    Arg::new("omega_matter")
                    .long("omega-m")
                    .short('m')
                    .help("Omega matter, mass density of the universe. [default 0.3]")
                )
                .arg(
                    Arg::new("omega_lambda")
                    .long("omega-l")
                    .short('l')
                    .help("Omega lambda, Effective mass density of dark energy. [default 0.7]")
                )
                .arg(
                    Arg::new("omega_k")
                    .long("omega-k")
                    .short('k')
                    .help("Omega k, Effective mass density of relativistic particles. [default 0.0]")
                )
                .arg(
                    Arg::new("hubble_constant")
                    .long("hubble-const")
                    .short('H')
                    .help("Hubble constant, default is 70 km/s/Mpc")
                ),
        )
        .subcommand(
            Command::new("angscale_co")
                .aliases(["angular_scale_comoving", "angscale_comoving"])
                .about("The co-moving angular scale on sky in units of kpc/arcsec.")
                .arg(Arg::new("z").required(true).index(1).help("redshift"))
                .arg(Arg::new("Mpc").long("mpc-per-arcmin").short('M').help("Return the angular scale in units of Mpc/arcmin").action(ArgAction::SetTrue))
                                .arg(
                    Arg::new("omega_matter")
                    .long("omega-m")
                    .short('m')
                    .help("Omega matter, mass density of the universe. [default 0.3]")
                )
                .arg(
                    Arg::new("omega_lambda")
                    .long("omega-l")
                    .short('l')
                    .help("Omega lambda, Effective mass density of dark energy. [default 0.7]")
                )
                .arg(
                    Arg::new("omega_k")
                    .long("omega-k")
                    .short('k')
                    .help("Omega k, Effective mass density of relativistic particles. [default 0.0]")
                )
                .arg(
                    Arg::new("hubble_constant")
                    .alias("hubble-constant")
                    .long("hubble-const")
                    .short('H')
                    .help("Hubble constant, default is 70 km/s/Mpc")
                ),
        )
}
