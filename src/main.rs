extern crate rusttracer;

use rusttracer::math;

fn main() {
    let vec1 = math::Vector3 {
        x: 10.0,
        y: 3.0,
        z: 0.0,
    };

    let vec2 = math::Vector3 {
        x: 1.3,
        y: 5.0,
        z: -3.0,
    };
    //    println!("This is the vector: {:?}", vec);

    println!("&vec1 + &vec2 = {:?}", &vec1 + &vec2);
    println!("vec1={:?}, vec2={:?}", vec1, vec2);
    println!("vec1 + vec2 = {:?}", vec1 + vec2);

    let a = 10;
    let b = 20;
    let c = a + b;
    println!("{}", b);
}
