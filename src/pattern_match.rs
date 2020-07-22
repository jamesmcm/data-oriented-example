use rand::Rng;

#[derive(Copy, Clone)]
pub struct Foo {
    x: i32,
    calc_type: CalcType,
}

#[derive(Copy, Clone)]
pub enum CalcType {
    Identity,
    Square,
    Cube,
}

pub fn gen_mixed(n: usize) -> Vec<Foo> {
    let mut out = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for i in 0..n {
        out.push(Foo {
            x: rng.gen_range(0, 5000),
            calc_type: match rng.gen_range(0, 3) {
                0 => CalcType::Identity,
                1 => CalcType::Square,
                2 => CalcType::Cube,
                _ => CalcType::Identity,
            },
        });
    }
    out
}

pub fn gen_separate(n: usize) -> (Vec<Foo>, Vec<Foo>, Vec<Foo>) {
    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    let mut z = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for i in 0..n {
        match rng.gen_range(0, 3) {
            0 => {
                x.push(Foo {
                    x: rng.gen_range(0, 5000),
                    calc_type: CalcType::Identity,
                });
            }
            1 => {
                y.push(Foo {
                    x: rng.gen_range(0, 5000),
                    calc_type: CalcType::Square,
                });
            }
            _ => {
                z.push(Foo {
                    x: rng.gen_range(0, 5000),
                    calc_type: CalcType::Cube,
                });
            }
        };
    }
    (x, y, z)
}

pub fn run_mixed(x: &[Foo]) -> Vec<i32> {
    x.into_iter()
        .map(|x| match x.calc_type {
            CalcType::Identity => x.x,
            CalcType::Square => x.x * x.x,
            CalcType::Cube => x.x * x.x * x.x,
        })
        .collect()
}

pub fn run_separate(x: &[Foo], y: &[Foo], z: &[Foo]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let x = x.into_iter().map(|x| x.x).collect();
    let y = y.into_iter().map(|x| x.x * x.x).collect();
    let z = z.into_iter().map(|x| x.x * x.x * x.x).collect();
    (x, y, z)
}
