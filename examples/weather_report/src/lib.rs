extern crate chrono;
extern crate reqwest;

use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{Duration, UNIX_EPOCH};

use gloo::events::EventListener;
use json::JsonValue;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlInputElement;

#[wasm_bindgen(module = "/util.js")]
extern "C" {
    fn initialize(lat: f64, lon: f64);
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let search_div = create_div(&document, "search", "ReportStyles-secondDiv col-md-12");
    let input_box = create_input_box(&document);
    search_div.append_child(&input_box)?;
    let submit_box = create_submit_box(&document);
    let temp_div = create_div(
        &document,
        "tempDetail",
        "ReportStyles-mainContainer col-md-12 maincontainer",
    );
    let first_div = document.create_element("div")?;
    let second_div = create_div(&document, "second_div", "col-md-6");
    let third_div = create_div(&document, "third_div", "ReportStyles-innerDiv");
    let fourth_div = create_div(&document, "cityName", "ReportStyles-city");

    let table_div = document.create_element("table")?;
    table_div.set_class_name("ReportStyles-table table-bordered table-striped");

    let tbody_div = document.create_element("tbody")?;

    let ftr_div = document.create_element("tr")?;

    let ftd_div = document.create_element("td")?;
    ftd_div.set_class_name(" ReportStyles-firstTd");

    let img_div = document.create_element("div")?;
    img_div.set_id("temp");

    let std_div = document.create_element("td")?;
    std_div.set_class_name(" ReportStyles-secondTd");

    let weather_div = document.create_element("div")?;
    weather_div.set_id("weather");

    ftr_div.append_child(&ftd_div)?;
    ftd_div.append_child(&img_div)?;
    ftr_div.append_child(&std_div)?;
    std_div.append_child(&weather_div)?;

    let str_div = document.create_element("tr")?;
    let ptd_div = document.create_element("td")?;
    ptd_div.set_class_name(" ReportStyles-td");
    ptd_div.set_inner_html("Pressure");
    let sptd_div = document.create_element("td")?;
    sptd_div.set_id("pressure");
    str_div.append_child(&ptd_div)?;
    str_div.append_child(&sptd_div)?;

    let ttr_div = document.create_element("tr")?;
    let htd_div = document.create_element("td")?;
    htd_div.set_class_name(" ReportStyles-td");
    htd_div.set_inner_html("Humidity");
    let shtd_div = document.create_element("td")?;
    shtd_div.set_id("humidity");
    ttr_div.append_child(&htd_div)?;
    ttr_div.append_child(&shtd_div)?;

    let sunr_tr_div = document.create_element("tr")?;
    let sunr_td_div = document.create_element("td")?;
    sunr_td_div.set_class_name(" ReportStyles-td");
    sunr_td_div.set_inner_html("Sunrise[UTC]");
    let sunr_s_td_div = document.create_element("td")?;
    sunr_s_td_div.set_id("sunrise");
    sunr_tr_div.append_child(&sunr_td_div)?;
    sunr_tr_div.append_child(&sunr_s_td_div)?;

    let suns_tr_div = document.create_element("tr")?;
    let suns_td_div = document.create_element("td")?;
    suns_td_div.set_class_name(" ReportStyles-td");
    suns_td_div.set_inner_html("Sunset[UTC]");
    let suns_s_td_div = document.create_element("td")?;
    suns_s_td_div.set_id("sunset");
    suns_tr_div.append_child(&suns_td_div)?;
    suns_tr_div.append_child(&suns_s_td_div)?;

    let geo_tr_div = document.create_element("tr")?;
    let geo_htd_div = document.create_element("td")?;
    geo_htd_div.set_class_name(" ReportStyles-td");
    geo_htd_div.set_inner_html("Geo coords");
    let geo_shtd_div = document.create_element("td")?;
    geo_shtd_div.set_id("geocoords");
    geo_tr_div.append_child(&geo_htd_div)?;
    geo_tr_div.append_child(&geo_shtd_div)?;

    tbody_div.append_child(&ftr_div)?;
    tbody_div.append_child(&str_div)?;
    tbody_div.append_child(&ttr_div)?;
    tbody_div.append_child(&sunr_tr_div)?;
    tbody_div.append_child(&suns_tr_div)?;
    tbody_div.append_child(&geo_tr_div)?;

    table_div.append_child(&tbody_div)?;
    third_div.append_child(&fourth_div)?;
    third_div.append_child(&table_div)?;

    let map_div = document.create_element("div")?;
    map_div.set_class_name("col-md-6");
    let map_canvas_div = document.create_element("div")?;
    map_canvas_div.set_class_name(" ReportStyles-mapCanvas");
    map_canvas_div.set_id("map_canvas");
    map_div.append_child(&map_canvas_div)?;

    second_div.append_child(&third_div)?;
    first_div.append_child(&second_div)?;
    first_div.append_child(&map_div)?;

    temp_div.append_child(&first_div)?;
    search_div.append_child(&submit_box)?;
    body.append_child(&search_div)?;
    body.append_child(&temp_div)?;

    let on_click = EventListener::new(&submit_box, "click", move |_event| {
        let input_value = document
            .get_element_by_id("name")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value();
        let temp_d = temp_div.clone();
        let city = fourth_div.clone();
        let image = img_div.clone();
        let weather = weather_div.clone();
        let pressure = sptd_div.clone();
        let humidity = shtd_div.clone();
        let sunrise = sunr_s_td_div.clone();
        let sunset = suns_s_td_div.clone();
        let geo = geo_shtd_div.clone();
        let input_value: &'static _ = Box::leak(Box::new(input_value));
        let response = get_response(input_value);
        spawn_local(async move {
            let parsed = response.await;
            let lon = parsed["coord"]["lon"].to_owned().as_f64().unwrap();
            let lat = parsed["coord"]["lat"].to_owned().as_f64().unwrap();
            initialize(lat, lon);
            let city_name: &str = &parsed["name"].to_owned().to_string();
            let country_name: &str = &parsed["sys"]["country"].to_owned().to_string();
            let place = [city_name, ",", country_name].concat();
            let icon = &parsed["weather"][0]["icon"].to_owned().to_string();
            let src = [
                "<img src='http://openweathermap.org/img/w/",
                icon,
                ".png'>",
                "  ",
            ]
            .concat();
            let temp = (parsed["main"]["temp"].to_owned().as_f64().unwrap() - 273.15) as i64;

            let content = [src, temp.to_string()].concat();
            let p: &str = &parsed["main"]["pressure"].to_owned().to_string();
            let h: &str = &parsed["main"]["humidity"].to_owned().to_string();
            let sun_r = (parsed["sys"]["sunrise"].to_owned().as_f64().unwrap()) as u64;
            let sun_s = (parsed["sys"]["sunset"].to_owned().as_f64().unwrap()) as u64;

            temp_d
                .set_attribute("style", "display: block")
                .expect("failed to set attr style");
            city.set_inner_html(&place);
            image.set_inner_html(&content);
            weather.set_inner_html(&parsed["weather"][0]["main"].to_owned().to_string());
            pressure.set_inner_html(&([p, " hpa"].concat()));
            humidity.set_inner_html(&([h, "%"].concat()));
            sunrise.set_inner_html(&get_time(sun_r));
            sunset.set_inner_html(&get_time(sun_s));
            geo.set_inner_html(&(["[", &lon.to_string(), ",", &lat.to_string(), "]"].concat()));
        });
    });

    // When a Closure is dropped it will invalidate the associated JS closure.
    // Here we want JS callback to be alive for the entire duration of the program.
    // So we used `forget` leak this instance of Closure.
    // It should be used sparingly to ensure the memory leak doesn't affect the program too much.
    on_click.forget();
    Ok(())
}

fn create_div(document: &Document, id: &str, class: &str) -> Element {
    let div = document.create_element("div").unwrap();
    div.set_id(id);
    div.set_class_name(class);
    div
}

fn create_submit_box(document: &Document) -> Element {
    let submit_box: Element = document.create_element("input").unwrap();
    submit_box
        .set_attribute("type", "button")
        .expect("failed to set attr type to button");
    submit_box
        .set_attribute("value", "Search")
        .expect("failed to set attr value to Search");
    submit_box
        .set_attribute("name", "submit")
        .expect("failed to set attr name to submit");
    submit_box.set_id("submit");
    submit_box.set_class_name(" ReportStyles-bootstrapButton btn btn-info");
    submit_box
}

fn create_input_box(document: &Document) -> Element {
    let input_box = document.create_element("input").unwrap();
    input_box
        .set_attribute("name", "name")
        .expect("failed to set attr name to name");
    input_box.set_attribute("value", "Delhi").expect(
        "
    failed to set attr value to Delhi",
    );
    input_box
        .set_attribute("type", "text")
        .expect("failed to set attr type to text");
    input_box
        .set_attribute("placeholder", "Type city name here")
        .expect("Failed to set attr placeholder to Type city name here");
    input_box.set_id("name");
    input_box.set_class_name("ReportStyles-search");
    input_box
}

// Get response from weather api
async fn get_response(location: &str) -> JsonValue {
    let url1 = "http://api.openweathermap.org/data/2.5/weather?q=";
    let url2 = "&appid=<apiKey>";

    let url = [url1, location, url2].concat();

    let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();

    json::parse(&resp).unwrap()
}

// Convert millisecond into UTC date
fn get_time(millis: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(millis);
    // Create DateTime from SystemTime
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    datetime.format("%H:%M:%S").to_string()
}
