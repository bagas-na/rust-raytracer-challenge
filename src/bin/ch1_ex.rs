use rust_raytracer_challenge::tuple::Tuple;

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

fn main() {
    // Initialize projectile and environment.
    let mut proj = Projectile {
        position: Tuple::new_point(0.0, 1.0, 0.0),
        velocity: Tuple::new_vector(1.0, 1.0, 0.0),
    };

    let env = Environment {
        gravity: Tuple::new_vector(0.0, -0.1, 0.0),
        wind: Tuple::new_vector(-0.01, 0.0, 0.0),
    };

    println!("position: {}, velocity: {}", proj.position, proj.velocity);

    while proj.position.data.y > 0.0 {
        tick(&env, &mut proj);
        println!("position: {}, velocity: {}", proj.position, proj.velocity);
    }
}
