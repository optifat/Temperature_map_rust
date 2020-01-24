#[derive(Copy, Clone)]
pub struct Temperature_map {
    dr: f64,
    dt: f64,
    Nx: i32,
    Ny: i32,
    Nz: i32,
    absorb: f64,
    D: f64,
    k: f64,
    Tm: i32,
    p: f64,
    a: f64,
    v: f64,
    T0: i32,
    factor: f64
}

impl Temperature_map{
    pub fn new(dr: f64, dt: f64, Nx: i32, Ny: i32, Nz: i32,
    absorb: f64, D: f64, k: f64, Tm: i32, p: f64, a: f64, v: f64, T0: i32)->Temperature_map{

        let factor = absorb*p/(std::f64::consts::PI*k/D*(4.0*std::f64::consts::PI*D).powf(0.5));

        Temperature_map{
            dr, dt, Nx, Ny, Nz, absorb, D, k, Tm, p, a, v, T0, factor
        }

    }

    pub fn integrate(self, x: f64, y: f64, z: f64) -> f64{
        let mut integral: f64 = 0.0;
        let y_square: f64 = y*y*self.dr*self.dr;
        let z_square: f64 = z*z*self.dr*self.dr;

        for t in 1..10000 {
            let time = f64::from(t)*self.dt;
            let divider = 1.0/(2.0*self.D*time + self.a*self.a);
            let x_new = x*self.dr + self.v*time;
            let power = (x_new*x_new + y_square)*0.5*divider + z_square/(4.0*self.D*time);

            let result = std::f64::consts::E.powf(-power);
            //println!("{}", result);

            integral += result * self.dt/time.powf(0.5) * divider;
            
        }
        //println!("{}", integral);
        integral *= self.factor;
        
        return integral
    }
}
