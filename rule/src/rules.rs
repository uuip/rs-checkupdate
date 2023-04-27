use once_cell::sync::Lazy;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use semver::Version;

static VER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[.\d]*\d").unwrap());

pub fn num_version(ver_info: String) -> Option<String> {
    VER_RE
        .find(ver_info.as_str())
        .map(|x| x.as_str().to_string())
}
/*
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
pub(crate) fn parse_navicat(resp: &str) -> Option<String> {
    let html = Document::from(resp);
    let element = html
        .find(
            Class("release-notes-table")
                .and(Attr("platform", "W"))
                .descendant(Name("td"))
                .descendant(Class("note-title")),
        )
        .next()?
        .text();
    Some(element)
}
*/
pub(crate) fn parse_beyond_compare(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("p").unwrap();
    let re = Regex::new("Current Version.+").unwrap();

    let element = html
        .select(&selector)
        .find_map(|x| re.find(x.text().next().unwrap_or_default()))?;
    Some(element.as_str().to_owned())
}
pub(crate) fn parse_faststone(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("b").unwrap();
    let re = Regex::new(r"Version\s*[.\d]+").unwrap();

    let element = html
        .select(&selector)
        .find_map(|x| re.find(x.text().next().unwrap_or_default()))?;
    Some(element.as_str().to_owned())
}
pub(crate) fn parse_winrar(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("b").unwrap();
    let re = Regex::new("^WinRAR.*elease").unwrap();

    let element = html
        .select(&selector)
        .find_map(|x| re.find(x.text().next().unwrap_or_default()))?;
    Some(element.as_str().to_owned())
}
pub(crate) fn parse_vmware(resp: &str) -> Option<String> {
    let html = Html::parse_fragment(resp);
    let selector = Selector::parse("metadata>version").unwrap();
    let mut element: Vec<Version> = html
        .select(&selector)
        .filter_map(|x| Version::parse(x.text().next().unwrap_or("0.0.0")).ok())
        .collect();
    element.sort();
    let ver = element.last()?;
    Some(ver.to_string())
}
pub(crate) fn parse_dev_man_view(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("h4").unwrap();
    let element = html
        .select(&selector)
        .find(|x| x.text().next().unwrap_or_default() == "Versions History")?;
    let element = element.next_siblings().nth(1)?.children().nth(1)?;
    let element = ElementRef::wrap(element)?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_aida64(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("td.version").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_git(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse(".version").unwrap();
    let element = html.select(&selector).next()?.text().next()?.trim();
    Some(element.to_owned())
}
pub(crate) fn parse_wgestures2_mac(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("a#download:nth-of-type(2)").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_wgestures2(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("a#download:nth-of-type(1)").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_contexts_mac(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse(".section--history__item__header>h1").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_python(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("p.download-buttons>a").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_everything(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("h2").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_navicat_mac(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse(r#".release-notes-table[platform="M"] td>.note-title"#).unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_navicat(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse(r#".release-notes-table[platform="W"] td>.note-title"#).unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_firefox(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse(".c-release-version").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_registry_workshop(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("p").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_secure_crt(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse("#download-tabs>h4").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_pdf_xchange(resp: &str) -> Option<String> {
    let html = Html::parse_document(resp);
    let selector = Selector::parse(r"#bh-history>li:first-of-type>a").unwrap();
    let element = html.select(&selector).next()?.text().next()?;
    Some(element.to_owned())
}
pub(crate) fn parse_emeditor(resp: &str) -> Option<String> {
    Some(resp.split('_').last()?.to_owned())
}
