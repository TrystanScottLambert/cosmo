library(celestial)
library(ggplot2)

#getting astropy values
file = '~/Desktop/GAMA_paper_plotter/timing_example.txt'
astropy_data = as.data.frame(read.csv(file, sep = " "))
colnames(astropy_data) = c("N", "S")
numbers = seq(from = 1e5, to=1e7, by=1e5)

asgr_times <- numeric()
tsl_times <- numeric()

for (N in numbers) {
  redshifts <- seq(from = 0, to = 10, length.out = N)
  #celest.start = Sys.time()
  #asgr_dists = celestial::cosdistCoDist(redshifts, H0=70)
  #celest.end = Sys.time()

  tsl.start = Sys.time()
  tsl_dists = comoving_distances(redshifts, 0.3, 0.0, 0.7, 70)
  tsl.end = Sys.time()

  #asgr_times <- c(asgr_times, as.numeric(difftime(celest.end, celest.start, units = "secs")))
  tsl_times <- c(tsl_times, as.numeric(difftime(tsl.end, tsl.start, units = "secs")))
}

# Build the plot
#ggplot(data = data.frame(tsl_dists, asgr_dists), aes(x = tsl_dists, y = asgr_dists)) +
#  geom_line() +                     # Scatter plot
#  geom_abline(intercept = 0, slope = 1, color = "red", linetype = "dashed") +  # y = x line
#  theme_minimal()

# Put into a data frame for ggplot
timing_df <- data.frame(
  N = numbers,
  #Celestial = asgr_times,
  TSL = tsl_times
)

# Plot
#ggplot(timing_df, aes(x = N)) +
#  geom_line(aes(y = Celestial, color = "Celestial")) +
#  geom_line(aes(y = TSL, color = "TSL")) +
#  scale_x_log10() +   # If your N values span wide ranges, log scale helps
#  labs(
#    x = "Number of Redshifts (N)",
#    y = "Time (seconds)",
#    color = "Method",
#    title = "Timing Comparison"
#  ) +
#  theme_minimal()

ggplot() +
  geom_line(data = timing_df, aes(x = N, y = TSL, color = "TSL")) +
  geom_line(data = astropy_data, aes(x = N, y = S, color = "S")) +
  scale_x_log10() +
  labs(x = "Number of Redshifts (N)", y = "Time (seconds)", title="Killing astropy", color="Method") +
  theme_minimal()
