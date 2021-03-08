function load() {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
        document.getElementById("demo").innerHTML = this.responseText + ", yo";
      }
    };
    xhttp.open("GET", "/dynamic?param=value", true);
    xhttp.send();
}