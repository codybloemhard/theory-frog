use web_dom::*;
use web_dom::document::get_element_by_id;
use web_dom::element::set_inner_html;
use music_theory::*;
use music_theory::theory::*;

#[no_mangle]
pub fn main() {
    // window::alert(window(),"hello world!");
    let doc = document();
    let button = document::query_selector(doc, "button");
    let listener = create_event_listener();
    eventtarget::add_event_listener(button, "click", listener);
}

#[no_mangle]
pub fn callback(_listener: EventListener, _event: Event){
    let doc = document();
    let input = document::query_selector(doc, "input");
    let msg = htmlinput::get_value(input);
    let output = notes_analysis(msg, ChordStyling::Std);
    let html = string_to_html_string(output);
    let answer_obj = get_element_by_id(doc, "answer");
    set_inner_html(answer_obj, &html);
}

pub fn string_to_html_string(string: String) -> String{
    let mut builder = "<p>".to_string();
    for c in string.chars(){
        if c == '\n' { builder.push_str("</p><p>"); }
        else { builder.push(c); }
    }
    builder
}
