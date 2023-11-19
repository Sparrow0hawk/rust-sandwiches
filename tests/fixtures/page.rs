use scraper::{Html, Selector};

pub struct IndexPage {
    page_html: Html,
    page_table: PageTable,
}

#[allow(dead_code)]
impl IndexPage {
    pub fn new(page_html: &str) -> Self {
        IndexPage {
            page_html: Html::parse_document(&page_html),
            page_table: PageTable::new(&page_html),
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

    pub fn get_table_rows(self) -> Vec<String> {
        self.page_table._get_rows()
    }
}

#[allow(dead_code)]
pub struct PageTable {
    page_html: Html,
}

impl PageTable {
    pub fn new(page_html: &str) -> Self {
        PageTable {
            page_html: Html::parse_document(&page_html),
        }
    }

    pub fn _get_rows(&self) -> Vec<String> {
        let table_selector = Selector::parse("table").expect("Unable to create selector");
        let tbody_selector = Selector::parse("tbody").expect("Unable to create selector");
        let td_selector = Selector::parse("td").expect("Unable to create selector");

        let table = self.page_html.select(&table_selector).next().unwrap();
        let tbody = table.select(&tbody_selector).next().unwrap();

        tbody.select(&td_selector).map(|x| x.inner_html()).collect()
    }
}
