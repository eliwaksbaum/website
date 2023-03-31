export const max_rounds = 7;

const eo_cache = new Array(Math.pow(2, max_rounds) + 1);
for (let i = 1; i < eo_cache.length; i++) {
    eo_cache[i] = new Array(max_rounds + 1);
}

function expected_oponent(n, t) {
    let logt = Math.log2(t);
    if (eo_cache[n][logt] == undefined) {
        eo_cache[n][logt] = calc_eo(n, t);
    }
    return eo_cache[n][logt];
}
function calc_eo(n, t) {
    if (n <= t) {
        return t+1 - n;
    }
    else {
        return expected_oponent(expected_oponent(n, 2*t), t);
    }
}

function CreateTournament(num_rounds) {
    const r = num_rounds;
    const T = Math.pow(2, r);

    const hb_cache = new Array(r+1);
    const R_cache = new Array(r+1);
    for (let i = 0; i < hb_cache.length; i++) {
        hb_cache[i] = new Array(T+1);
        R_cache[i] = new Array(T+1);
        for (let j = 0; j < T+1; j++) {
            R_cache[i][j] = new Map();
        }
    }

    function home_bracket(k, n) {
        if (hb_cache[k][n] == undefined) {
            hb_cache[k][n] = calc_hb(k, n);
        }
        return hb_cache[k][n];
    }
    function calc_hb(k, n) {
        if (k == 1) {
            return [n];
        }
        else {
            return home_bracket(k-1, n).concat(opposing_bracket(k-1, n));
        }
    }

    function opposing_bracket(k, n) {
        const t = T / Math.pow(2, k-1)
        return home_bracket(k, expected_oponent(n, t));
    }


    function w(n, m, p) {
        if (n < m) {
            return p;
        }
        else if (n > m) {
            return 1 - p;
        }
        else {
            return 1;
        }
    }

    function R(k, n, p) {
        if (!R_cache[k][n].has(p)) {
            R_cache[k][n].set(p, calc_R(k, n, p));
        }
        return R_cache[k][n].get(p);
    }

    function calc_R(k, n, p) {
        if (k == 0) {
            return 1;
        }
        else {
            let sum = 0;
            
            opposing_bracket(k, n).forEach( m => {
                sum += w(n, m, p) * R(k-1, m, p);
            });

            return R(k-1, n, p) * sum;
        }
    }

    return { R };
}

const Tournaments = [];
for (let i = 0; i < max_rounds + 1; i++) {
    if (i == 0 || i == 1) {
        Tournaments.push(0);
    }
    else {
        Tournaments.push(CreateTournament(i));
    }
}

export function CreateSeedGrapher(seed) {
    const n = seed;

    function is_in_tournament(T) { return n <= T; }

    function get_curve(r, k) {
        return p => Tournaments[r].R(k, n, p);
    }

    return { is_in_tournament, get_curve }
}