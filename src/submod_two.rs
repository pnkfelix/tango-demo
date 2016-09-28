pub fn thrice<F, X>(f: F, x: X) -> X
    where F: Fn(X) -> X
{
    f(f(f(x)))
}
