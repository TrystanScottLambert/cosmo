use extendr_api::prelude::*;
use rayon::prelude::*;
use integrate::adaptive_quadrature;
use libm::{sin, sinh, log10, asinh};
use std::f64::{self, consts::PI};

const SPEED_OF_LIGHT: f64 = 299_792.458; // km/s

/// e func that is integrated often.
/// @export
#[extendr]
fn e_func(z: f64, omega_m: f64, omega_k: f64, omega_l: f64) -> f64 {
    (omega_m * (1.0 + z).powi(3) + omega_k * (1.0 + z).powi(2) + omega_l).sqrt()
}

/// Hubble Distance.
/// @export
#[extendr]
fn hubble_distance(hubble_constant: f64) -> f64 {
    SPEED_OF_LIGHT/hubble_constant
}

/// Comoving distance.
/// @export
#[extendr]
fn comoving_distance(redshift: f64, omega_m: f64, omega_k: f64, omega_l: f64, h0: f64) -> f64 {
    if redshift == 0. {
        return 0.;
    }
    let tolerance = 10.0e-6;
    let min_h = 10.0e-8;
    let f = |z:f64| 1./e_func(z, omega_m, omega_k, omega_l);
    let cosmo_recession_velocity = adaptive_quadrature::adaptive_simpson_method(f, 0.0, redshift, min_h, tolerance)
        .expect("Value to close to zero. Must be within 10e-8");
    hubble_distance(h0) * cosmo_recession_velocity
}

/// Comoving distance.
/// @export
#[extendr]
fn comoving_distances(redshift_array: Vec<f64>, omega_m:f64, omega_k:f64, omega_l:f64, h0: f64) -> Vec<f64> {
    redshift_array
        .par_iter()
        .map(|z| comoving_distance(*z, omega_m, omega_k, omega_l, h0))
        .collect()
}


/// Comoving transverse distance.
/// @export
#[extendr]
pub fn comoving_trans_distance(redshift: f64, omega_m: f64, omega_k: f64, omega_l: f64, h0: f64) -> f64 {
    let co_dist = comoving_distance(redshift, omega_m, omega_k, omega_l, h0);
    let h_dist = hubble_distance(h0);

    match omega_k {
        val if val > 0. => {h_dist * (1./omega_k.sqrt()) * sinh(omega_k.sqrt() * (co_dist/h_dist))},
        val if val < 0. => {h_dist * (1./omega_k.abs().sqrt()) * sin(omega_k.abs().sqrt() * (co_dist/h_dist))},
        _ => co_dist,
    }
}

/// Comoving transverse distances for an array
/// @export
#[extendr]
pub fn comoving_transverse_distances(redshifts: Vec<f64>, omega_m:f64, omega_k:f64, omega_l:f64, h0:f64) -> Vec<f64> {
    redshifts
        .par_iter()
        .map(|z| comoving_trans_distance(*z, omega_m, omega_k, omega_l, h0))
        .collect()
}

/// Distance modulus.
/// @export
#[extendr]
fn dist_mod(redshift: f64, omega_m: f64, omega_k: f64, omega_l: f64, h0: f64) -> f64 {
    let co_trans_dist = comoving_trans_distance(redshift, omega_m, omega_k, omega_l, h0);
    5. * log10(co_trans_dist * (1. + redshift)) + 25.
}

/// Distance moduli
/// @export
#[extendr]
fn dist_mods(redshifts: Vec<f64>, omega_m: f64, omega_k: f64, omega_l: f64, h0: f64) -> Vec<f64> {
    redshifts
        .par_iter()
        .map(|z| dist_mod(*z, omega_m, omega_l, omega_k, h0))
        .collect()
}

/// Comoving Volume.
/// @export
#[extendr]
fn comoving_volume(redshift: f64, omega_m: f64, omega_k: f64, omega_l: f64, h0: f64) -> f64 {
    let h_dist = hubble_distance(h0);
    let co_dist_tran = comoving_trans_distance(redshift, omega_m, omega_k, omega_l, h0);

    match omega_k {
        val if val == 0. => {(4./3.) * PI * co_dist_tran.powi(3)},
        val if val < 0. => {
            (4.*PI*h_dist.powi(3)/(2.*omega_k))*((co_dist_tran/h_dist)*(1.+omega_k*(co_dist_tran/h_dist).powi(2)).sqrt()-
                (1./omega_k.abs().sqrt())*asinh(omega_k.abs().sin()*(co_dist_tran/h_dist)))
        },
        _ => {h_dist*(1./omega_k.sqrt())*sinh(omega_k.sqrt()*co_dist_tran/h_dist)}
    }
}

/// Comoving Volumes
/// @export
#[extendr]
fn comoving_volumes(redshifts: Vec<f64>, omega_m: f64, omega_k: f64, omega_l: f64, h0: f64) -> Vec<f64> {
    redshifts
        .par_iter()
        .map(|z| comoving_volume(*z, omega_m, omega_k, omega_l, h0))
        .collect()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod astrs;
    fn comoving_distance;
    fn comoving_distances;
    fn comoving_volume;
    fn comoving_volumes;
    fn comoving_trans_distance;
    fn comoving_transverse_distances;
    fn dist_mod;
    fn dist_mods;
}
