var css;
var button;

var colors = ["#e772ac", "#9edf5e", "skyblue", "tomato"]

var theme_preference = localStorage.getItem("theme");
if (theme_preference == null) {
    theme_preference = "light";
}

var current_section = window.location.href.split("/")[3];

function setCss(theme) {
    css.href = "/res/stylesheets/" + theme + ".css";
}

function setButton(theme) {
    var anti;
    if (theme == "light") {
        anti = "dark";
    }
    else {
        anti = "light";
    }
    button.src = "/res/svg/" + anti + ".svg";
    button.alt = anti;
}

function setTheme(theme) {
    setButton(theme);
    setCss(theme);
    colorHeader(theme);
    loadThemedImages(theme);
}

function toggleTheme() {
    if (css.getAttribute("href") == "/res/stylesheets/light.css") {
        setTheme("dark");
        localStorage.setItem("theme", "dark");
    }
    else {
        setTheme("light");
        localStorage.setItem("theme", "light");
    }
}

function findCss() {
    css = document.getElementById("theme");
    setCss(theme_preference);
}

function findButton() {
    button = document.getElementById("theme-button");
    setButton(theme_preference);
    button.addEventListener("click", toggleTheme);
}

function colorHeader(theme) {
    var header = document.getElementById("header");
    var i = Math.floor((Math.random() * colors.length));
    var color = colors[i];
    
    if (theme == "light") {
        header.style.backgroundColor = "skyblue";
        header.style.color = 
    }
    else {
        header.style.color = "lightblue";
    }
}

function loadThemedImages(theme) {
    var imgs = document.getElementsByClassName("themed");
    for (img of imgs) {
        var themedSrc = img.getAttribute("data-themedSrc");
        var src = themedSrc.replace("theme", theme);
        img.src = src;
    }
}