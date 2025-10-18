`cosmo`. Be an adult, use the terminal.

# Motivation

There are several cosmology calculations that are frequently used in astronomy and cosmology calculators have become a staple. The most popular one being [Ned Wrights cosmology calculator](https://www.astro.ucla.edu/~wright/CosmoCalc.html), which many people have improved upon, notably [ICRAR's version by Aaron Robotham](https://cosmocalc.icrar.org/) (which this tool is largely inspired by).

In addition, there are several software packages available in many languages for performing these same cosmological calculations, most popular by far the `astropy` cosmology module.

However, personally, if I want to quickly calculate a co-moving distance, or angular scale or the age at a particular redshift then **I don't want to open a web browser or write python code!** Instead, I just want to run a command in the terminal and have the answer instantly.

`cosmo` is a CLI tool that aims to address this niche and provides basic cosmological functions directly from the terminal.

# Usage

`cosmo` has a very usable help function that can be used for the main command via `cosmo --help` which will list all the possible sub-commands. All of these also have `--help` which will describe the available arguments.

## Adopting a cosmology

For all commands the cosmological parameters can be set with `--omega-m`, `--omega-lambda`, `--omega-k`, and the `--hubble-const` or their short equivalents `-m`, `-l`, `-k`, `-H` respectively.
For example, calculating the co-moving distance at redshift 0.3, using the Plank 13 cosmology (H0 = 67.3, Omega matter = 0.315, Omega Lambda = 0.685, Omega k = 0.) can be done like this

```bash
cosmo codist 0.3 -m 0.315 -l 0.685 -H 67.3
```

The default values for cosmo is a vanilla 737 cosmology (H0 = 70, Omega matter = 0.3, Omega Lambda = 0.7, Omega k = 0.) Each one of these values will be adopted if they are not given explicitly.

_At the moment `cosmo` is built for flat cosmologies and will raise an error for open or closed universes_

## Sub-commands

### all

For most use-cases, `cosmo all <z>` will be sufficient. This returns a summary representation of the most common cosmological properties at a given redshift <z>

```bash
cosmo all 0.3
```

This will also show additional values which haven't been included as an official sub-command.

### Co-moving distance

Co-moving distance at a given redshift in Mpc.

```bash
cosmo codist 0.3
```

The `-i` `--inverse` flag can be used to calculate the redshift at a given distance which is given in Mpc.

```bash
cosmo codist -i 100
```

### Luminosity distance

Luminosity distance at a given redshift in Mpc.

```bash
cosmo lumdist 0.3
```

The `-i` `--inverse` flag can be used to calculate the redshift at a given distance which is given in Mpc.

```bash
cosmo lumdist -i 100
```

### Co-moving volume

Co-moving volume at a given redshift in Gpc³.

```bash
cosmo covol 0.3
```

The `-i` `--inverse` flag can be used to calculate the redshift at a given volume which is given in Gpc³.

```bash
cosmo covol -i 3.14
```

### Look-back time

Look-back time at a given redshift in Gyr.

```bash
cosmo lookback 0.3
```

The `-i` `--inverse` flag can be used to calculate the redshift at a given look-back time which is given in Gyr.

```bash
cosmo lookback -i 3.14
```

### Distance modulus

Distance modulus at a given redshift in magnitudes.

```bash
cosmo distmod 0.3
```

### Physical angular scale
The physical on-sky angular scale at a given redshift. The default value is returned in kpc/arcsec.
```bash
cosmo angscale_phys 0.3
```
*The physical scale is also the default value for angular scale and can also be calculated just using `angscale`
```bash
cosmo angscale 0.3
```

The `-M` `--mpc-per-arcmin` flag can be used to return the value in units of mpc/arcmin instead of kpc/arcsecond.

```bash
cosmo angscale -M 0.3
```
### Co-moving angular scale
The co-moving on-sky angular scale at a given redshift. The default value is returned in kpc/arcsec.
```bash
cosmo angscale_phys 0.3
```

The `-M` `--mpc-per-arcmin` flag can be used to return the value in units of mpc/arcmin instead of kpc/arcsecond.



# Installation

## Downloading the binary

The binary for `cosmo` is only available on mac-os but will be made available on Linux soon.

**Installing dog is very easy**

```
curl -L -o dog https://github.com/trystanscottlambert/cosmo/releases/download/v1.0.0/cosmo-macos-x86_64
chmod +x cosmo
sudo mv cosmo /usr/local/bin/
```

You may need to start a new terminal to get it working.

If you don't want to install the binary then you can compile the program from source using 'cargo'.

## Compile from source

If you are using Linux or don't want to download a binary file then `cosmo` can be built from source using cargo.

First make sure you have rust installed:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Download the cosmo repo with git

```
git clone git@github.com:TrystanScottLambert/cosmo.git
```

cd into the cosmo folder
`cd cosmo/`

You should be able to see the `Cargo.toml` file. From here compile using cargo (which would already be installed with rust.)

```
cargo build --release
```

Then simply move the binary file into your /bin directory

```
sudo mv target/release/cosmo /usr/local/bin
```

You may need to restart the terminal. You can remove the cloned repo if you wish.
