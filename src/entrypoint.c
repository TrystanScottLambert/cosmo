// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_astrs_extendr(void *dll);

void R_init_astrs(void *dll) {
    R_init_astrs_extendr(dll);
}
