#[derive(Debug)]
struct Points<'a> {
    x: &'a mut Point,
    y: &'a mut Point,
}

#[derive(Debug)]
struct Point {
    p: i32,
}

fn main() {
    let mut x = Point { p: 1 };
    let mut y = Point { p: 2 };
    let p = Points {
        x: &mut x,
        y: &mut y,
    };

    println!("1. {:?}", p);

    // This works.
    p.x.p = 10;
    // This doesn't work.
    // reference: https://www.reddit.com/r/rust/comments/8gbp22/explanation_struct_with_mutable_reference_field/
    // let p2 = &p;
    // p2.x.p = 10;

    println!("2. {:?}", p);

    let mut p1 = Point { p: 1 };
    let mut p2 = Point { p: 2 };
    let mut p3 = Point { p: 3 };
    let mut pv = vec![&mut p1, &mut p2, &mut p3];
    let mut p4 = Point { p: 1 };
    let mut p5 = Point { p: 2 };
    let mut p6 = Point { p: 3 };
    let pv2 = vec![&mut p4, &mut p5, &mut p6];

    println!("3. {:?}", pv);
    println!("4. {:?}", pv2);

    // This works.
    pv[1].p = 1000;
    println!("5. {:?}", pv);

    // This doesn't work.
    // pv2[1].p = 1000;

    // This works.
    let pv_1 = &mut pv[1];
    pv_1.p = 100;
    println!("6. {:?}", pv_1);
    let pv_2 = &mut pv[2];
    pv_2.p = 1000;
    println!("7. {:?}", pv_2);
    // This doesn't work.
    // reference: https://www.reddit.com/r/rust/comments/p60z3e/when_i_have_a_mutable_reference_of_a_vectors/
    // let pv_1 = &mut pv[1];
    // let pv_2 = &mut pv[2];
    // pv_1.p = 100;
    // pv_2.p = 100;

    println!("8. {:?}", pv);
}
