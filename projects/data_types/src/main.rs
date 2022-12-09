fn main() {
    let t = ([1;2], [3;4]);
    // t = ([1,1], [3,3,3,3])
    println!("{}", t.1[2]);
    let (a, _) = t;
    // a = [1,1]
    // _ means ignore, in this case, we only care to destructure into a
    println!("{}", a[0] + t.1[0]  );
    // 1 + 3
    // result = 4
}
