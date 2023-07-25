function resize() {
    let el = document.getElementById("main");
    let winHeight = window.innerHeight;
    let winWidth = window.innerWidth;
    el.style.setProperty('--main-height', winHeight + "px");
    el.style.setProperty('--main-width', winWidth + "px");

    let topPanel = document.getElementById("top-panel");
    let myTopPanelHeight = topPanel.clientHeight;
    el.style.setProperty('--working-area-height', (winHeight - topPanel.clientHeight) + "px");
    el.style.setProperty('--top-panel-height', topPanel.clientHeight + "px");

    let leftPanel = document.getElementById("left-panel");
    el.style.setProperty('--right-panel-width', (winWidth - leftPanel.clientWidth - 10) + "px");
}

addEventListener("resize", resize);
setTimeout(resize, 100);