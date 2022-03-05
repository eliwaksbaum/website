var canvas; 
var sheets = [];
var music;

var pageDatas;
var svgPaths;
var numPages;
var displayPage = 0;

var pageObjs = [];
var curPage = 0;
var timeouts = [];

var isPlaying = false;
var isPaused = false;

var playButton;
var pauseButton;
var stopButton;

var blue = "#0643f9";
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
<g transform="translate(7.9722 1.8639)">
 <g id="playbox">
  <rect x="68.241" y="76.454" width="22.098" height="23.548" ry="0" fill-opacity="0" stop-color="#000000"/>
  <path id="play" transform="matrix(.16795 0 0 .16795 58.311 62.207)" d="m176.37 155.36-94.5 54.559v-109.12z" fill="#2e2e2e" stop-color="#000000" stroke="#2e2e2e" stroke-linecap="round" stroke-linejoin="round" stroke-width="9.031"/>
 </g>
 <g id="pausebox" transform="matrix(.56732 0 0 .56732 79.852 60.148)">
  <g id="pause" fill="#2e2e2e" stroke="#2e2e2e" stroke-linecap="round" stroke-linejoin="round" stroke-width="2.3894">
   <rect x="71.102" y="33.329" width="6.2955" height="32.589" ry="3.1478" stop-color="#000000"/>
   <rect x="89.804" y="33.329" width="6.2955" height="32.589" ry="3.1478" stop-color="#000000"/>
  </g>
  <rect x="65.363" y="29.768" width="36.257" height="40.014" ry="0" fill-opacity="0" stop-color="#000000"/>
 </g>
 <rect id="stop" x="166.64" y="79.146" width="18.309" height="18.309" ry="2.0212" fill="#2e2e2e" stop-color="#000000" stroke="#2e2e2e" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5343"/>
</g>
<g id="next">
 <path transform="matrix(.079918 0 0 .10057 254.58 74.54)" d="m176.37 155.36-94.5 54.559v-109.12z" fill="#2e2e2e" stop-color="#000000" stroke="#2e2e2e" stroke-linecap="round" stroke-linejoin="round" stroke-width="9.031"/>
 <rect x="267.65" y="84.323" width="2.8587" height="11.783" ry="1.4293" fill="#2e2e2e" stop-color="#000000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.12"/>
 <rect x="258.34" y="81.762" width="14.903" height="17.453" ry="0" fill-opacity="0" stop-color="#000000"/>
</g>
<g id="prev" transform="matrix(-1 0 0 1 170.44 -80.965)">
 <path transform="matrix(.079918 0 0 .10057 152.77 155.47)" d="m176.37 155.36-94.5 54.559v-109.12z" fill="#2e2e2e" stop-color="#000000" stroke="#2e2e2e" stroke-linecap="round" stroke-linejoin="round" stroke-width="9.031"/>
 <rect x="165.84" y="165.26" width="2.8587" height="11.783" ry="1.4293" fill="#2e2e2e" stop-color="#000000" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.12"/>
 <rect transform="scale(-1,1)" x="-170.82" y="162.23" width="14.903" height="17.453" ry="0" fill-opacity="0" stop-color="#000000"/>
</g>
</g>
</svg>
`

function mpInit(json, svgsrcs, audiosrc) {
    pageDatas = JSON.parse(json);
    svgPaths = svgsrcs;
    numPages = pageDatas.length;
    pageObjs = new Array(numPages);

    canvas = document.getElementById("player");
    canvas.style.width = "min-content";
    canvas.style.margin = "auto";

    var panel = document.createElement("div");
    panel.innerHTML = panelHTML;
    panel.style.height = "fit-content";
    canvas.appendChild(panel);

    panelsvg = panel.getElementsByTagName("svg")[0];

    playButton = panelsvg.getElementById("play");
    panelsvg.getElementById("playbox").addEventListener("click", play);

    pauseButton = panelsvg.getElementById("pause");
    panelsvg.getElementById("pausebox").addEventListener("click", pause);

    stopButton = panelsvg.getElementById("stop");
    stopButton.addEventListener("click", stop);

    panelsvg.getElementById("next").addEventListener("click", next);
    panelsvg.getElementById("prev").addEventListener("click", prev);

    for (let i = 0; i < numPages; i++) {
        var sheet = document.createElement("object");
        sheet.setAttribute("data", svgPaths[i]);
        sheet.setAttribute("type", "image/svg+xml");
        sheet.style.height = "100vh";
        sheet.style.background = "white";
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

function play() {
    if (!isPlaying) {
        playButton.style.fill = blue;
        playButton.style.stroke = blue;

        if (isPaused) {
            pauseButton.style.fill = gray;
            pauseButton.style.stroke = gray;

            isPaused = false;
            Timer.resume();
        } else {
            sheets[displayPage].style.display = "none";
            displayPage = curPage;
            sheets[displayPage].style.display = "block";

            pagePlay()
        }

        isPlaying = true;
        music.play();
    }
}

function pagePlay() {
    if (pageObjs[curPage] == null) {
        pageObjs[curPage] = new Page(curPage);
    }
    pageObjs[curPage].play();
}

function stop() {
    music.pause();
    music.currentTime = 0;
    pageObjs[curPage].stop();
    isPlaying = false;
    isPaused = false;

    playButton.style.fill = gray;
    playButton.style.stroke = gray;
    pauseButton.style.fill = gray;
    pauseButton.style.stroke = gray;

    sheets[curPage].style.display = "none";
    curPage = 0;
    displayPage = 0;
    sheets[curPage].style.display = "block";

    Timer.stop();
}

function pause() {
    if (isPlaying && !isPaused) {
        playButton.style.fill = gray;
        playButton.style.stroke = gray;
        pauseButton.style.fill = blue;
        pauseButton.style.stroke = blue;

        isPlaying = false;
        isPaused = true;
        Timer.pause();
        music.pause();
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

class Page {
    constructor(num) {
        this.pageSVG = sheets[num].contentDocument.getElementsByTagName("svg")[0];
        this.svgArrays = {"Note": this.pageSVG.getElementsByClassName("Note"), "Rest": this.pageSVG.getElementsByClassName("Rest")};
        this.parts = pageDatas[num];
        this.flag = false;
        this.state = [];
        this.resetState();
    }

    resetState() {
        this.state = new Array(this.parts[0].length).fill(0);
    }

    play() {
        this.flag = false;
        for (let i = 0; i < this.parts.length; i++) {
            this.MEPlay(this.parts[i], i, this.state[i]);
        }
    }

    MEPlay(part, pIndex, eIndex) {
        var measureElement = part[eIndex];
        var meStyle = this.svgArrays[measureElement["class"]][measureElement["index"]].style
        meStyle.fill = blue;
        meStyle.stroke = blue;
        this.state[pIndex] = eIndex;
    
        var playnext = eIndex + 1 >= part.length ? () => {
            meStyle.fill = "black";
            meStyle.stroke = "black";

            if (!this.flag) {
                this.flag = true;
                this.resetState();
                if (curPage == displayPage) {
                    next();
                }
                if (curPage + 1 < numPages) {
                    curPage++;
                    pagePlay();
                } else {
                    curPage = 0;
                    isPlaying = false;
                    playButton.style.fill = gray;
                    playButton.style.stroke = gray;
                }
            }
        } : () => {
            meStyle.fill = "black";
            meStyle.stroke = "black";
            this.MEPlay(part, pIndex, eIndex + 1);
        }
        new Timer(playnext, measureElement["duration"]*1000);
    }

    stop() {
        this.blackout();
        this.resetState();
    }

    blackout() {
        for (let i = 0; i < this.parts.length; i++) {
            var measureElement = this.parts[i][this.state[i]];
            var svgArray = this.svgArrays[measureElement["class"]];
            svgArray[measureElement["index"]].style.fill = "black";
            svgArray[measureElement["index"]].style.stroke = "black";
        }
    }

}

class Timer {
    constructor(call, wait) {
        this.call = () => {Timer.timers.delete(this); call();};
        this.id = window.setTimeout(this.call, wait);
        this.start = Date.now();
        this.elapsed = 0;
        this.remaining = wait;
        Timer.timers.add(this);
    }

    pause() {
        window.clearTimeout(this.id);
        this.elapsed = Date.now() - this.start;
        this.remaining -= this.elapsed;
    }

    resume() {
        this.id = window.setTimeout(this.call, this.remaining);
        this.start = Date.now();
    }

    stop() {
        window.clearTimeout(this.id);
        Timer.timers.delete(this);
    }

    static timers = new Set();
    static pause() {
        for (let t of this.timers) {
            t.pause();
        }
    }
    static resume() {
        for (let t of this.timers) {
            t.resume();
        }
    }
    static stop() {
        for (let t of this.timers) {
            t.stop();
        }
    }

}