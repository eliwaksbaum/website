let cur_open = null;
let holder = document.getElementById("holder");

window.addEventListener("resize", (e) => { sizeDrawer(); })

function toggleDrawer(drawer_id)
{
    let drawer = document.getElementById(drawer_id);

    if (drawer.style.maxHeight)
    {
        //drawer.style.display = "none"
        drawer.style.maxHeight = null;
        cur_open = null;
    }
    else
    {
        drawer.style.display = "block";
        if (cur_open != null)
        { 
            cur_open.style.display = "none";
            cur_open.style.maxHeight = null;
        }
        cur_open = drawer;
        sizeDrawer();
    }
}

function sizeDrawer()
{
    if (cur_open != null)
    {   
        let cur_card = cur_open.previousElementSibling;

        let holder_rect = holder.getBoundingClientRect();
        let card_rect = cur_card.getBoundingClientRect();

        let scale = .98 * (holder_rect.width / (card_rect.width));
        let left_hand_error = card_rect.left - holder_rect.left;
        let right_hand_error = (holder_rect.width - (card_rect.width * scale)) / 2;

        cur_open.style.width = `${scale * 100}%`;
        cur_open.style.right = `${left_hand_error - right_hand_error}px`;
        cur_open.style.maxHeight = cur_open.scrollHeight + "px";
    }
}