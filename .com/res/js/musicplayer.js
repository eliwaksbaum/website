var canvas; 
var sheets = [];
var music;

var pages;
var svgPaths;
var numPages;
var displayPage = 0;

var curPage = 0;
var state = [];
var timeouts = [];

var playButton;
var pauseButton;
var stopButton;

var blue = "#0084bf";
var gray = "#2e2e2e";

var panelHTML = `
<svg width="293.3mm" height="39.569mm" version="1.1" viewBox="0 0 293.3 39.569" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" style="width:90%; height: fit-content; margin-left: 5%">
<defs>
<linearGradient id="linearGradient1179" x1="89.079" x2="89.079" y1="59.538" y2="36.208" gradientTransform="matrix(1.3447 0 0 1.5264 -263.48 16.243)" gradientUnits="userSpaceOnUse">
 <stop stop-color="#787878" offset="0"/>
 <stop stop-color="#e5e5e5" offset="1"/>
</linearGradient>
</defs>
<g transform="translate(10.168 -70.38)">
<rect transform="scale(-1,1)" x="-283.13" y="70.38" width="293.3" height="39.569" ry="4.8048" fill="url(#linearGradient1179)" stop-color="#000000"/>
<g transform="translate(7.9722 1.8639)" fill="#2e2e2e" stroke="#2e2e2e" stroke-linecap="round" stroke-linejoin="round">
 <path id="play" transform="matrix(.16795 0 0 .16795 58.311 62.207)" d="m176.37 155.36-94.5 54.559v-109.12z" stop-color="#000000" stroke-width="9.031"/>
 <g id="pause" transform="matrix(.56732 0 0 .56732 79.852 60.148)" stroke-width="2.3894">
  <rect x="71.102" y="33.329" width="6.2955" height="32.589" ry="3.1478" stop-color="#000000"/>
  <rect x="89.804" y="33.329" width="6.2955" height="32.589" ry="3.1478" stop-color="#000000"/>
 </g>
 <rect id="stop" x="166.64" y="79.146" width="18.309" height="18.309" ry="2.0212" stop-color="#000000" stroke-width="1.5343"/>
</g>
<g id="next" fill="#2e2e2e" stroke-linecap="round" stroke-linejoin="round">
 <path transform="matrix(.079918 0 0 .10057 254.58 74.54)" d="m176.37 155.36-94.5 54.559v-109.12z" stop-color="#000000" stroke="#2e2e2e" stroke-width="9.031"/>
 <rect x="267.65" y="84.323" width="2.8587" height="11.783" ry="1.4293" stop-color="#000000" stroke-width="1.12"/>
</g>
<g id="prev" transform="matrix(-1 0 0 1 170.44 -80.965)" fill="#2e2e2e" stroke-linecap="round" stroke-linejoin="round">
 <path transform="matrix(.079918 0 0 .10057 152.77 155.47)" d="m176.37 155.36-94.5 54.559v-109.12z" stop-color="#000000" stroke="#2e2e2e" stroke-width="9.031"/>
 <rect x="165.84" y="165.26" width="2.8587" height="11.783" ry="1.4293" stop-color="#000000" stroke-width="1.12"/>
</g>
</g>
</svg>
`

function mpInit(json, svgsrcs, audiosrc) {
    pages = JSON.parse(json);
    svgPaths = svgsrcs;
    numPages = pages.length;
    resetState();

    canvas = document.getElementById("player");
    canvas.style.background = "white";
    canvas.style.width = "min-content";
    canvas.style.margin = "auto";

    var panel = document.createElement("div");
    panel.innerHTML = panelHTML;
    panel.style.height = "fit-content";
    canvas.appendChild(panel);

    panelsvg = panel.getElementsByTagName("svg")[0];
    playButton = panelsvg.getElementById("play")
    playButton.addEventListener("click", play);
    pauseButton = panelsvg.getElementById("pause")
    //pauseButton.addEventListener("pause", pause);
    stopButton = panelsvg.getElementById("stop");
    stopButton.addEventListener("click", stop);
    panelsvg.getElementById("next").addEventListener("click", next);
    panelsvg.getElementById("prev").addEventListener("click", prev);

    for (let i = 0; i < numPages; i++) {
        var sheet = document.createElement("object");
        sheet.setAttribute("data", svgPaths[i]);
        sheet.setAttribute("type", "image/svg+xml");
        sheet.style.height = "100vh";
        sheet.style.border = "3px black solid";
        sheets.push(sheet);
        canvas.appendChild(sheet);
    }                                       //let's all the pages pre-render i think, flipping without stutter
    for (let i = 1; i < numPages; i++) {
        sheets[i].style.display = "none";
    }

    music = document.createElement("audio");
    music.setAttribute("src", audiosrc);
    canvas.appendChild(music);
}

function resetState() {
    state = new Array(pages[0].length).fill(0);
}

function play() {
    playButton.style.fill = blue;
    playButton.style.stroke = blue;

    sheets[displayPage].style.display = "none";
    displayPage = curPage;
    sheets[displayPage].style.display = "block";

    pagePlay();
    music.play();
}

function stop() {
    music.pause();
    music.currentTime = 0;
    blackout();

    sheets[curPage].style.display = "none";
    curPage = 0;
    displayPage = 0;
    sheets[curPage].style.display = "block";

    for (let i = 0; i < timeouts.length; i++) {
        window.clearTimeout(timeouts[i]);
    }
}

function next() {
    if (displayPage + 1 < numPages) {
        sheets[displayPage].style.display = "none";
        displayPage++;
        sheets[displayPage].style.display = "block";
    }
}
function prev() {
    if (displayPage - 1 >= 0) {
        sheets[displayPage].style.display = "none";
        displayPage--;
        sheets[displayPage].style.display = "block";
    }
}

function pagePlay() {
    var pageSVG = sheets[curPage].contentDocument.getElementsByTagName("svg")[0];
    var svgArrays = {"Note": pageSVG.getElementsByClassName("Note"), "Rest": pageSVG.getElementsByClassName("Rest")};

    var parts = pages[curPage];
    resetState();
    var flag = new Flag(false);
    for (let i = 0; i < parts.length; i++) {
        MEPlay(parts[i], i, state[i], svgArrays, flag);
    }
}

function MEPlay(part, pIndex, eIndex, svgArrays, pageFlag) {
    var measureElement = part[eIndex];
    var svgArray = svgArrays[measureElement["class"]];
    svgArray[measureElement["index"]].style.fill = blue;
    svgArray[measureElement["index"]].style.stroke = blue;
    state[pIndex] = eIndex;

    playnext = eIndex + 1 >= part.length ? () => {
        svgArray[measureElement["index"]].style.fill = "black";
        svgArray[measureElement["index"]].style.stroke = "black";

        if (!pageFlag.value) {
            pageFlag.value = true;
            if (curPage == displayPage) {
                next();
            }
            if (curPage + 1 < numPages) {
                curPage++;
                pagePlay();
            } else {
                curPage = 0;
                playButton.style.fill = gray;
                playButton.style.stroke = gray;
            }
        }
    } : () => {
        svgArray[measureElement["index"]].style.fill = "black";
        svgArray[measureElement["index"]].style.stroke = "black";
        MEPlay(part, pIndex, eIndex + 1, svgArrays, pageFlag);
    }
    timeouts.push(window.setTimeout(playnext, measureElement["duration"]*1000));
}

function blackout() {
    var pageSVG = sheets[curPage].contentDocument.getElementsByTagName("svg")[0];
    var svgArrays = {"Note": pageSVG.getElementsByClassName("Note"), "Rest": pageSVG.getElementsByClassName("Rest")};
    var parts = pages[curPage]

    for (let i = 0; i < parts.length; i++) {
        var measureElement = parts[i][state[i]];
        var svgArray = svgArrays[measureElement["class"]];
        svgArray[measureElement["index"]].style.fill = "black";
        svgArray[measureElement["index"]].style.stroke = "black";
    }

    playButton.style.fill = gray;
    playButton.style.stroke = gray;
}

class Flag {
    //Boolean
    constructor(start) {
        this.value = start;
    }
}