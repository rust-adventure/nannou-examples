# Perlin Noise Benchmarks

Benchmarking generating perlin noise into a nannou DynamicImage for 1080p and 4k resolutions

```
noise 1920x1080         time:   [33.369 ms 33.850 ms 34.366 ms]
                        change: [-83.876% -83.673% -83.443%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

noise 3840x2160         time:   [132.11 ms 133.60 ms 135.35 ms]
                        change: [-84.762% -84.349% -84.005%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe
```
