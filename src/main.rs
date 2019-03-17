#![feature(const_fn)]

mod lib;

use lib::{Vec3, Ray};

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let end = &lower_left_corner + &(&(u * &horizontal) + &(v * &vertical));
            let r = Ray::new(&origin, &end);
            let col = color(&r);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn hit_sphere(center : &Vec3, radius : f64, r : &Ray) -> bool {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * &Vec3::dot(&oc, r.direction());
    let c = &Vec3::dot(&oc, &oc) - &(radius * radius);
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

fn color(r : &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    &((1.0 - t) * &Vec3::new(1.0, 1.0, 1.0))  + &(t * &Vec3::new(0.5, 0.7, 1.0))
}
