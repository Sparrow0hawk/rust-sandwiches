use scraper::{Html, Selector};

pub struct IndexPage {
    page_html: Html,
}

impl IndexPage {
    pub fn new(page_html: &str) -> Self {
        IndexPage {
            page_html: Html::parse_document(&page_html),
        }
    }

    pub fn get_header(self) -> String {
        let header_selector = Selector::parse("h1").expect("Unable to create Selector");

        self.page_html
            .select(&header_selector)
            .next()
            .unwrap()
            .inner_html()
    }
}
