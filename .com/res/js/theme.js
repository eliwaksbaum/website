var css;

var colors = ["rgb(146, 101, 243)", "rgb(236, 79, 19)", "rgb(77, 196, 295)", "rgb(19, 236, 205)", "rgb(255, 77, 255)", "rgb(255, 136, 77)", "rgb(0, 213, 0)"]

var theme_preference = localStorage.getItem("theme");
if (theme_preference == null) {
    theme_preference = "light";
}

var root = document.documentElement;
var style = getComputedStyle(root);
var dark = style.getPropertyValue("--dark");
var light = style.getPropertyValue("--light");

function setCss(theme) {
    css.href = "/res/stylesheets/" + theme + ".css";
}

function setButton(button, theme) {
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
    var button = document.getElementById("theme-button-desktop");
    setButton(button, theme_preference);
    button.addEventListener("click", toggleTheme);

    // button = document.getElementById("theme-button-mobile");
    // setButton(button, theme_preference);
    // button.addEventListener("click", toggleTheme);
}

function colorHeader() {
    var headers = document.getElementsByTagName("header");
    var i = Math.floor((Math.random() * colors.length));
    var color = colors[i];
    
    for (var header of headers) {
        header.style.backgroundColor = color;
    }
}

function loadThemedImages(theme) {
    var imgs = document.getElementsByClassName("themed");
    for (var img of imgs) {
        var themedSrc = img.getAttribute("data-themedSrc");
        var src = themedSrc.replace("theme", theme);
        img.src = src;
    }
}