var css;
var button;

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
    /*highlightNav(theme);*/
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

function highlightNav(theme) {
    var root = document.documentElement;
    var style = getComputedStyle(root);
    var color = style.getPropertyValue("--" + current_section);
    var dark = style.getPropertyValue("--dark-menu-hover");
    var light = style.getPropertyValue("--light");

    var nav = document.getElementById(current_section);
    if (theme == "light") {
        nav.style.backgroundColor = color;
        nav.style.color = light;
    }
    else {
        nav.style.backgroundColor = dark;
        nav.style.color = color;
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