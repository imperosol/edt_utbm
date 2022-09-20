use reqwest::blocking;
use scraper::{Html, Selector};


pub fn init_client() -> blocking::Client {
    blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36")
        .cookie_store(true)
        .build().unwrap()
}

pub fn login(client: &blocking::Client, username: impl AsRef<str>, password: impl AsRef<str>) {
    let url = "https://cas.utbm.fr/login?service=https://monespace.utbm.fr/Login";
    let res = client.get(url).send().unwrap();
    let html = Html::parse_document(res.text().unwrap().as_str());
    let selector_exec = Selector::parse("input[name='execution']").unwrap();
    let execution = html.select(&selector_exec)
        .next().unwrap()
        .value().attr("value").unwrap();
    client.post("https://cas.utbm.fr/login")
        .form(&[
            ("username", username.as_ref()),
            ("password", password.as_ref()),
            ("_eventId", "submit"),
            ("execution", execution)
        ]).send().unwrap();
}

pub fn get_timetable_page(client: &blocking::Client) -> Result<blocking::Response, ()> {
    let url = "https://extranet1.utbm.fr/gpedago/dossier/edt.xhtml?noIndividu=256642";
    let res = client.get(url).send().unwrap();
    // Le serveur fait une redirection lorsqu'une erreur arrive
    if res.url().as_str() == url {
        Ok(res)
    } else {
        Err(())
    }
}
