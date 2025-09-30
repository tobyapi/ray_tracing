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

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Rc::<Lambertian>::new(Lambertian { albedo: Color::new(1.0, 1.0, 1.0) });
    let ground_sphere = Rc::<Sphere>::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));
    world.add(ground_sphere);

    let mut rng = rand::rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc::<dyn Material> =
                    if choose_mat < 0.8 {
                        let albedo = Color::random() * Color::random();
                        Rc::<Lambertian>::new(Lambertian { albedo })
                    } else if choose_mat < 0.95 {
                        let albedo = Color::random();
                        //let albedo = Color::random(0.5..1.0);
                        let fuzz = rng.random_range(0.0..0.5);
                        Rc::<Metal>::new(Metal { albedo, fuzz })
                    } else {
                        Rc::<Directric>::new(Directric { ref_idx: 1.5 })
                    };
                world.add(Rc::<Sphere>::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let center1 = Point3::new(0.0, 1.0, 0.0);
    let center2 = Point3::new(-4.0, 1.0, 0.0);
    let center3 = Point3::new(4.0, 1.0, 0.0);

    let material1 = Rc::<Directric>::new(Directric { ref_idx: 1.5 });
    let material2 = Rc::<Lambertian>::new(Lambertian { albedo: Color::new(0.4, 0.2, 0.1) });
    let material3 = Rc::<Metal>::new(Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 });

    let sphere1 = Sphere::new(center1, 1.0, material1);
    let sphere2 = Sphere::new(center2, 1.0, material2);
    let sphere3 = Sphere::new(center3, 1.0, material3);

    world.add(Rc::<Sphere>::new(sphere1));
    world.add(Rc::<Sphere>::new(sphere2));
    world.add(Rc::<Sphere>::new(sphere3));
    world
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 384;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        16.0 / 9.0,
        aperture,
        dist_to_focus);
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