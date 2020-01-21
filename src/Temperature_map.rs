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
}

impl Temperature_map{
    pub fn new(dr: f64, dt: f64, Nx: i32, Ny: i32, Nz: i32,
    absorb: f64, D: f64, k: f64, Tm: i32, p: f64, a: f64, v: f64, T0: i32)->Temperature_map{

        Temperature_map{
            dr, dt, Nx, Ny, Nz, absorb, D, k, Tm, p, a, v, T0
        }

    }

    pub fn integrate(self, x: f64, y: f64, z: f64) -> f64{
        let mut integral: f64 = 0.0;
        let value: f64 = self.absorb*self.p/(std::f64::consts::PI*self.k/self.D*(4.0*std::f64::consts::PI*self.D).powf(0.5));
        for t in 1..10000 {
            let t = f64::from(t);

            let power: f64 = ((x*self.dr+self.v*t*self.dt).powf(2.0) +
            (y*self.dr).powf(2.0))/(4.0*self.D*t*self.dt+2.0*self.a*self.a) +
            (z*self.dr).powf(2.0)/(4.0*self.D*t*self.dt);

            let result = std::f64::consts::E.powf(-power);
            //println!("{}", result);

            integral += result * self.dt/(t*self.dt).powf(0.5)/(2.0*self.D*t*self.dt + self.a.powf(2.0)) * value;
            
        }
        //println!("{}", integral);
        return integral
    }
}
