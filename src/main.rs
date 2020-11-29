use ark_bls12_381::{G1Projective, G2Projective, Bls12_381};
use ark_ff::{UniformRand, One};
use ark_ec::{PairingEngine, ProjectiveCurve};

use rand::thread_rng;

fn main() {
    let rng = &mut thread_rng();
    // This calculates a*b + (-a)*b, so should be one in the target group.
    let a = G1Projective::rand(rng);
    let b = G2Projective::rand(rng);
    let c = -a;
    let m = Bls12_381::miller_loop(vec![
        (a.into_affine().into(), b.into_affine().into()),
        (c.into_affine().into(), b.into_affine().into()),
    ].iter());
    println!("miller loop result is one: {}", m.is_one());
    let f = Bls12_381::final_exponentiation(&m).unwrap();
    println!("final exp result is one: {}", f.is_one());
}
