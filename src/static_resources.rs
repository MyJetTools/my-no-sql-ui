pub const bootstrap_css: &[u8] = std::include_bytes!("../files/bootstrap.css");
pub const app_css: &[u8] = std::include_bytes!("../files/app.css");
pub const app_js: &[u8] = std::include_bytes!("../files/app.js");

pub fn get_header_content() -> String {
    let app_js_str = std::str::from_utf8(app_js).unwrap();
    let bootstrap_css_str = std::str::from_utf8(bootstrap_css).unwrap();
    let app_css_str = std::str::from_utf8(app_css).unwrap();

    format!(
        r#"
    <script>
        {app_js_str}
    </script>
    
    <style>
        {bootstrap_css_str}
        {app_css_str}
    </style>
    "#
    )
}
