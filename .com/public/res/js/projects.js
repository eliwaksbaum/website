let cur_open = null;
let holder = document.getElementById("holder");

window.addEventListener("resize", (e) => { sizeDrawer(); })

function toggleDrawer(card, drawer_id)
{
    let drawer = document.getElementById(drawer_id);
    if (drawer.style.display == "block")
    {
        drawer.style.display = "none";
        cur_open = null;
    }
    else
    {
        drawer.style.display = "block";
        if (cur_open != null)
        { 
            cur_open.style.display = "none";
        }
        cur_open = drawer;
        sizeDrawer();
    }
}

function sizeDrawer()
{
    if (cur_open != null)
    {   
        let cur_listing = cur_open.parentElement;

        let holder_rect = holder.getBoundingClientRect();
        let listing_rect = cur_listing.getBoundingClientRect();
        let card_rect = cur_open.previousElementSibling.getBoundingClientRect();

        let scale = (holder_rect.width / (listing_rect.width));
        cur_open.style.width = `${scale * 100}%`;
        cur_open.style.right = `${.98 * (listing_rect.left - holder_rect.left)}px`;
        //cur_open.style.left = `${.98 * (holder_rect.right - (listing_rect.left + (card_rect.width * scale)))}px`;
        console.log(holder_rect.right,  listing_rect.left + (card_rect.width * scale));
    }
}