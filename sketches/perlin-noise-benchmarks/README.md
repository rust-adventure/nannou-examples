# 2 dimension Perlin noise

Perlin noise visualized with the inputs being a grid location and the output being a color value.

For colorful variants, the color used is LCH with the noise being responsible for the hue and the lightness/chroma being controlled by a user.

![perlin noise](./perlin-noise-2d.png)

```
noise 1920x1080         time:   [207.00 ms 207.32 ms 207.68 ms]
Found 16 outliers among 100 measurements (16.00%)
  9 (9.00%) high mild
  7 (7.00%) high severe


noise 3840x2160         time:   [841.83 ms 853.65 ms 874.37 ms]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
```
