/*
zarchan.rs

Written by Austin Carrig, created 4/11/21

Code obtained from Tactical and Strategic Missile Guidance by Paul Zarchan.
*/
#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(unused_mut)]

fn main() {
    // Constants...
    let vm: f64 = 3000.0;
    let vt: f64 = 1000.0;
    let xnt: f64 = 0.0;
    let hedeg: f64 = 20.0;
    let xnp: f64 = 3.0;
    let rm1: f64 = 0.0;
    let rm2: f64 = 10000.0;
    let rt1: f64 = 40000.0;
    let rt2: f64 = 10000.0;

    let mut beta: f64 = 0.0;

    let mut vt1: f64 = -vt*beta.cos();
    let mut vt2: f64 = vt*beta.sin();

    let mut he: f64 = hedeg.to_radians();

    let mut t: f64 = 0.0;
    let mut s: f64 = 0.0;
    let mut rtm1 = rt1 - rm1;
    let mut rtm2 = rt2 - rm2;
    let mut rtm = f64::sqrt(rtm1.powi(2) + rtm2.powi(2));
    let mut xlam = rtm2.atan2(rtm1);
    let mut xlead = f64::asin(vt*(xlam + beta).sin()/vm);
    let mut thet = xlam + xlead;
    let mut vm1 = vm*(thet + he).cos();
    let mut vm2 = vm*(thet + he).sin();
    let mut vtm1 = vt1 - vm1;
    let mut vtm2 = vt2 - vm2;
    let mut vc = -(rtm1*vtm1 + rtm2*vtm2)/rtm;

    let mut am1: f64 = 0.0;
    let mut am2: f64 = 0.0;
    let mut xnc: f64 = 0.0;

    // START 10
    if vc >= 0.0
    {
        let mut h: f64 = 0.0;
        if rtm < 1000.0
        {
            h = 0.0002
        }
        else
        {
            h = 0.01
        }

        let mut betaold = beta;
        let mut rt1old = rt1;
        let mut rt2old = rt2;
        let mut rm1old = rm1;
        let mut rm2old = rm2;
        let mut vm1old = vm1;
        let mut vm2old = vm2;
        let mut step: i8 = 1;
        // STOP 10

        // START 200
        rtm1 = rt1 - rm1;
        rtm2 = rt2 - rm2;
        rtm = f64::sqrt(rtm1.powi(2) + rtm2.powi(2));
        vtm1 = vt1 - vm1;
        vtm2 = vt2 - vm2;
        vc = -(rtm1*vtm1 + rtm2*vtm2) / rtm;
        xlam = rtm2.atan2(rtm1);
        let xlamd = (rtm1*vtm2 - rtm2*vtm1) / rtm.powi(2);
        xnc = xnp*vc*xlamd;
        am1 = -xnc*xlam.sin();
        am2 = xnc*xlam.cos();
        vt1 = -vt*beta.cos();
        vt2 = vt*beta.sin();
        let betad = xnt / vt;

        // STOP 200
    }

    // ORDER: 10 -> 200 -> 66 -> 200 -> 55 -> 10
    // exit condition is vc < 0

    println!("T  : {}", t.to_string());
    println!("RT1: {}", rt1.to_string());
    println!("RT2: {}", rt2.to_string());
    println!("RM1: {}", rm1.to_string());
    println!("RM2: {}", rm2.to_string());
    println!("XNC: {}", xnc.to_string());
    println!("RTM: {}", rtm.to_string());
}