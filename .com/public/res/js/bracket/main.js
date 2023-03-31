import { CreateSeedGrapher, max_rounds } from "./bracket.js";
import gss from "./golden_section.js";

var board;
var r_slider;
var k_slider;
var k_at_max;

const r = () => { return r_slider.value; }
const k = () => { return k_slider.value; }
const T = () => { return Math.pow(2, r()); }

const tracked_seeds = new Map();
const colors = ["#3333ff", "#e60000", "#2eb82e", "#4d2600", "#c806f9", "#ff751a", "#00e6e6", "#a6ff4d"];
let color_i = 0;

window.init = function() {
    board = JXG.JSXGraph.initBoard('box', {
        boundingbox: [-0.1, 1.1, 1.1, -0.1],
        axis: true,
        grid: true,
        showNavigation: false
    });
    r_slider = document.getElementById("r_slider");
    k_slider = document.getElementById("k_slider");

    track_seed(2);

    r_slider.max = max_rounds;
    r_slider.value = 3;
    update_T(3);
    k_at_max = true;
}

window.plot = function () {
    const n = document.getElementById('n_input').value;
    const num = Number.parseInt(n, 10);
    if (!Number.isNaN(num) && !tracked_seeds.has(num)) {
        track_seed(n);
        update_curves();
    }
}

window.update_T = function () {
    document.getElementById("T_label").innerHTML = T();

    k_slider.max = r();
    if (k() >= r() || k_at_max) {
        k_slider.value = r();
        k_at_max = true;
    }
    
    update_k(k());
    update_curves();
}

window.update_k = function (new_k) {
    k_at_max = new_k == r();

    let readout;
    if (new_k == r()) {
        readout = new_k + " (wins the tournament)";
    }
    else if (new_k == r() - 1) {
        readout = new_k + " (makes it to the finals)";
    }
    else if (new_k == r() - 2) {
        readout = new_k + " (makes it to the semis)";
    }
    else {
        readout = new_k;
    }

    document.getElementById("k_label").innerHTML = readout;
    update_curves();
}

function track_seed(n) {
    const color = colors[color_i % 8];
    color_i++;

    const template = document.getElementById("seed_template");
    const clone = template.content.cloneNode(true);

    const row = clone.querySelector("div.seed-row");

    //seed number text
    clone.querySelector("span.seed-text").textContent = n;

    //seed box color
    clone.querySelector("svg").style.fill = color;

    //highlight on hover
    row.onmouseenter = (e) => {
        const curve = tracked_seeds.get(n).curve;
        curve.setAttribute({ strokeOpacity: 1 });
    }
    row.onmouseleave = (e) => {
        const curve = tracked_seeds.get(n).curve;
        curve.setAttribute({ strokeOpacity: .5 });
    }

    //button
    clone.querySelector("input").onclick = (e) => {
        const curve = tracked_seeds.get(n).curve;
        board.removeObject(curve.id);
        tracked_seeds.delete(n);
        row.remove();
    }

    document.getElementById("seed_table").appendChild(clone);
    
    const grapher = CreateSeedGrapher(n);
    const curve = board.create('functiongraph', [grapher.get_curve, 0, 1]);
    
    tracked_seeds.set(n, { "grapher": grapher, "curve": curve, "color": color });

}

function update_curves() {
    board.suspendUpdate();

    //removing in reverse order for performance
    Array.from(tracked_seeds.values()).reverse().forEach( team => {
        board.removeObject(team.curve.id);
    });

    tracked_seeds.forEach( team => {
        if (team.grapher.is_in_tournament(T())) {
            const f = team.grapher.get_curve(r(), k());            

            const graph = board.create('functiongraph', [f, 0, 1 ], {
                strokeWidth: 2,
                
                strokeColor: team.color,
                strokeOpacity: .5,
                
                highlightStrokeColor: team.color,
                highlightStrokeOpacity: 1
            });

            const max_p = gss(f, 0, 1);
            const max = board.create('point', [() => { return max_p; }, () => { return f(max_p); }], {
                size: 1,
                showInfobox: false,
                
                strokeWidth: 2,
                strokeColor: team.color,
                strokeOpacity: .5,
                fillColor: team.color,
                fillOpacity: .5,
                
                highlightStrokeColor: team.color,
                highlightStrokeOpacity: 1,
                highlightFillColor: team.color,
                highlightFillOpacity: 1
            });

            max.setLabel(`(${max_p.toFixed(4)}, ${f(max_p).toFixed(4)})`);
            const l = Object.values(max.childElements)[0];
            max.on('over', () => { l.show(); });
            max.on('out', () => { l.hide(); });
            l.hide();
            
            graph.addChild(max);
            
            team.curve = graph;
        }
    });

    board.unsuspendUpdate();
}