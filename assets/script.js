const engines = {
    google: q => `https://www.google.com/search?q=${q}`,
    bing: q => `https://www.bing.com/search?q=${q}`,
    yahoo: q => `https://search.yahoo.co.jp/search?p=${q}`,
    duck: q => `https://duckduckgo.com/?q=${q}`,
    wiki: q => `https://ja.wikipedia.org/wiki/${q}`,
    github: q => `https://github.com/search?q=${q}`
};

function search() {
    const engine = document.getElementById("engine").value;
    const q = document.getElementById("query").value;

    const url = engines[engine](encodeURIComponent(q));
    window.location.href = url;

    saveHistory(q, url);
}

function saveHistory(q, url) {
    let h = JSON.parse(localStorage.getItem("history") || "[]");
    h.unshift({ q, url });
    localStorage.setItem("history", JSON.stringify(h));
    renderHistory();
}

function renderHistory() {
    let h = JSON.parse(localStorage.getItem("history") || "[]");
    let html = "<h3>履歴</h3>";

    h.slice(0, 10).forEach(item => {
        html += `<div><a href="${item.url}">${item.q}</a></div>`;
    });

    document.getElementById("history").innerHTML = html;
}

renderHistory();
