// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    // original
    // let vec0 = Vec::new();

    // let mut vec1 = fill_vec(vec0);

    // // Do not change the following line!
    // println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // vec1.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // -----------------
    // separate variable (first idea from hint)
    let vec0 = Vec::<i32>::new();
    let vec00 = Vec::<i32>::new();

    let mut vec1 = fill_vec(vec00);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // -----------------
    // fill_vec borrows vec (second idea from hint)
    let vec20 = Vec::new();

    let vec21 = fill_vec2(&vec20);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec20.len(), vec20);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec21.len(), vec21);

    // -----------------
    // fill_vec mutably borrows vec (third idea from hint)
    let mut vec30 = Vec::new();

    fill_vec3(& mut vec30);
    fill_vec3(& mut vec30);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec30.len(), vec30);

    vec1.push(88);

    //println!("{} has length {} content `{:?}`", "vec1", vec31.len(), vec31);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);   
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec2(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec.to_vec();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec3(vec: & mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
