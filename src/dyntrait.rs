use rand::Rng;

pub struct Foo {
    id: usize,
}

pub struct Bar {
    id: usize,
}

pub trait MyTrait {
    fn test(&self) -> String;
    fn square_id(&mut self);
}

impl MyTrait for Foo {
    fn test(&self) -> String {
        format!("Foo_{}", self.id)
    }

    fn square_id(&mut self) {
        self.id = self.id * self.id;
    }
}

impl MyTrait for Bar {
    fn test(&self) -> String {
        format!("Bar_{}", self.id)
    }

    fn square_id(&mut self) {
        self.id = self.id * self.id * self.id;
    }
}

pub fn gen_vecs(n: usize) -> (Vec<Foo>, Vec<Bar>) {
    let mut out = Vec::with_capacity(n);
    let mut out2 = Vec::with_capacity(n);
    for i in 0..n {
        out.push(Foo { id: i });
        out2.push(Bar { id: i });
    }
    (out, out2)
}

pub fn gen_vecs_box(n: usize) -> (Vec<Box<Foo>>, Vec<Box<Bar>>) {
    let mut out = Vec::with_capacity(n);
    let mut out2 = Vec::with_capacity(n);
    for i in 0..n {
        out.push(Box::new(Foo { id: i }));
        out2.push(Box::new(Bar { id: i }));
    }
    (out, out2)
}

pub fn gen_vec_dyn(n: usize) -> Vec<Box<dyn MyTrait>> {
    let mut rng = rand::thread_rng();
    let mut out: Vec<Box<dyn MyTrait>> = Vec::with_capacity(n);
    for i in 0..(2 * n) {
        if rng.gen_bool(0.5) {
            out.push(Box::new(Foo { id: i }));
        } else {
            out.push(Box::new(Bar { id: i }));
        }
    }
    out
}

pub fn run_vecs<T: MyTrait>(x: &Vec<T>) -> Vec<String> {
    x.into_iter().map(|x| x.test()).collect()
}

pub fn run_vecs_box<T: MyTrait>(x: &Vec<Box<T>>) -> Vec<String> {
    x.into_iter().map(|x| x.test()).collect()
}

pub fn run_dyn(x: &Vec<Box<dyn MyTrait>>) -> Vec<String> {
    x.into_iter().map(|x| x.test()).collect()
}

pub fn run_vecs_square<T: MyTrait>(x: &mut Vec<T>) {
    x.iter_mut().for_each(|x| x.square_id())
}

pub fn run_vecs_box_square<T: MyTrait>(x: &mut Vec<Box<T>>) {
    x.iter_mut().for_each(|x| x.square_id())
}

pub fn run_dyn_square(x: &mut Vec<Box<dyn MyTrait>>) {
    x.iter_mut().for_each(|x| x.square_id())
}
