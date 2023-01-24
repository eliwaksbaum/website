var colors = ["rgb(146, 101, 243)", "rgb(236, 79, 19)", "rgb(77, 196, 295)", "rgb(19, 236, 205)", "rgb(255, 77, 255)", "rgb(255, 136, 77)", "rgb(0, 213, 0)"]

function toggleTheme() {
    toggleCss();
    toggleThemedImages();
}

function toggleThemedImages() {
    var imgs = document.getElementsByClassName("themed");
    for (var i = 0; i < imgs.length; i++) {
        imgs[i].src = imgs[i].src.split("?a=")[0].replace("cur", "toggle") + "?a=" + performance.now();
    }
}

function toggleCss() {
    let theme = document.getElementById("theme");
    theme.href = theme.href.split("?a=")[0].replace("cur", "toggle") + "?a=" + performance.now();
}

function colorize() {
    var headers = document.getElementsByClassName("colorful");
    var i = Math.floor((Math.random() * colors.length));
    var color = colors[i];
    
    for (var i = 0; i < headers.length; i++) {
        headers[i].style.backgroundColor = color;
    }
}