function load() {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
        document.getElementById("demo").innerHTML = this.responseText + ", yo";
      }
    };

    var url;
    var query = window.location.search;
    if (query.slice(0,5) == "?tag=") {
      var tag = query.split("?tag=")[1];
      url = "/dynamic/tag/" + tag;
    }
    else {
      url = "/dynamic/all-projects/";
    }
    
    xhttp.open("GET", url, true);
    xhttp.send();
}