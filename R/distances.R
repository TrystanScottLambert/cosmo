# Wrappers for the rust functions

.do_for_all <- function(cosmo_function_array, cosmo_function_single, redshift, o_m, o_k, o_l, h_0) {
    # Function taht will do the cosmo func for a single redshift or an array of redshifts.
    if (length(redshift) > 1) {
        cosmo_function_array(redshift, o_m, o_k, o_l, h_0)
    } else if (length(redshift) == 1) {
        cosmo_function_single(redshift, o_m, o_k, o_l, h_0)
    }
}

#' Work out the comoving distance for a given redshift or array of redshifts
#'
#' @param redshift either a single redshift or an array of redshifts.
#' @param omega_m Mass density (often 0.3 in LCDM)
#' @param omega_k Effective mass density of relativistic particles (often 0. in LCDM)
#' @param omega_l Effective mass density of dark energy (often 0.7 in LCDM)
#' @param h_0 H0 = 100 * h
#' @export
co_dist <- function(redshift, o_m, o_k, o_l, h_0) {
    .do_for_all(comoving_distances, comoving_distance, redshift, o_m, o_k, o_l, h_0)
}

#' Work out the comoving volume for a given redshift or array of redshifts
#'
#' @param redshift either a single redshift or an array of redshifts.
#' @param omega_m Mass density (often 0.3 in LCDM)
#' @param omega_k Effective mass density of relativistic particles (often 0. in LCDM)
#' @param omega_l Effective mass density of dark energy (often 0.7 in LCDM)
#' @param h_0 H0 = 100 * h
#' @export
co_vol <- function(redshift, o_m, o_k, o_l, h_0) {
    .do_for_all(comoving_volumes, comoving_volume, redshift, o_m, o_k, o_l, h_0)
}

#' Work out the distance modulus for a given redshift or an array of redshifts.
#'
#' @param redshift either a single redshift or an array of redshifts.
#' @param omega_m Mass density (often 0.3 in LCDM)
#' @param omega_k Effective mass density of relativistic particles (often 0. in LCDM)
#' @param omega_l Effective mass density of dark energy (often 0.7 in LCDM)
#' @param h_0 H0 = 100 * h
#' @export
distance_mod <- function(redshift, o_m, o_k, o_l, h_0) {
    .do_for_all(dist_mods, dist_mod, redshift, o_m, o_k, o_l, h_0)
}


#' Do the approximate inverse of the cosmological function
#'
#' @param cosmo_function The cosmological function to inverse
#' @param value The value of the function that you wan't to find the redshift for
#' @param omega_m Mass density (often 0.3 in LCDM)
#' @param omega_k Effective mass density of relativistic particles (often 0. in LCDM)
#' @param omega_l Effective mass density of dark energy (often 0.7 in LCDM)
#' @param h_0 H0 = 100 * h
#' @return The redshift at the given value
#' @export
z_at <- function(cosmo_function, value, omega_m, omega_k, omega_l, h0) {
    all_z = seq(0, 1500, length.out = 1000000)
    all_val = cosmo_function(all_z, omega_m, omega_k, omega_l, h0)
    .func = approxfun(all_val, all_z)
    return(.func(value))
}
