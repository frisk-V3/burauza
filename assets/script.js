let tabs = [];
let current = 0;

function newTab(url = "https://www.google.com") {
    tabs.push({ url });
    current = tabs.length - 1;
    renderTabs();
    load();
}

function renderTabs() {
    const el = document.getElementById("tabs");
    el.innerHTML = "";

    tabs.forEach((t, i) => {
        const div = document.createElement("div");
        div.className = "tab" + (i === current ? " active" : "");

        const icon = document.createElement("img");
        icon.src = `https://www.google.com/s2/favicons?domain=${t.url}`;

        const title = document.createElement("span");
        title.innerText = "Tab " + (i + 1);

        const close = document.createElement("span");
        close.innerText = "×";
        close.onclick = (e) => {
            e.stopPropagation();
            tabs.splice(i, 1);
            current = Math.max(0, current - 1);
            renderTabs();
            load();
        };

        div.onclick = () => {
            current = i;
            renderTabs();
            load();
        };

        div.append(icon, title, close);
        el.appendChild(div);
    });
}

function load() {
    const tab = tabs[current];
    document.getElementById("view").src = tab.url;
    document.getElementById("url").value = tab.url;
}

function go() {
    let input = document.getElementById("url").value;

    if (!input.startsWith("http")) {
        input = "https://www.google.com/search?q=" + encodeURIComponent(input);
    }

    tabs[current].url = input;
    load();

    window.ipc.postMessage(input);
}

function back() {
    document.getElementById("view").contentWindow.history.back();
}

function forward() {
    document.getElementById("view").contentWindow.history.forward();
}

newTab();
