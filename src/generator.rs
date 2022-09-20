use std::fs::File;
use std::io::{Write};
use crate::parser::UE;

const CELL_HEIGHT: i32 = 11;

fn get_slot_num(time: &str) -> i32 {
    let mut time = time.split(":").map(|s| s.parse::<i32>().unwrap());
    let h = time.next().unwrap();
    let m = time.next().unwrap();
    (h - 8) * 4 + (m / 15)
}


pub fn get_html(ues: Vec<UE>) {
    let mut html = String::from("<!doctype html><html lang=\"fr\"><head>\
        <meta charset=\"UTF-8\"><meta name=\"viewport\"\
        content=\"width=device-width,user-scalable=no,initial-scale=1.0,\
        maximum-scale=1.0,minimum-scale=1.0\">\
        <title>Emploi du temps</title></head>\
        <style>body{font-family:Verdana,Geneva,Arial,Helvetica,sans-serif;\
        font-size:9px;max-width:700px;display:flex}\
        .day{flex:2}.bg-blue-odd{background-color:#E9EDF2}.cell{display:flex;justify-content: center;\
        align-items:center;flex-direction: column;height:11px;}\
        .bg-blue-even{background-color:#8b8eff}\
        p {margin: 1px;}.busy{background-color: #BBCCFF;\
        border: black 1px solid;box-sizing: border-box}\
        .empty {height: 10px;border-bottom: dotted 1px #C1D2EE;}</style>\
        <body><div style=\"flex:1\"><div class=\"cell\"></div>"
    );
    let mut h = 8;
    let mut m = 0;
    while h < 21 {
        // if m == 0 || m == 30 {
        if h % 2 == 0 {
            html.push_str(format!("<div class=\"cell bg-blue-odd empty\">{:0>2}:{:0<2}</div>", h, m).as_str());
        } else {
            html.push_str(format!("<div class=\"cell bg-blue-even empty\">{:0>2}:{:0<2}</div>", h, m).as_str());
        }
        m += 15;
        if m == 60 {
            m = 0;
            h += 1;
        }
    }
    let mut current_day = ues[0].day.as_str();
    let mut curr_slot = 0;
    html.push_str("</div><div class=\"day\"><div class=\"cell bg-blue-odd\"><b>");
    html.push_str(current_day);
    html.push_str("</b></div>");
    for ue in ues.iter() {
        if ue.day.as_str() != current_day {
            for _ in curr_slot..52 {
                html.push_str("<div class=\"cell empty\"></div>")
            }
            current_day = ue.day.as_str();
            curr_slot = 0;
            html.push_str("</div><div class=\"day\"><div class=\"cell bg-blue-odd\"><b>");
            html.push_str(current_day);
            html.push_str("</b></div>");
        }
        let slot_start = get_slot_num(ue.start.as_str());
        let slot_end = get_slot_num(ue.end.as_str());
        let duration = slot_end - slot_start;
        for _ in curr_slot..slot_start {
            html.push_str("<div class=\"cell empty\"></div>")
        }
        html.push_str(
            format!(
                "<div class=\"cell busy\" style=\"height:{}px\"><p>{}</p><p>{}</p><p>{}</p></div>",
                duration * CELL_HEIGHT, ue.code, ue.course_type, ue.room
            ).as_str()
        );
        curr_slot = slot_end;
    }
    for _ in curr_slot..52 {
        html.push_str("<div class=\"cell empty\"></div>")
    }
    html.push_str("</div></body></html>");
    let mut file = File::create("edt.html").unwrap();
    file.write(html.as_ref()).unwrap();
}