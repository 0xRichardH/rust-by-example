fn main() {
    let s = &[1, 2, 3, 4, 5];
    // this one is not working -> https://twitter.com/LlambertCEO/status/1673582226573762560?s=20
    // let _ = s.iter().map(|x| println!("{}", x + 1));
    let _ = s.iter().map(|x| println!("{}", x + 1)).collect::<Vec<_>>();
}
