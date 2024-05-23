pub fn foobaaz() {
    let x = if 1 > 2 { true } else { false };
}

struct A {
    a: i32,
}
struct B {
    a: i32,
}

trait CustomTrait {
    fn calculate(&self) -> i32;
    fn calculate2(&self) -> i32;
}

impl CustomTrait for A {
    fn calculate(&self) -> i32 {
        self.a * 2
    }
    fn calculate2(&self) -> i32 {
        self.a * 3
    }
}
impl CustomTrait for B {
    fn calculate(&self) -> i32 {
        let c = self.a * 2;
        println!("{c}");
        c
    }
    fn calculate2(&self) -> i32 {
        let c2 = self.a * 3;
        println!("{c2}");
        c2
    }
}

impl B {
    fn just_return(&self) -> i32 {
        self.a
    }
}

pub fn use_a(arg: impl CustomTrait) {
    arg.calculate();
    arg.calculate2();
}

pub fn do_a() {
    use_a(B { a: 123 })
}

#[test]
fn do_test() {
    let my_b = B { a: 42 };
    assert!(my_b.just_return() == my_b.a);
}

pub fn do_loop() {
    let arr = [1, 2, 3, 4];
    for x in arr {
        println!("{x}")
    }

    let mut v = Vec::new();
    println!("{} {}", v.len(), v.capacity());
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    println!("{} {}", v.len(), v.capacity());
}
