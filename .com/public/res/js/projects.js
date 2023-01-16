
function openDrawer(id)
{
    var drawer = document.getElementById(id);
    if (drawer.style.display == "block")
    {
        drawer.style.display = "none";
    }
    else
    {
        drawer.style.display = "block";
    }
}