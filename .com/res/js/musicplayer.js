var canvas; 
var sheet;
var music;

var pages;
var svgPaths;
var numPages;
var curPage = 0;

function mpInit(json, svgsrcs, audiosrc) {
    pages = JSON.parse(json);
    svgPaths = svgsrcs;
    numPages = pages.length;

    canvas = document.getElementById("player");
    canvas.style.background = "white";
    canvas.style.height = "100vh";
    canvas.style.width = "fit-content";
    canvas.style.margin = "auto";

    sheet = document.createElement("object");
    sheet.setAttribute("data", svgPaths[0]);
    sheet.setAttribute("type", "image/svg+xml");
    sheet.style.height = "100%";
    canvas.appendChild(sheet);

    music = document.createElement("audio");
    music.setAttribute("src", audiosrc);
    canvas.appendChild(music);

    sheet.contentDocument.getElementsByTagName("svg")[0].addEventListener("click", play);
}

function play() {
    var pageSVG = sheet.contentDocument.getElementsByTagName("svg")[0];
    var parts = pages[curPage];
    var svgArrays = {"Note": pageSVG.getElementsByClassName("Note"), "Rest": pageSVG.getElementsByClassName("Rest")};

    for (let i = 0; i < parts.length; i++) {
        MEPlay(parts[i], 0, svgArrays);
    }
    music.play();
}

function MEPlay(part, pIndex, svgArrays) {
    var measureElement = part[pIndex]
    var svgArray = svgArrays[measureElement["class"]];
    svgArray[measureElement["index"]].style.fill = "#0084bf";

    playnext = () => {
        svgArray[measureElement["index"]].style.fill = "black";
        MEPlay(part, pIndex + 1, svgArrays);
    }
    window.setTimeout(playnext, measureElement["duration"]*1000);
}