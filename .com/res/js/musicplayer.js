var canvas; 
var sheet;

var pages;
var svgPaths;
var audioPath;
var numPages;
var curPage = 0;

function mpInit(json, svgsrcs, audiosrc) {
    canvas = document.getElementById("player");
    pages = JSON.parse(json);
    svgPaths = svgsrcs;
    audioPath = audiosrc;
    numPages = pages.length;

    sheet = document.createElement("object");
    sheet.setAttribute("data", svgPaths[0]);
    sheet.setAttribute("type", "image/svg+xml");
    canvas.appendChild(sheet);
}

function play(pageIndex) {
    var pageSVG = document.getElementById("music");
    var parts = pages[pageIndex];
    var svgArrays = {"Note": pageSVG.getElementsByClassName("Note"), "Rest": pageSVG.getElementsByClassName("Rest")};
    var audio = document.getElementById("audio");

    for (let i = 0; i < parts.length; i++) {
        MEPlay(parts[i], 0, svgArrays);
    }
    audio.play();
}

function MEPlay(part, pIndex, svgArrays) {
    var measureElement = part[pIndex]
    var svgArray = svgArrays[measureElement["class"]];
    svgArray[measureElement["index"]].style.fill = "skyblue";

    playnext = () => {
        svgArray[measureElement["index"]].style.fill = "black";
        MEPlay(part, pIndex + 1, svgArrays);
    }
    window.setTimeout(playnext, measureElement["duration"]*1000);
}