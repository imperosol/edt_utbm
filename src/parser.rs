use std::fmt::{Display, Formatter};
use ego_tree::NodeRef;
use scraper::{Selector, Html, Node};

pub struct UE {
    pub code: String,
    pub course_type: String,
    pub day: String,
    pub start: String,
    pub end: String,
    pub room: String,
}

impl UE {
    pub fn from_row(row: NodeRef<Node>) -> Self {
        let td_list = row.children()
            .filter(|c| c.value().is_element() && c.first_child().is_some())
            .map(|col| col.first_child().unwrap().value().as_text().unwrap().trim())
            .collect::<Vec<&str>>();
        UE {
            code: td_list[0].to_string(),
            course_type: td_list[1].split(" ").next().unwrap().to_string(),
            day: td_list[2].to_string(),
            start: td_list[3].to_string(),
            end: td_list[4].to_string(),
            room: td_list[7].to_string(),
        }
    }
}

impl Display for UE {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{} : {} de {} à {} le {} en {}",
            self.code, self.course_type, self.start,
            self.end, self.day, self.room
        )
    }
}

pub fn get_ues(html: &str) -> Vec<UE> {
    let html = Html::parse_document(html);
    let selector = Selector::parse("#form").unwrap();
    let container = match html.select(&selector).next() {
        Some(container) => container,
        None => {
            eprintln!("Vous n'avez pas d'emploi du temps ce semestre.");
            std::process::exit(1);
        }
    };
    let selector = Selector::parse("tbody").unwrap();
    let form = container.select(&selector).next().unwrap();
    let tr_list = form.children().filter(|c| c.value().is_element());
    tr_list.map(|row| UE::from_row(row)).collect()
}
