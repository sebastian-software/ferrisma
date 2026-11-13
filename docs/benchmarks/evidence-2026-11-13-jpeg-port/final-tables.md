## arm64 simd
| case | C ms | before ms | after ms | before / C | after / C | after / before |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline 4:2:0, RGB | 26.93 | 30.52 | 29.81 | 1.13 | 1.11 | 0.98 |
| baseline 4:2:2, RGB | 31.18 | 35.82 | 34.78 | 1.15 | 1.12 | 0.97 |
| baseline 4:4:4, RGB | 38.49 | 45.89 | 43.85 | 1.19 | 1.14 | 0.96 |
| progressive 4:2:0, RGB | 68.61 | 100.38 | 80.73 | 1.46 | 1.18 | 0.80 |
| baseline 4:2:0, RGBX | 27.60 | 30.18 | 30.37 | 1.09 | 1.10 | 1.01 |
| baseline 4:2:0, fast IDCT | 25.53 | 27.65 | 27.89 | 1.08 | 1.09 | 1.01 |
| baseline 4:2:0, float IDCT | 38.51 | 40.13 | 40.55 | 1.04 | 1.05 | 1.01 |
| baseline 4:2:0, merged upsampling | 25.63 | 27.89 | 27.16 | 1.09 | 1.06 | 0.97 |
| baseline 4:2:2, merged upsampling | 30.29 | 33.89 | 33.08 | 1.12 | 1.09 | 0.98 |
| baseline 4:2:0, scale 1/2 | 22.24 | 23.85 | 23.83 | 1.07 | 1.07 | 1.00 |
| baseline 4:2:0, scale 1/4 | 20.15 | 21.38 | 21.40 | 1.06 | 1.06 | 1.00 |
| baseline 4:2:0, scale 1/8 | 18.50 | 19.56 | 19.52 | 1.06 | 1.05 | 1.00 |
| testorig.jpg (227x149) | 0.09 | 0.11 | 0.11 | 1.25 | 1.20 | 0.96 |
load average (1 min) 21.0-23.6; env {}; 7 runs x 5 decodes; all checksums equal across C and the port binaries
## arm64 portable
| case | C ms | before ms | after ms | before / C | after / C | after / before |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline 4:2:0, RGB | 58.06 | 75.59 | 58.71 | 1.30 | 1.01 | 0.78 |
| baseline 4:2:2, RGB | 61.71 | 84.20 | 68.26 | 1.36 | 1.11 | 0.81 |
| baseline 4:4:4, RGB | 73.50 | 84.71 | 77.85 | 1.15 | 1.06 | 0.92 |
| progressive 4:2:0, RGB | 91.86 | 140.67 | 104.68 | 1.53 | 1.14 | 0.74 |
| baseline 4:2:0, RGBX | 52.76 | 72.66 | 55.26 | 1.38 | 1.05 | 0.76 |
| baseline 4:2:0, fast IDCT | 50.48 | 70.36 | 52.89 | 1.39 | 1.05 | 0.75 |
| baseline 4:2:0, float IDCT | 52.64 | 69.22 | 55.50 | 1.31 | 1.05 | 0.80 |
| baseline 4:2:0, merged upsampling | 44.25 | 53.27 | 45.97 | 1.20 | 1.04 | 0.86 |
| baseline 4:2:2, merged upsampling | 55.25 | 66.08 | 59.37 | 1.20 | 1.07 | 0.90 |
| baseline 4:2:0, scale 1/2 | 32.43 | 35.81 | 34.61 | 1.10 | 1.07 | 0.97 |
| baseline 4:2:0, scale 1/4 | 24.20 | 24.99 | 25.32 | 1.03 | 1.05 | 1.01 |
| baseline 4:2:0, scale 1/8 | 18.39 | 20.14 | 19.63 | 1.10 | 1.07 | 0.97 |
| testorig.jpg (227x149) | 0.18 | 0.27 | 0.22 | 1.51 | 1.22 | 0.81 |
load average (1 min) 23.0-26.5; env {}; 7 runs x 5 decodes; all checksums equal across C and the port binaries
## x86-64 emulated avx2
| case | C ms | before ms | after ms | before / C | after / C | after / before |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline 4:2:0, RGB | 88.80 | 99.96 | 85.71 | 1.13 | 0.97 | 0.86 |
| baseline 4:2:2, RGB | 96.80 | 102.22 | 101.94 | 1.06 | 1.05 | 1.00 |
| baseline 4:4:4, RGB | 121.32 | 124.91 | 124.31 | 1.03 | 1.02 | 1.00 |
| progressive 4:2:0, RGB | 134.85 | 197.87 | 149.82 | 1.47 | 1.11 | 0.76 |
| baseline 4:2:0, RGBX | 80.09 | 80.82 | 84.16 | 1.01 | 1.05 | 1.04 |
| baseline 4:2:0, fast IDCT | 60.16 | 65.88 | 57.45 | 1.10 | 0.96 | 0.87 |
| baseline 4:2:0, float IDCT | 62.00 | 61.12 | 61.02 | 0.99 | 0.98 | 1.00 |
| baseline 4:2:0, merged upsampling | 71.38 | 84.33 | 84.62 | 1.18 | 1.19 | 1.00 |
| baseline 4:2:2, merged upsampling | 91.85 | 93.89 | 91.71 | 1.02 | 1.00 | 0.98 |
| baseline 4:2:0, scale 1/2 | 43.09 | 45.29 | 42.61 | 1.05 | 0.99 | 0.94 |
| baseline 4:2:0, scale 1/4 | 27.77 | 34.99 | 28.53 | 1.26 | 1.03 | 0.82 |
| baseline 4:2:0, scale 1/8 | 23.96 | 25.19 | 25.29 | 1.05 | 1.06 | 1.00 |
| testorig.jpg (227x149) | 0.32 | 0.35 | 0.35 | 1.11 | 1.10 | 0.99 |
load average (1 min) 24.5-28.3; env {'ROSETTA_ADVERTISE_AVX': '1'}; 3 runs x 5 decodes; all checksums equal across C and the port binaries
## x86-64 emulated sse2
| case | C ms | before ms | after ms | before / C | after / C | after / before |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline 4:2:0, RGB | 49.50 | 50.92 | 42.62 | 1.03 | 0.86 | 0.84 |
| baseline 4:2:2, RGB | 61.33 | 49.45 | 55.12 | 0.81 | 0.90 | 1.11 |
| baseline 4:4:4, RGB | 64.72 | 69.05 | 59.41 | 1.07 | 0.92 | 0.86 |
| progressive 4:2:0, RGB | 95.08 | 194.45 | 142.61 | 2.05 | 1.50 | 0.73 |
| baseline 4:2:0, RGBX | 47.99 | 45.44 | 42.21 | 0.95 | 0.88 | 0.93 |
| baseline 4:2:0, fast IDCT | 45.29 | 41.13 | 43.62 | 0.91 | 0.96 | 1.06 |
| baseline 4:2:0, float IDCT | 53.02 | 44.03 | 53.70 | 0.83 | 1.01 | 1.22 |
| baseline 4:2:0, merged upsampling | 47.90 | 47.48 | 59.20 | 0.99 | 1.24 | 1.25 |
| baseline 4:2:2, merged upsampling | 56.95 | 49.44 | 45.34 | 0.87 | 0.80 | 0.92 |
| baseline 4:2:0, scale 1/2 | 40.36 | 34.18 | 41.61 | 0.85 | 1.03 | 1.22 |
| baseline 4:2:0, scale 1/4 | 32.30 | 29.75 | 27.76 | 0.92 | 0.86 | 0.93 |
| baseline 4:2:0, scale 1/8 | 23.33 | 24.18 | 32.35 | 1.04 | 1.39 | 1.34 |
| testorig.jpg (227x149) | 0.16 | 0.17 | 0.17 | 1.05 | 1.05 | 1.00 |
load average (1 min) 28.3-29.7; env {'ROSETTA_ADVERTISE_AVX': '1', 'JSIMD_FORCESSE2': '1'}; 3 runs x 5 decodes; all checksums equal across C and the port binaries
## x86-64 emulated portable
| case | C ms | before ms | after ms | before / C | after / C | after / before |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| baseline 4:2:0, RGB | 203.36 | 148.89 | 169.50 | 0.73 | 0.83 | 1.14 |
| baseline 4:2:2, RGB | 140.89 | 208.51 | 83.63 | 1.48 | 0.59 | 0.40 |
| baseline 4:4:4, RGB | 94.64 | 116.78 | 101.05 | 1.23 | 1.07 | 0.87 |
| progressive 4:2:0, RGB | 117.15 | 205.22 | 136.03 | 1.75 | 1.16 | 0.66 |
| baseline 4:2:0, RGBX | 89.36 | 112.09 | 83.53 | 1.25 | 0.93 | 0.75 |
| baseline 4:2:0, fast IDCT | 66.01 | 91.64 | 68.64 | 1.39 | 1.04 | 0.75 |
| baseline 4:2:0, float IDCT | 71.39 | 105.07 | 86.27 | 1.47 | 1.21 | 0.82 |
| baseline 4:2:0, merged upsampling | 56.01 | 91.36 | 71.83 | 1.63 | 1.28 | 0.79 |
| baseline 4:2:2, merged upsampling | 107.01 | 134.19 | 115.00 | 1.25 | 1.07 | 0.86 |
| baseline 4:2:0, scale 1/2 | 42.12 | 80.01 | 89.34 | 1.90 | 2.12 | 1.12 |
| baseline 4:2:0, scale 1/4 | 32.25 | 33.48 | 32.90 | 1.04 | 1.02 | 0.98 |
| baseline 4:2:0, scale 1/8 | 23.00 | 25.18 | 24.98 | 1.09 | 1.09 | 0.99 |
| testorig.jpg (227x149) | 0.22 | 0.33 | 0.26 | 1.52 | 1.20 | 0.79 |
load average (1 min) 28.2-47.5; env {}; 3 runs x 5 decodes; all checksums equal across C and the port binaries
