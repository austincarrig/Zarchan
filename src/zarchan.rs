/*
zarchan.rs

Written by Austin Carrig, created 4/11/21

Code obtained from Tactical and Strategic Missile Guidance by Paul Zarchan.
*/

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(unused_mut)]

fn main() {
    let path = Path::new("./output/data.txt");

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    
    file.write_all("t ".as_bytes()).expect("Failed to write.");
    file.write_all("rt1 ".as_bytes()).expect("Failed to write.");
    file.write_all("rt2 ".as_bytes()).expect("Failed to write.");
    file.write_all("rm1 ".as_bytes()).expect("Failed to write.");
    file.write_all("rm2 ".as_bytes()).expect("Failed to write.");
    file.write_all("xnc ".as_bytes()).expect("Failed to write.");
    file.write_all("rtm\n".as_bytes()).expect("Failed to write.");
    
    // Constants...
    let vm: f64 = 3000.0;
    let vt: f64 = 1000.0;
    let xnt: f64 = 0.0;
    let hedeg: f64 = -20.0;
    let xnp: f64 = 3.0;
    let mut rm1: f64 = 0.0;
    let mut rm2: f64 = 10000.0;
    let mut rt1: f64 = 40000.0;
    let mut rt2: f64 = 10000.0;

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
    let mut xlead = f64::asin(vt * (xlam + beta).sin() / vm);
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
    while vc >= 0.0 && t < 200.0
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

        // START 66

        beta = beta + h * betad;
        rt1 = rt1 + h*vt1;
        rt2 = rt2 + h*vt2;
        rm1 = rm1 + h*vm1;
        rm2 = rm2 + h*vm2;
        vm1 = vm1 + h*am1;
        vm2 = vm2 + h*am2;
        t = t + h;

        // STOP 66

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

        // START 55

        beta = 0.5*(betaold + beta + h*betad);
        rt1 = 0.5*(rt1old + rt1 + h*vt1);
        rt2 = 0.5*(rt2old + rt2 + h*vt2);
        rm1 = 0.5*(rm1old + rm1 + h*vm1);
        rm2 = 0.5*(rm2old + rm2 + h*vm2);
        vm1 = 0.5*(vm1old + vm1 + h*am1);
        vm2 = 0.5*(vm2old + vm2 + h*am2);

        s = s + h;

        if s >= 0.09999
        {
            s = 0.0;
            
            file.write_all(format!("{} ", t.to_string()).as_bytes()).expect("Failed to write.");
            file.write_all(format!("{} ", rt1.to_string()).as_bytes()).expect("Failed to write.");
            file.write_all(format!("{} ", rt2.to_string()).as_bytes()).expect("Failed to write.");
            file.write_all(format!("{} ", rm1.to_string()).as_bytes()).expect("Failed to write.");
            file.write_all(format!("{} ", rm2.to_string()).as_bytes()).expect("Failed to write.");
            file.write_all(format!("{} ", xnc.to_string()).as_bytes()).expect("Failed to write.");
            file.write_all(format!("{}\n", rtm.to_string()).as_bytes()).expect("Failed to write.");
            //write!(file, "{} ", rt1.to_string());
            //write!(file, "{} ", rt2.to_string());
            //write!(file, "{} ", rm1.to_string());
            //write!(file, "{} ", rm2.to_string());
            //write!(file, "{} ", xnc.to_string());
            //write!(file, "{}\n", rtm.to_string());
            
        }
        // STOP 55
    }

    // ORDER: 10 -> 200 -> 66 -> 200 -> 55 -> 10
    // exit condition is vc < 0

    file.write_all(format!("{} ", t.to_string()).as_bytes()).expect("Failed to write.");
    file.write_all(format!("{} ", rt1.to_string()).as_bytes()).expect("Failed to write.");
    file.write_all(format!("{} ", rt2.to_string()).as_bytes()).expect("Failed to write.");
    file.write_all(format!("{} ", rm1.to_string()).as_bytes()).expect("Failed to write.");
    file.write_all(format!("{} ", rm2.to_string()).as_bytes()).expect("Failed to write.");
    file.write_all(format!("{} ", xnc.to_string()).as_bytes()).expect("Failed to write.");
    file.write_all(format!("{}\n", rtm.to_string()).as_bytes()).expect("Failed to write.");
}