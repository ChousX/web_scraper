
fn main() {

    let response = reqwest::blocking::get(
        "https://github.com/ChousX?tab=repositories",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("h3.wb-break-all>a").unwrap();
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    titles
        .for_each(|item| println!("{}", item));

}
