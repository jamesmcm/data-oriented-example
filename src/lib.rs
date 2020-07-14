use rand::Rng;

pub struct Player {
    name: String,
    health: f64,
    location: (f64, f64),
    velocity: (f64, f64),
    acceleration: (f64, f64),
}

pub struct DOPlayers {
    names: Vec<String>,
    health: Vec<f64>,
    locations: Vec<(f64, f64)>,
    velocities: Vec<(f64, f64)>,
    acceleration: Vec<(f64, f64)>,
}

pub fn gen_oop(n: usize) -> Vec<Player> {
    let mut rng = rand::thread_rng();
    let mut players = Vec::with_capacity(n);
    for i in 0..n {
        players.push(Player {
            name: format!("player_name_{}", i),
            health: 100.0,
            location: (rng.gen_range(0.0, 10.0), rng.gen_range(0.0, 10.0)),
            velocity: (rng.gen_range(0.0, 10.0), rng.gen_range(0.0, 10.0)),
            acceleration: (rng.gen_range(0.0, 10.0), rng.gen_range(0.0, 10.0)),
        });
    }
    players
}

pub fn gen_dop(n: usize) -> DOPlayers {
    let mut rng = rand::thread_rng();

    let mut names = Vec::with_capacity(n);
    let mut health = Vec::with_capacity(n);
    let mut locations = Vec::with_capacity(n);
    let mut velocities = Vec::with_capacity(n);
    let mut acceleration = Vec::with_capacity(n);

    for i in 0..n {
        names.push(format!("player_name_{}", i));
        health.push(100.0);
        locations.push((rng.gen_range(0.0, 10.0), rng.gen_range(0.0, 10.0)));
        velocities.push((rng.gen_range(0.0, 10.0), rng.gen_range(0.0, 10.0)));
        acceleration.push((rng.gen_range(0.0, 10.0), rng.gen_range(0.0, 10.0)));
    }
    DOPlayers {
        names,
        health,
        locations,
        velocities,
        acceleration,
    }
}

pub fn run_oop(players: &mut Vec<Player>) {
    for player in players.iter_mut() {
        player.location = (
            player.location.0 + player.velocity.0,
            player.location.1 + player.velocity.1,
        );
        player.velocity = (
            player.velocity.0 + player.acceleration.0,
            player.velocity.1 + player.acceleration.1,
        );
    }
}

pub fn run_dop(world: &mut DOPlayers) {
    for (pos, (vel, acc)) in world
        .locations
        .iter_mut()
        .zip(world.velocities.iter_mut().zip(world.acceleration.iter()))
    {
        *pos = (pos.0 + vel.0, pos.1 + vel.1);
        *vel = (vel.0 + acc.0, vel.1 + acc.1);
    }
}

fn main() {
    println!("Hello, world!");
}
