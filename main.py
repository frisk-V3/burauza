import sys
from PySide6.QtWidgets import (
    QApplication, QMainWindow, QToolBar, QLineEdit,
    QPushButton, QTabWidget, QWidget, QVBoxLayout
)
from PySide6.QtWebEngineWidgets import QWebEngineView
from PySide6.QtCore import QUrl


# 🔍 検索エンジン設定
SEARCH_ENGINES = {
    "google": "https://www.google.com/search?q={}",
    "bing": "https://www.bing.com/search?q={}",
    "yahoo": "https://search.yahoo.com/search?p={}",
    "duckduckgo": "https://duckduckgo.com/?q={}",
    "wikipedia": "https://ja.wikipedia.org/wiki/{}",
    "github": "https://github.com/search?q={}"
}


class BrowserTab(QWidget):
    def __init__(self):
        super().__init__()
        self.layout = QVBoxLayout()
        self.browser = QWebEngineView()
        self.layout.addWidget(self.browser)
        self.setLayout(self.layout)

    def load_url(self, url):
        self.browser.setUrl(QUrl(url))


class Browser(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Frisk Browser")
        self.resize(1200, 800)

        # タブ
        self.tabs = QTabWidget()
        self.tabs.setTabsClosable(True)
        self.tabs.tabCloseRequested.connect(self.close_tab)
        self.setCentralWidget(self.tabs)

        # ツールバー
        toolbar = QToolBar()
        self.addToolBar(toolbar)

        # 戻るボタン
        back_btn = QPushButton("←")
        back_btn.clicked.connect(self.go_back)
        toolbar.addWidget(back_btn)

        # 進むボタン
        forward_btn = QPushButton("→")
        forward_btn.clicked.connect(self.go_forward)
        toolbar.addWidget(forward_btn)

        # リロード
        reload_btn = QPushButton("⟳")
        reload_btn.clicked.connect(self.reload_page)
        toolbar.addWidget(reload_btn)

        # URLバー
        self.url_bar = QLineEdit()
        self.url_bar.returnPressed.connect(self.navigate)
        toolbar.addWidget(self.url_bar)

        # 新規タブ
        new_tab_btn = QPushButton("+")
        new_tab_btn.clicked.connect(self.add_tab)
        toolbar.addWidget(new_tab_btn)

        self.history = []

        self.add_tab("https://www.google.com")

    def add_tab(self, url=None):
        tab = BrowserTab()
        index = self.tabs.addTab(tab, "New Tab")
        self.tabs.setCurrentIndex(index)

        if url:
            tab.load_url(url)

        tab.browser.urlChanged.connect(
            lambda q, tab=tab: self.update_url(q, tab)
        )
        tab.browser.loadFinished.connect(
            lambda _, tab=tab: self.update_title(tab)
        )

    def close_tab(self, index):
        if self.tabs.count() > 1:
            self.tabs.removeTab(index)

    def current_browser(self):
        return self.tabs.currentWidget().browser

    def navigate(self):
        text = self.url_bar.text().strip()

        # URL判定
        if "." in text and " " not in text:
            url = text if text.startswith("http") else "http://" + text
        else:
            # デフォルト検索（Google）
            url = SEARCH_ENGINES["google"].format(text)

        self.current_browser().setUrl(QUrl(url))
        self.history.append(url)

    def update_url(self, q, tab):
        if tab == self.tabs.currentWidget():
            self.url_bar.setText(q.toString())

    def update_title(self, tab):
        index = self.tabs.indexOf(tab)
        self.tabs.setTabText(index, tab.browser.page().title())

    def go_back(self):
        self.current_browser().back()

    def go_forward(self):
        self.current_browser().forward()

    def reload_page(self):
        self.current_browser().reload()


if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = Browser()
    window.show()
    sys.exit(app.exec())
