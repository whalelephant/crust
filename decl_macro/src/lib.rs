// Declarative macro -> give a pattern for input and give substitution
// Proc macro -> takes rust syntax token stream input and output different token stream, it lets you write rust program

#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($ele: expr),+) => {{
        let mut v = Vec::new();
        // this looks at how many time $ele is repeated
        // and then repeat the same number of times
        $(v.push($ele);)*
        v
    }};
}

#[test]
fn empty_works() {
    let v: Vec<u32> = avec!();
    assert!(v.is_empty());
}
#[test]
fn single_works() {
    let v: Vec<u32> = avec![12];
    assert!(v.len() == 1);
    assert!(v[0] == 12);
}
#[test]
fn double_works() {
    let v: Vec<u32> = avec![12, 13];
    assert!(v.len() == 2);
    assert!(v[0] == 12);
    assert!(v[1] == 13);
}
