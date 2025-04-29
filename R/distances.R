# Wrappers for the rust functions

.do_for_all <- function(cosmo_function_array, cosmo_function_single, redshift, o_m, o_k, o_l, h_0) {
    # Function taht will do the cosmo func for a single redshift or an array of redshifts.
    if (length(redshift) > 1) {
        cosmo_function_array(redshift, o_m, o_k, o_l, h_0)
    } else if (length(redshift) == 1) {
        cosmo_function_single(redshift, o_m, o_k, o_l, h_0)
    }
}

co_dist <- function(redshift, o_m, o_k, o_l, h_0) {
    .do_for_all(comoving_distances, comoving_distance, redshift, o_m, o_k, o_l, h_0)
}

co_vol <- function(redshift, o_m, o_k, o_l, h_0) {
    .do_for_all(comoving_volumes, comoving_volume, redshift, o_m, o_k, o_l, h_0)
}

distance_mod <- function(redshift, o_m, o_k, o_l, h_0) {
    .do_for_all(dist_mods, dist_mod, redshift, o_m, o_k, o_l, h_0)
}
