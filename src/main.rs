use std::rc::Rc;
use rand::prelude::*;
use ray_tracing::{
    camera::*,
    color::*,
    hittable::*,
    hittable_list::*,
    material::*,
    ray::*,
    sphere::*,
    vec3::*,
};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, f64::MAX, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let t = 0.5 * (unit_vector(r.dir).y() + 1.0f64);
    (1.0f64 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn create_world() -> HittableList {
    let center1 = Point3::new(0.0, 0.0, -1.0);
    let center2 = Point3::new(0.0, -100.5, -1.0);
    let center3 = Point3::new(1.0, 0.0, -1.0);
    let center4 = Point3::new(-1.0, 0.0, -1.0);

    let material1 = Rc::<Lambertian>::new(Lambertian { albedo: Color::new(0.7, 0.3, 0.3) });
    let material2 = Rc::<Lambertian>::new(Lambertian { albedo: Color::new(0.8, 0.8, 0.0) });
    let material3 = Rc::<Metal>::new(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.0 });
    let material4 = Rc::<Directric>::new(Directric { ref_idx: 1.5 });

    let sphere1 = Sphere::new(center1, 0.5, material1);
    let sphere2 = Sphere::new(center2, 100.0, material2);
    let sphere3 = Sphere::new(center3, 0.5, material3);
    let sphere4 = Sphere::new(center4, 0.5, material4.clone());
    let sphere5 = Sphere::new(center4, -0.45, material4.clone());

    let mut world = HittableList::default();
    world.add(Rc::<Sphere>::new(sphere1));
    world.add(Rc::<Sphere>::new(sphere2));
    world.add(Rc::<Sphere>::new(sphere3));
    world.add(Rc::<Sphere>::new(sphere4));
    world.add(Rc::<Sphere>::new(sphere5));
    world
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let world = create_world();
    let cam = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        16.0 / 9.0);
    let mut rng = rand::rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone.\n");
}