
var number_exp = 3;


function drawConnections() {
    let count = 1;
    const canvas_el = document.getElementById("p-e-canvas");
    const content_el = document.getElementById("p-e-content");
    const content_box = content_el.getBoundingClientRect();
    canvas_el.setAttribute("width", content_box.width);
    canvas_el.setAttribute("height", content_box.height);
    canvas_el.setAttribute("top", content_box.top);
    canvas_el.setAttribute("left", content_box.left);
    const ctx = canvas_el.getContext("2d");
    ctx.strokeStyle = "#E4FDE1";
    ctx.fillStyle = "#E4FDE1";
    for (let count = 1; count <= (number_exp-1); count++) {
        let cur_id_str = `p-e-${count}`;
        let nex_id_str = `p-e-${count+1}`;
        let cur_ele = document.getElementById(cur_id_str);
        let next_ele = document.getElementById(nex_id_str);
        let cur_box = cur_ele.getBoundingClientRect();
        let next_box = next_ele.getBoundingClientRect();
        let center_x = (cur_box.left + cur_box.right) / 2;
        let center_y = (cur_box.top + cur_box.bottom) / 2;
        let next_x = (next_box.left + next_box.right) / 2;
        let next_y = (next_box.top + next_box.bottom) / 2;

        ctx.beginPath();
        ctx.moveTo(center_x, center_y);
        ctx.lineTo(next_x, next_y);
        ctx.stroke();

        ctx.beginPath();
        ctx.moveTo(center_x, cur_box.bottom);
        ctx.arc(center_x, cur_box.bottom, 5, 0, Math.PI);
        ctx.fill();

        ctx.beginPath();
        ctx.moveTo(center_x, next_box.top);
        ctx.arc(center_x, next_box.top, 5, 0, Math.PI, true);
        ctx.fill();
    }
}

function moveBevy() {
    const canvas_el = document.getElementById("bevy-canvas1");
    const content_el = document.getElementById("hero");
    const content_box = content_el.getBoundingClientRect();
    canvas_el.setAttribute("width", content_box.width);
    canvas_el.setAttribute("height", content_box.height);
    canvas_el.setAttribute("top", content_box.top);
    canvas_el.setAttribute("left", content_box.left);
}

window.addEventListener("scroll", (event) => {
    drawConnections();
    moveBevy();
});