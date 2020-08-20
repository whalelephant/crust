// This file plays with writing a vec macro called avec!
// Declarative macro -> give a pattern for input and give substitution
// Proc macro -> takes rust syntax token stream input and output different token stream, it lets you write rust program

#[macro_export]
macro_rules! avec {

    // this is saying the pattern is ele must be at least one
    // e.g. x, y, z and does not allow for trailing comma
    // ($($ele: expr),+) => {{
    // so we need addition zero/one for comma and 0 to multiple for ele, but this allows for avec![,]

    // ($($ele: expr),* $(,)?) => {{

    // so instead lets add another pattern and translate
    ($($ele: expr),*) => {{
        #[allow(unused_mut)]

        // this looks at how many time $ele is repeated
        // and then repeat the same number of times, but this will reallocate
        // let mut v = Vec::new();

        // what if we want to create with capacity, there isn't actually a way direct way to count the repeated pattern
        // https://danielkeep.github.io/tlborm/book/blk-counting.html#slice-length
        let mut v  = Vec::with_capacity($crate::count![@COUNT; $($ele),*]);
        $(v.push($ele);)*
        v
    }};

    // Translate dealing with zeros
    ($($ele: expr,)* ) => {{
        $crate::avec![$($ele),*]
    }};

    ($ele: expr; $count: expr) => {{
        // we need this to evaluate only once (for if use option.take())
        let c = $count;
        // or just do Vec::new() and let it reallocate if count > default
        let mut v = Vec::with_capacity(c);
        // Instead of using push, needing to check bound every time,
        // can use extend for global check? trait bound of root level crate ::std::iter::repeat x must impl clone
        // v.extend(::std::iter::repeat(x).take(c));
        // or use resize, does not check bound
        v.resize(c, $ele);
        v
    }}
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    // @COUNT just a pattern to match
    (@COUNT; $($ele: expr),*) => {
        // this invokes the len method of a empty slide on a slice with ele type () - zero mem to get the length (or the #of ele)
        <[()]>::len(&[$($crate::count!(@SUBST; $ele: expr)),*])
    };
    // sub whatever expr is into ()
    (@SUBST; $ele: expr) => {()};
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
    let v: Vec<u32> = avec![12, 13,];
    assert!(v.len() == 2);
    assert!(v[0] == 12);
    assert!(v[1] == 13);
}

#[test]
fn clone_works() {
    let mut x = Some(15);
    let v: Vec<u32> = avec![x.take().unwrap(); 2];
    assert!(v.len() == 2);
    assert!(v[0] == 15);
    assert!(v[1] == 15);
}

/// ```compile_fail
/// Here macro expect int as count not &str
/// let x = decl_macro::avec![14; "foo"];
///```
pub struct CompileFailTest;
