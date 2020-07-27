use crate::lcg::Random;


#[derive(Copy, Clone, Debug)]
pub struct SimplexNoise {
    x0: f64,
    y0: f64,
    z0: f64,
    permutations: [u8; 256],
}

impl SimplexNoise {
    pub fn init(random: Random) -> SimplexNoise {
        xo: f64 = random.nextDouble() * 256.0;
        yo: f64 = random.nextDouble() * 256.0;
        zo: f64 = random.nextDouble() * 256.0;
        permutations: [u8; 256];
        for n in 0u8..256 {
            permutations[n] = n;
        }
        for n in 0u8..256 {
            random_index: u8 = random.nextInt(256 - n);
            std::mem::swap(&mut permutations[n2 + n], &mut permutations[n]);
        }
        SimplexNoise { x0, y0, z0, permutations }
    }
    pub fn lookup(&mut self, n: u8) -> u8 {
        self.permutations[n & 0xff]
    }

    pub fn dot(g: [i32; 3], d: f64, d2: f64, d3: f64) -> f64 {
        (g[0]) as f64 * d + (g[1]) as f64 * d2 + (g[2]) as f64 * d3
    }

    pub fn get_corner_noise3d(n: i32, x: f64, y: f64, z: f64, max: f64) -> f64 {
        let res: f64;
        let mut contribution: f64 = max - x * x - y * y - z * z;
        if contribution < 0.0 {
            res = 0.0;
        } else {
            contribution *= contribution;
            res = contribution * contribution * SimplexNoise::dot(GRADIENT[n], x, y, z);
        }
        res
    }

    pub fn get_value_2d(&mut self, x: f64, z: f64) -> f64 {
        let hairy_factor: f64 = (x + z) * F2;
        let temperature_x: f64 = (x + hairy_factor).floor();
        let temperature_z: f64 = (z + hairy_factor).floor();
        let mixed_temperature_x_z: f64 = (temperature_x + temperature_z) * G2;
        let temp_diff_x_to_z = temperature_x - mixed_temperature_x_z;

        let temp_diff_z_to_x: f64 = temperature_z - mixed_temperature_x_z;
        let x0: f64 = x - temp_diff_x_to_z;
        let y0: f64 = z - temp_diff_z_to_x;
        let offset_second_corner_x: f64;
        let offset_second_corner_z: f64;

        if x0 > y0 {  // lower triangle, XY order: (0,0)->(1,0)->(1,1)
            offset_second_corner_x = 1f64;
            offset_second_corner_z = 0f64;
        } else { // upper triangle, YX order: (0,0)->(0,1)->(1,1)
            offset_second_corner_x = 0f64;
            offset_second_corner_z = 1f64;
        }
        let x1: f64 = x0 - offset_second_corner_x + G2;
        let y1: f64 = y0 - offset_second_corner_z + G2;
        let x2: f64 = x0 - 1.0 + 2.0 * G2;
        let y2: f64 = y0 - 1.0 + 2.0 * G2;
        let ii: u8 = temperature_x & 0xFF;
        let jj: u8 = temperature_z & 0xFF;
        let gi0: u8 = self.lookup(ii + self.lookup(jj)) % 12;
        let gi1: u8 = self.lookup(ii + offset_second_corner_x + self.lookup(jj + offset_second_corner_z)) % 12;
        let gi2: u8 = self.lookup(ii + 1 + self.lookup(jj + 1)) % 12;
        let t0: f64 = self.get_corner_noise3d(gi0, x0, y0, 0.0, 0.5);
        let t1: f64 = self.get_corner_noise3d(gi1, x1, y1, 0.0, 0.5);
        let t2: f64 = self.get_corner_noise3d(gi2, x2, y2, 0.0, 0.5);
        70.0 * (t0 + t1 + t2)
    }
    pub fn get_value_3d(&mut self, x: f64, y: f64, z: f64)->f64 {
        let skew_factor = (d + d2 + d3) * F3; // F3 is 1/3
        // Skew the input space to determine which simplex cell we're in
        let i: i32 = (x + skew_factor).floor();
        let j: i32 = (y + skew_factor).floor();
        let k: i32 = (z + skew_factor).floor();
        let unskew_factor = (double)(i + j + k) * G3; // G3 is 1/6
        // Unskew the cell origin back to (x,y,z) space
        let x0 = (i) as f64 - unskew_factor;
        let y0 = (j) as f64 - unskew_factor;
        let z0 = (k) as f64 - unskew_factor;
        // The x,y,z distances from the cell origin
        let x0 = x - x0;
        let y0 = y - y0;
        let z0 = z - z0;
        //For the 3D case, the simplex shape is a slightly irregular tetrahedron.
        // Determine which simplex we are in.
        let (i1, j1, k1): (i32, i32, i32); // Offsets for second corner of simplex in (i,j,k) coords
        let (i2, j2, k2): (i32, i32, i32); // Offsets for third corner of simplex in (i,j,k) coords
        if x0 >= y0 {
            if y0 >= z0 {
                i1 = 1;
                j1 = 0;
                k1 = 0;
                i2 = 1;
                j2 = 1;
                k2 = 0;
            } // X Y Z order
            else if x0 >= z0 {
                i1 = 1;
                j1 = 0;
                k1 = 0;
                i2 = 1;
                j2 = 0;
                k2 = 1;
            } // X Z Y order
            else {
                i1 = 0;
                j1 = 0;
                k1 = 1;
                i2 = 1;
                j2 = 0;
                k2 = 1;
            } // Z X Y order
        } else { // x0<y0
            if y0 < z0 {
                i1 = 0;
                j1 = 0;
                k1 = 1;
                i2 = 0;
                j2 = 1;
                k2 = 1;
            } // Z Y X order
            else if x0 < z0 {
                i1 = 0;
                j1 = 1;
                k1 = 0;
                i2 = 0;
                j2 = 1;
                k2 = 1;
            } // Y Z X order
            else {
                i1 = 0;
                j1 = 1;
                k1 = 0;
                i2 = 1;
                j2 = 1;
                k2 = 0;
            } // Y X Z order
        }

        let x1:f64 = x0 - i1 + G3; // Offsets for second corner in (x,y,z) coords
        let y1:f64 = y0 - j1 + G3;
        let z1:f64 = z0 - k1 + G3;
        let x2:f64 = x0 - i2 + 2.0 * G3; // Offsets for third corner in (x,y,z) coords
        let y2:f64 = y0 - j2 + 2.0 * G3;
        let z2:f64 = z0 - k2 + 2.0 * G3;
        let x3:f64 = x0 - 1.0 + 3.0 * G3; // Offsets for last corner in (x,y,z) coords
        let y3:f64 = y0 - 1.0 + 3.0 * G3;
        let z3:f64 = z0 - 1.0 + 3.0 * G3;
        let ii:u8=(i&0xff) as u8;
        let jj:u8=(j&0xff) as u8;
        let kk:u8=(k&0xff) as u8;
        let gi0:u8=self.lookup(ii+self.lookup(jj+self.lookup(kk)))%12;
        let gi1:u8=self.lookup(ii+i1+self.lookup(jj+j1+self.lookup(kk+k1)))%12;
        let gi2:u8=self.lookup(ii+i2+self.lookup(jj+j2+self.lookup(kk+k2)))%12;
        let gi3:u8=self.lookup(ii+1+self.lookup(jj+1+self.lookup(kk+1)))%12;

        // calculate the contribution of the 4 corners
        // should be 0.5 not 0.6 else the noise is not continuous on simplex boundaries but yeah mojang
        let t0:f64 = this.getCornerNoise3D(gi0, x0, y0, z0, 0.6);
        let t1:f64 = this.getCornerNoise3D(gi1, x1, y1, z1, 0.6);
        let t2:f64 = this.getCornerNoise3D(gi2, x2, y2, z2, 0.6);
        let t3:f64 = this.getCornerNoise3D(gi3, x3, y3, z3, 0.6);
        32.0 * (t0 + t1 + t2 + t3)
    }
}

pub const F2: f64 = 0.3660254037844386;
pub const G2: f64 = 0.21132486540518713;
pub const F3: f64 = 0.3333333333333333;
pub const G3: f64 = 0.16666666666666666;
pub const GRADIENT: [[i32; 3]; 16] =
    [[1, 1, 0],
        [-1, 1, 0],
        [1, -1, 0],
        [-1, -1, 0],
        [1, 0, 1],
        [-1, 0, 1],
        [1, 0, -1],
        [-1, 0, -1],
        [0, 1, 1],
        [0, -1, 1],
        [0, 1, -1],
        [0, -1, -1],
        [1, 1, 0],
        [0, -1, 1],
        [-1, 1, 0],
        [0, -1, -1]];