const gr = (Math.sqrt(5) + 1) / 2;
const tol = 1e-5;

export default function gss(f, x1, x2) {
    let g = (x) => {
        if (x < x1 || x > x2) {
            return Number.NEGATIVE_INFINITY;
        }
        else {
            return f(x);
        }
    }
    let a = x1 - .1;
    let b = x2 + .1; 

    let c = b - (b - a) / gr;
    let d = a + (b - a) / gr;

    while (Math.abs(b-a) > tol) {
        if (g(c) > g(d)) {
            b = d;
        }
        else {
            a = c;
        }

        c = b - (b - a) / gr;
        d = a + (b - a) / gr;
    }
    return (b + a) / 2;
}