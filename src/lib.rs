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
    let input = document::query_selector(doc, "input");
    let listener = create_event_listener();
    eventtarget::add_event_listener(button, "click", listener);
    eventtarget::add_event_listener(input, "keyup", listener);
    // Set up help with and example when the site starts
    let answer_obj = get_element_by_id(doc, "answer");
    let help_msg = get_help_html_object();
    set_inner_html(answer_obj, &help_msg);
}

#[no_mangle]
pub fn callback(_listener: EventListener, _event: Event){
    let doc = document();
    let input = document::query_selector(doc, "input");
    let answer_obj = get_element_by_id(doc, "answer");
    let msg = htmlinput::get_value(input);
    if &msg == "help" || &msg == "Help" {
        set_inner_html(answer_obj, &get_help_html_object());
        return;
    }
    let output = notes_analysis(msg, ChordStyling::Std);
    let html = if output.is_empty() {
        let mut res = "<div class=\"piece\"><h2>Error</h2><p>Could not parse input! Try again!</p></div>".to_string();
        res.push_str(&get_help_html_object());
        res
    } else {
        structure_to_html_string(output)
    };
    set_inner_html(answer_obj, &html);
}

pub fn structure_to_html_string(structure: Vec<(String, String)>) -> String{
    let mut builder = String::new();
    for (header, content) in structure{
        if content.is_empty() { continue; }
        builder.push_str("<div class=\"piece\">");
        builder.push_str(&format!("<h2>{}</h2>", header));
        string_into_html_string(&mut builder, content);
        builder.push_str("</div>")
    }
    builder
}

pub fn string_into_html_string(builder: &mut String, string: String){
    builder.push_str("<p>");
    for c in string.chars(){
        if c == '\n' { builder.push_str("</p><p>"); }
        else { builder.push(c); }
    }
}

pub fn get_help_html_object() -> String{
    let mut msg = "<div class=\"piece\">
        <h2>Help</h2>
        <p>You can access this help page by typing \"help\" in the bar and press \"Ask!\".</p>
        <h2>Input</h2>
        <p>Type in a list of comma separated notes. Is not case sensitive. For sharp and flats, use #, b, ♯ and ♭. Examples: \"f,a,c,e\", \"a,b,c,d,e,f,g\", \"bb,db,fb\".</p>
        <h2>Output</h2>
        <p>It will generate(if named) inversions(rotation of a chord), subchords(subsequence of a chord), chordtone wholetone scale(a way to build a scale from a tetrad), strict chordscales(where the chordtones are on uneven scale degrees), supersequences(scales such that the notes appear in it in order and uninterrupted) and supersets(scales such that the notes are all present in it).</p>
        <h2>About</h2>
        <p>Theøry Frøg is a simple front-end web app, without backend. It's made with simple Html, Css, Js, and WASM! A Rust library that I build does the actual calculations, which is imported into this theory-frog rust crate. The theory-frog rust lib is than compiled to WASM and used. Theøry Frøg is made by Cody Bloemhard.</p>
        <a href=\"https://github.com/codybloemhard/theory-frog\" target=\"_blank\">github repository</a>
        <h2>Example</h2>
        <p>Below is an example output of the input \"f,a,c,e\"</p>
    ".to_string();
    let output = notes_analysis("f,a,c,e".to_string(), ChordStyling::Std);
    msg.push_str(&structure_to_html_string(output));
    msg.push_str("</div>");
    msg
}
