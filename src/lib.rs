#[test]
fn basic diff() {
    let exp1 = poly([2,4],"x");
    let exp1_diff = differentiate(exp1,"x");
    if xdiff != 2
    {
    fail!("Fail")
}

struct Polynomial {
    coeffs: Vec<Vec<int>>,
    variable: Vec<String>;
}

enum Expression{
    Polynomial

fn poly(vec: Vec<int>, var: String) -> Polynomial {
    /// A function for making a single polynomial
    /// This wants a vector of coeffecients and a name to bind the variables toc
    Polynomial{coeffs: Vec[Vec[vec]], variable: var}
}

fn differentiate(exp: T) -> T {
    match T


