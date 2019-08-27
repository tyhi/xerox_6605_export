use csv::Writer;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::io;

fn main() {
    
    println!("Input IP Address:");

    let mut ip = String::new();
    io::stdin()
        .read_line(&mut ip)
        .expect("read_line failed to read the line...");
    ip.truncate(ip.len() - 2);

    let mut addr: HashMap<String, String> = HashMap::new();

    for i in 1..6 {
        let ee = get_page_body(&ip, i);
        save_addr(&mut addr, ee)
    }

    let mut wtr = Writer::from_path("./addr.csv").expect("error opening/creating csv file.");
    for (k, v) in addr.iter() {
        wtr.write_record(&[k, v])
            .expect("issue with writing record");
    }
    wtr.flush().expect("error flushing to file");
    println!("{:?}", addr)
}

fn get_page_body(ip: &String, index: i8) -> String {
    let mut resp = chttp::get(format!("http://{}/srvcset/emlusrlst0000{}.htm", ip, index))
        .expect("error getting srvcset form for address book");

    resp.body_mut()
        .text()
        .expect("error getting body from response")
}

fn save_addr(hash: &mut HashMap<String, String>, body: String) {
    let frag = Html::parse_document(&body);
    for i in 3..23 {
        let name = match frag
            .select(
                &Selector::parse(&format!(
                    "tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(2) > a:nth-child(1)",
                    i
                ))
                .expect("error finding selector"),
            )
            .next()
        {
            None => continue,
            Some(e) => e.inner_html(),
        };

        let email = frag
            .select(
                &Selector::parse(&format!(
                    "tbody:nth-child(1) > tr:nth-child({}) > td:nth-child(3)",
                    i
                ))
                .expect("error finding selector"),
            )
            .next()
            .expect("error finding next selector")
            .inner_html();

        hash.insert(name, email);
    }
}
