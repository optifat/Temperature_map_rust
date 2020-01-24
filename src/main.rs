mod Temperature_map;

fn main(){
    let p: f64 = 100.0;
    let dr: f64 = 5.0*10.0_f64.powf(-7.0_f64);
    let dt: f64 = 60.0*10.0_f64.powf(-9.0_f64);
    let Nx = 400;
    let Ny = 100;
    let Nz = 150;
    let absorb = 0.25;
    let D: f64 = 5.0*10.0_f64.powf(-6.0_f64);
    let k: f64 = 26.0;
    let Tm = 1563;
    let size: f64 = 50.0*10.0_f64.powf(-6.0_f64);
    let v = 0.8;
    let T0 = 300;

    let A = Temperature_map::Temperature_map::new(dr, dt, Nx, Ny, Nz, absorb, D, k, Tm, p, size, v, T0);

    let mut temp_map: Vec<Vec<f64>> = Vec::with_capacity(Nx as usize);

    for _ in 0..Nx{
        let mut result: Vec<f64> = Vec::with_capacity(Nz as usize);
        for _ in 0..Nz{
            result.push(f64::from(T0))
        }
        temp_map.push(result);
    }

    for i in 0..Nx{
        for j in 0..Nz{
            temp_map[i as usize][j as usize] += A.integrate((i - Nx/2) as f64, 0.0, j as f64);
        }
    }

    let mut depth: i32 = 0;

    for i in 0..Nx{
        for j in 0..Nz{
            //println!("{}", temp_map[i as usize][j as usize]);
            if temp_map[i as usize][j as usize] < Tm as f64{
                if depth < j{
                    depth = j;
                }
                break;
            }
        }
    }

    println!("{}", depth)

}
