var css;
var desktop_button;
var mobile_button;


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

function setButton(theme) {
    var anti;
    if (theme == "light") {
        anti = "dark";
    }
    else {
        anti = "light";
    }
    desktop_button.src = "/res/svg/" + anti + ".svg";
    desktop_button.alt = anti;
    mobile_button.src = "/res/svg/" + anti + ".svg";
    mobile_button.alt = anti;
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
    desktop_button = document.getElementById("theme-button-desktop");
    mobile_button = document.getElementById("theme-button-mobile");
    setButton(theme_preference);
}

function colorize() {
    var headers = document.getElementsByClassName("colorful");
    var i = Math.floor((Math.random() * colors.length));
    var color = colors[i];
    
    for (var i = 0; i < headers.length; i++) {
        headers[i].style.backgroundColor = color;
    }
}

function loadThemedImages(theme) {
    var imgs = document.getElementsByClassName("themed");
    for (var i = 0; i < imgs.length; i++) {
        var themedSrc = imgs[i].getAttribute("data-themedSrc");
        var src = themedSrc.replace("theme", theme);
        imgs[i].src = src;
    }
}