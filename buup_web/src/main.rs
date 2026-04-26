use dioxus::document;
use dioxus::prelude::*;
use std::rc::Rc;

#[cfg(feature = "web")]
use wasm_bindgen::JsCast;

mod styles; // Add module declaration

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const FAVICON_32: Asset = asset!("assets/favicon-32x32.png");
const FAVICON_16: Asset = asset!("assets/favicon-16x16.png");
const APPLE_TOUCH_ICON: Asset = asset!("assets/apple-touch-icon.png");
const BUUP_ICON_SVG: Asset = asset!("assets/buup-icon.svg");
const SITE_MANIFEST: Asset = asset!("assets/site.webmanifest");

fn main() {
    dioxus::launch(App);
}

// Function to provide custom meta descriptions for popular tools
fn get_meta_description(
    transformer_id: &str,
    transformer_name: &str,
    default_description: &str,
) -> String {
    match transformer_id {
        "base64encode" => "Encode text to Base64 online. Free, secure Base64 encoder works entirely in your browser. No data sent to servers. Ideal for strings, JSON, and binary data.".to_string(),
        "base64decode" => "Decode Base64 strings to text instantly. Free, secure Base64 decoder operates offline in your browser for maximum privacy. Supports standard and URL-safe formats.".to_string(),
        "gzipcompress" => "Compress text using GZIP algorithm online. Reduce data size with this free, client-side compression tool. Secure and private – no server uploads required.".to_string(),
        "gzipdecompress" => "Decompress GZIP-encoded data online. Free tool securely extracts compressed text directly in your browser. Works offline, ensuring privacy.".to_string(),
        "jsonformat" => "Format and prettify JSON data online. Free tool improves readability and validates JSON structure. Works completely offline for security and privacy.".to_string(),
        "jsonminify" => "Minify JSON data online to reduce file size. Free tool removes whitespace while preserving structure. Operates 100% offline in your browser.".to_string(),
        "urlencode" => "URL-encode strings online. Free tool safely encodes special characters and Unicode for URLs and query parameters. Works entirely in your browser, no server interaction.".to_string(),
        "urldecode" => "URL-decode strings online. Free tool converts percent-encoded characters back to readable text. Secure, private, and works offline in your browser.".to_string(),
        "htmlencode" => "Encode HTML special characters to entities online. Prevent XSS attacks with this free, client-side HTML encoder. Securely processes data in your browser.".to_string(),
        "htmldecode" => "Decode HTML entities back to characters online. Free, secure HTML decoder works entirely in your browser. No data is sent to any server.".to_string(),
        "md5hash" => "Generate MD5 hash online from text. Free MD5 hashing tool works completely offline in your browser for data privacy and security.".to_string(),
        "sha256hash" => "Generate SHA-256 hash online from text. Free cryptographic hashing tool works entirely in your browser. Secure, private, no server interaction.".to_string(),
        "hexencode" => "Encode text to Hexadecimal online. Free Hex encoder works entirely in your browser, ensuring data privacy. Convert strings to hex format securely.".to_string(),
        "hexdecode" => "Decode Hexadecimal strings to text online. Free Hex decoder operates offline in your browser for maximum security. Convert hex back to readable text.".to_string(),
        "binaryencode" => "Encode text to Binary (base-2) online. Free binary encoder converts text to space-separated byte values. Works offline in your browser.".to_string(),
        "binarydecode" => "Decode Binary strings (space-separated bytes) to text online. Free binary decoder works securely in your browser, offline.".to_string(),
        "morseencode" => "Encode text to Morse code online. Free Morse code translator works entirely in your browser for privacy. Convert letters and numbers easily.".to_string(),
        "morsedecode" => "Decode Morse code to text online. Free Morse code translator operates offline in your browser. Convert dots and dashes back to readable text.".to_string(),
        "jwtdecode" => "Decode JSON Web Tokens (JWT) online without verification. Free JWT decoder tool inspects header and payload client-side. Works securely offline.".to_string(),
        "uuid_generate" => "Generate Version 4 UUIDs (Universally Unique Identifiers) online. Free UUID generator creates random v4 UUIDs directly in your browser.".to_string(), // Note: PRNG quality mentioned in README
        "textreverse" => "Reverse text strings online. Free tool reverses the order of characters in your text. Works instantly in your browser, offline.".to_string(),
        "linesorter" => "Sort lines of text alphabetically online. Free tool sorts lines in ascending order. Works entirely in your browser for privacy.".to_string(),
        "slugify" => "Convert text to a URL-friendly slug online. Free slug generator creates lowercase, dashed strings safe for URLs. Works offline in your browser.".to_string(),
        "csvtojson" => "Convert CSV data to JSON format online. Free tool transforms comma-separated values into a JSON array of objects. Secure, client-side conversion.".to_string(),
        "jsontocsv" => "Convert JSON array of objects to CSV format online. Free tool transforms JSON data into comma-separated values. Secure, client-side conversion.".to_string(),
        _ => format!(
            "Use Buup's free online {} tool. Securely {} directly in your browser - works offline with no server processing.",
            transformer_name,
            default_description.to_lowercase().replace("...", "") // Basic attempt to make it active
        ),
    }
}

// Components
#[component]
fn App() -> Element {
    // Read preferences from localStorage during initialization
    #[cfg(feature = "web")]
    let initial_theme = {
        use js_sys::{global, Function, Object};
        use wasm_bindgen::JsCast;

        let localStorage = js_sys::Reflect::get(&global(), &"localStorage".into())
            .ok()
            .and_then(|val| val.dyn_into::<Object>().ok());

        if let Some(storage) = localStorage {
            let get_item = js_sys::Reflect::get(&storage, &"getItem".into())
                .ok()
                .and_then(|val| val.dyn_into::<Function>().ok());

            if let Some(get_fn) = get_item {
                let theme_result = get_fn.call1(&storage, &"buup_dark_mode".into());
                if let Ok(theme_val) = theme_result {
                    if !theme_val.is_null() {
                        let theme_str = theme_val.as_string().unwrap_or_default();
                        theme_str == "true"
                    } else {
                        // Default to system preference if not found
                        js_sys::eval("window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches")
                            .map(|v| v.as_bool().unwrap_or(false))
                            .unwrap_or(false)
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    };

    #[cfg(not(feature = "web"))]
    let initial_theme = false;

    // Read the initially saved transformer ID if any
    #[cfg(feature = "web")]
    let initial_transformer_id = {
        use js_sys::{global, Function, Object};
        use wasm_bindgen::JsCast;

        // First check if there's a hash in the URL (e.g., #base64encode)
        let _window = web_sys::window().expect("Window should exist");
        let location = _window.location();
        let hash = location.hash().unwrap_or_default();

        // If hash exists and isn't empty, try to use it as transformer ID
        if !hash.is_empty() {
            // Remove the '#' prefix
            let hash = hash.trim_start_matches('#');

            // Check if this ID exists in our transformers
            if !hash.is_empty() && buup::transformer_from_id(hash).is_ok() {
                hash.to_string()
            } else {
                // If hash is invalid, fallback to localStorage
                let localStorage = js_sys::Reflect::get(&global(), &"localStorage".into())
                    .ok()
                    .and_then(|val| val.dyn_into::<Object>().ok());

                if let Some(storage) = localStorage {
                    let get_item = js_sys::Reflect::get(&storage, &"getItem".into())
                        .ok()
                        .and_then(|val| val.dyn_into::<Function>().ok());

                    if let Some(get_fn) = get_item {
                        let id_result = get_fn.call1(&storage, &"buup_transformer_id".into());
                        if let Ok(id_val) = id_result {
                            if !id_val.is_null() {
                                id_val
                                    .as_string()
                                    .unwrap_or_else(|| "base64encode".to_string())
                            } else {
                                "base64encode".to_string()
                            }
                        } else {
                            "base64encode".to_string()
                        }
                    } else {
                        "base64encode".to_string()
                    }
                } else {
                    "base64encode".to_string()
                }
            }
        } else {
            // No hash in URL, use localStorage
            let localStorage = js_sys::Reflect::get(&global(), &"localStorage".into())
                .ok()
                .and_then(|val| val.dyn_into::<Object>().ok());

            if let Some(storage) = localStorage {
                let get_item = js_sys::Reflect::get(&storage, &"getItem".into())
                    .ok()
                    .and_then(|val| val.dyn_into::<Function>().ok());

                if let Some(get_fn) = get_item {
                    let id_result = get_fn.call1(&storage, &"buup_transformer_id".into());
                    if let Ok(id_val) = id_result {
                        if !id_val.is_null() {
                            id_val
                                .as_string()
                                .unwrap_or_else(|| "base64encode".to_string())
                        } else {
                            "base64encode".to_string()
                        }
                    } else {
                        "base64encode".to_string()
                    }
                } else {
                    "base64encode".to_string()
                }
            } else {
                "base64encode".to_string()
            }
        }
    };

    #[cfg(not(feature = "web"))]
    let initial_transformer_id = "base64encode".to_string();

    // Initialize signals with saved values
    let mut is_dark_mode = use_signal(|| initial_theme);
    let mut current_transformer = use_signal(|| {
        Rc::new(
            buup::transformer_from_id(&initial_transformer_id)
                .unwrap_or_else(|_| buup::transformer_from_id("base64encode").unwrap()),
        )
    });
    let mut input = use_signal(|| "".to_string());
    let mut show_transformer_menu = use_signal(|| false);
    let mut transformer_category = use_signal(|| "all".to_string());
    let mut search_query = use_signal(String::new);
    let mut show_copy_success = use_signal(|| false);

    // Initialize categories signal with values from the core library
    let categorized = buup::categorized_transformers();
    let transformers = buup::all_transformers();

    // Get all categories for the menu
    let mut categories: Vec<_> = categorized.keys().collect();
    // Sort categories alphabetically for consistent ordering
    categories.sort_by_key(|c| c.to_string());

    // Save preferences when they change
    use_effect(move || {
        #[cfg(feature = "web")]
        {
            let dark_mode = is_dark_mode();
            let transformer_id = current_transformer().id();

            // Update URL hash when transformer changes
            if let Some(_window) = web_sys::window() {
                let _ = js_sys::eval(&format!("window.location.hash = '{}';", transformer_id));
            }

            let js_code = format!(
                r#"
                try {{
                    localStorage.setItem('buup_dark_mode', '{}');
                    localStorage.setItem('buup_transformer_id', '{}');
                }} catch (e) {{
                    console.error('Failed to save preferences:', e);
                }}
                "#,
                dark_mode, transformer_id
            );

            let _ = js_sys::eval(&js_code);
        }
    });

    // Add JavaScript click handler for closing menu when clicking outside
    #[cfg(feature = "web")]
    use_effect(move || {
        // Set up the click outside detection with plain JavaScript
        let js_code = r#"
            // Define a global click handler function
            function handleOutsideClick(event) {
                // Only run if the menu is open
                const menu = document.querySelector('.transformer-menu');
                if (!menu) return;
                
                const button = document.querySelector('.current-transformer');
                
                // If the click is outside both the menu and button, close the menu
                if (!menu.contains(event.target) && !button.contains(event.target)) {
                    // Create a custom event for closing the menu
                    document.dispatchEvent(new CustomEvent('buup:close-menu'));
                }
            }
            
            // Attach the handler to the document
            document.addEventListener('mousedown', handleOutsideClick);
        "#;

        // Run the script once to set up the event listener
        let _ = js_sys::eval(js_code);

        // Add a handler to close the menu on the custom event
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                // Create a closure that will close the transformer menu
                let mut menu_signal = show_transformer_menu;
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    menu_signal.set(false);
                })
                    as Box<dyn FnMut()>);

                // Attach the event listener
                let _ = document.add_event_listener_with_callback(
                    "buup:close-menu",
                    closure.as_ref().unchecked_ref(),
                );

                // Prevent the closure from being dropped
                closure.forget();
            }
        }
    });

    // Apply transformation and get output
    let output = if input().is_empty() {
        "".to_string()
    } else {
        match current_transformer().transform(&input()) {
            Ok(result) => result,
            Err(err) => err.to_string(),
        }
    };

    // Clone output for use in the clipboard function
    let output_for_clipboard = output.clone();

    // Function to copy to clipboard
    let copy_to_clipboard = move |_| {
        // Show the success indicator
        show_copy_success.set(true);

        // Copy to clipboard using a simple JS function
        #[cfg(feature = "web")]
        {
            // Create a JavaScript function to copy text
            let js_code = format!(
                r#"
                (function() {{
                    // Create temporary textarea
                    const el = document.createElement('textarea');
                    el.value = {};
                    el.style.position = 'absolute';
                    el.style.left = '-9999px';
                    document.body.appendChild(el);
                    el.select();
                    document.execCommand('copy');
                    document.body.removeChild(el);
                    return true;
                }})()
                "#,
                serde_json::to_string(&output_for_clipboard).unwrap()
            );

            // Execute the JS function
            let _ = js_sys::eval(&js_code);
        }

        // Reset the success indicator after 2 seconds
        let mut success_copy = show_copy_success;
        let timeout_callback = move || {
            success_copy.set(false);
        };

        #[cfg(feature = "web")]
        {
            // Set a timeout to hide the success message after 2 seconds
            gloo_timers::callback::Timeout::new(2000, timeout_callback).forget();
        }
    };

    // Theme colors
    let theme = if is_dark_mode() {
        Theme {
            bg: "#000000",
            surface: "#1C1C1E",
            text: "#FFFFFF",
            text_secondary: "rgba(255, 255, 255, 0.7)",
            border: "#38383A",
            accent: "#0A84FF",
            hover: "#2C2C2E",
        }
    } else {
        Theme {
            bg: "#FFFFFF",
            surface: "#F5F5F7",
            text: "#000000",
            text_secondary: "rgba(0, 0, 0, 0.7)",
            border: "#D2D2D7",
            accent: "#0066CC",
            hover: "#E8E8ED",
        }
    };

    // Function to swap between encoder/decoder pairs
    let swap_transform = move |_| {
        if let Some(inverse) = buup::inverse_transformer(&**current_transformer()) {
            // First, get the current output value
            let current_output = match input().is_empty() {
                true => "".to_string(),
                false => current_transformer()
                    .transform(&input())
                    .unwrap_or_else(|err| err.to_string()),
            };

            // Set the output as the new input
            input.set(current_output);

            // Switch to the inverse transformer
            current_transformer.set(Rc::new(inverse));

            // Update URL hash when swapping transformers
            if let Some(_window) = web_sys::window() {
                let new_id = inverse.id();
                let _ = js_sys::eval(&format!("window.location.hash = '{}';", new_id));
            }
        }
    };

    // Filter transformers based on selected category and search query
    let filtered_transformers = {
        let category_filtered = if transformer_category() == "all" {
            transformers.to_vec()
        } else {
            // Parse the category string to TransformerCategory enum
            if let Ok(category) = transformer_category().parse::<buup::TransformerCategory>() {
                // Get transformers for the selected category
                categorized.get(&category).unwrap().clone()
            } else {
                // Fallback to all transformers if category parsing fails
                transformers.to_vec()
            }
        };

        // If search query is empty, show all transformers for the selected category
        if search_query().is_empty() {
            category_filtered
        } else {
            // Filter transformers based on search query (match name or description)
            let search_lower = search_query().to_lowercase();
            category_filtered
                .into_iter()
                .filter(|transformer| {
                    transformer.name().to_lowercase().contains(&search_lower)
                        || transformer
                            .description()
                            .to_lowercase()
                            .contains(&search_lower)
                        || transformer.id().to_lowercase().contains(&search_lower)
                })
                .collect::<Vec<_>>()
        }
    };

    // Dynamic page title based on current transformer
    let page_title = format!(
        "{} | Buup - Text Utility Belt",
        current_transformer().name()
    );

    // Dynamic description based on current transformer
    let meta_description = get_meta_description(
        current_transformer().id(),
        current_transformer().name(),
        current_transformer().description(),
    );

    // Dynamic keywords based on transformer and its category
    let keywords = format!(
        "text utility, online tool, {}, {}, text transformer, online {}, web tool, free tool, secure, offline, no server",
        current_transformer().id(),
        current_transformer().name().to_lowercase(),
        current_transformer().category().to_string().to_lowercase()
    );

    // Generate canonical URL
    let canonical_url = format!(
        "https://benletchford.github.io/buup/#{}",
        current_transformer().id()
    );

    // Create a list of all tool names for rich results
    let _all_tool_names = transformers
        .iter()
        .map(|t| t.name())
        .collect::<Vec<_>>()
        .join(", ");

    // JSON-LD structured data for better SEO
    let structured_data = format!(
        r#"{{
            "@context": "https://schema.org",
            "@type": "WebApplication",
            "name": "Buup - Text Utility Belt",
            "url": "https://benletchford.github.io/buup/",
            "applicationCategory": "UtilitiesApplication",
            "offers": {{
                "@type": "Offer",
                "price": "0",
                "priceCurrency": "USD"
            }},
            "description": "Online text transformation toolkit with 50+ utilities including encoding/decoding, compression, formatting, and cryptography tools. Free, open-source, and works completely offline.",
            "operatingSystem": "Any",
            "browserRequirements": "Requires JavaScript",
            "keywords": "{}",
            "potentialAction": {{
                "@type": "UseAction",
                "object": {{
                    "@type": "SoftwareApplication",
                    "name": "{}"
                }}
            }}
        }}"#,
        keywords,
        current_transformer().name()
    );

    rsx! {
        // Dynamic page title
        document::Title { "{page_title}" }

        // Standard meta tags
        document::Meta { name: "description", content: "{meta_description}" }
        document::Meta { name: "keywords", content: "{keywords}" }
        document::Meta { name: "robots", content: "index, follow" }
        document::Meta { name: "author", content: "Buup" }

        // Canonical URL
        document::Link { rel: "canonical", href: "{canonical_url}" }

        // Open Graph tags
        document::Meta { property: "og:title", content: "{page_title}" }
        document::Meta { property: "og:description", content: "{meta_description}" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:url", content: "{canonical_url}" }
        document::Meta { property: "og:image", content: "https://benletchford.github.io/buup/apple-touch-icon.png" }
        document::Meta { property: "og:site_name", content: "Buup - Text Utility Belt" }

        // Twitter Card tags
        document::Meta { name: "twitter:card", content: "summary" }
        document::Meta { name: "twitter:title", content: "{page_title}" }
        document::Meta { name: "twitter:description", content: "{meta_description}" }
        document::Meta { name: "twitter:image", content: "https://benletchford.github.io/buup/apple-touch-icon.png" }

        // Structured data
        script { r#type: "application/ld+json", dangerous_inner_html: "{structured_data}" }

        // Keep existing meta tags
        document::Link {
            rel: "icon",
            type: "image/x-icon",
            href: FAVICON_ICO
        }
        document::Link {
            rel: "apple-touch-icon",
            sizes: "180x180",
            href: APPLE_TOUCH_ICON
        }
        document::Link {
            rel: "icon",
            type: "image/png",
            sizes: "32x32",
            href: FAVICON_32
        }
        document::Link {
            rel: "icon",
            type: "image/png",
            sizes: "16x16",
            href: FAVICON_16
        }
        document::Link {
            rel: "manifest",
            href: SITE_MANIFEST
        }
        document::Meta {
            name: "apple-mobile-web-app-capable",
            content: "yes"
        }
        document::Meta {
            name: "apple-mobile-web-app-status-bar-style",
            content: "default"
        }
        document::Meta {
            name: "apple-mobile-web-app-title",
            content: "Buup"
        }
        document::Meta {
            name: "theme-color",
            content: "#FFE0E9"
        }

        // Use the imported function for CSS generation
        style { { styles::generate_css(&theme) } }

        div { class: if is_dark_mode() { "container dark" } else { "container" },
            // Header section
            div { class: "header",
                div { class: "app-title",
                    img {
                        src: BUUP_ICON_SVG,
                        alt: "Buup logo",
                        style: "width: 30px; height: 30px; margin-right: 10px;"
                    }
                    "Buup"
                }
                div { class: "controls",
                    button {
                        class: "icon-button",
                        onclick: move |_| is_dark_mode.set(!is_dark_mode()),
                        if is_dark_mode() { "☀️" } else { "🌙" }
                    }
                }
            }

            // Transformer selector
            div { class: "transformer-selector",
                div {
                    class: "current-transformer",
                    tabindex: "0",
                    onclick: move |evt| {
                        // Toggle menu visibility
                        show_transformer_menu.set(!show_transformer_menu());

                        // Prevent default to ensure click works properly on mobile
                        evt.stop_propagation();

                        #[cfg(feature = "web")]
                        {
                            // If opening the menu, focus the search input
                            if !show_transformer_menu() {
                                // Use JS to ensure the button stays focusable
                                let js_code = r#"
                                    setTimeout(() => {
                                        const button = document.querySelector('.current-transformer');
                                        if (button) {
                                            button.focus();
                                        }
                                    }, 10);
                                "#;
                                let _ = js_sys::eval(js_code);
                            } else {
                                // If opening the menu, focus the search input
                                let js_code = r#"
                                    setTimeout(() => {
                                        const searchInput = document.querySelector('.search-input');
                                        if (searchInput) {
                                            searchInput.focus();
                                        }
                                    }, 10);
                                "#;
                                let _ = js_sys::eval(js_code);
                            }
                        }
                    },

                    div {
                        div { class: "transformer-name", "{current_transformer().name()}" }
                        div { class: "transformer-description", "{current_transformer().description()}" }
                    }

                    div {
                        class: if show_transformer_menu() { "arrow-icon open" } else { "arrow-icon" },
                        "▼"
                    }
                }

                // Dropdown menu for transformer selection
                {if show_transformer_menu() {
                    rsx! {
                        div {
                            class: "transformer-menu",
                            tabindex: "0",
                            onblur: move |evt| {
                                // Only close if the related target is not within the menu structure
                                #[cfg(feature = "web")]
                                {
                                    let js_code = r#"
                                        const menu = document.querySelector('.transformer-menu');
                                        const button = document.querySelector('.current-transformer');
                                        const related = document.activeElement;
                                        
                                        // Only close if focus is moving outside our components
                                        if (menu && button && related) {
                                            if (!menu.contains(related) && !button.contains(related)) {
                                                return true; // Close the menu
                                            }
                                        }
                                        return false; // Keep menu open
                                    "#;

                                    if let Ok(result) = js_sys::eval(js_code) {
                                        if let Some(close) = result.as_bool() {
                                            if close {
                                                show_transformer_menu.set(false);
                                            }
                                        }
                                    }
                                }

                                #[cfg(not(feature = "web"))]
                                {
                                    show_transformer_menu.set(false);
                                }

                                evt.stop_propagation();
                            },

                            // Search input
                            div { class: "search-container",
                                input {
                                    class: "search-input",
                                    r#type: "text",
                                    placeholder: "Search transformations...",
                                    value: "{search_query}",
                                    oninput: move |evt| search_query.set(evt.value().clone()),
                                    autofocus: true,
                                    onmounted: move |_| {
                                        #[cfg(feature = "web")]
                                        {
                                            // Focus the search input when mounted
                                            let js_code = r#"
                                                setTimeout(() => {
                                                    const searchInput = document.querySelector('.search-input');
                                                    if (searchInput) {
                                                        searchInput.focus();
                                                    }
                                                }, 50);
                                            "#;
                                            let _ = js_sys::eval(js_code);
                                        }
                                    },
                                }
                            }

                            // Categories
                            div { class: "transformer-categories",
                                button {
                                    class: if transformer_category() == "all" { "category-button active" } else { "category-button" },
                                    onclick: move |_| transformer_category.set("all".to_string()),
                                    "All"
                                }
                                {categories.iter().map(|category| {
                                    let category_str = category.to_string();
                                    rsx! {
                                        button {
                                            class: if transformer_category() == category_str { "category-button active" } else { "category-button" },
                                            onclick: move |_| {
                                                // Now we can directly use the category string
                                                transformer_category.set(category_str.clone());
                                            },
                                            "{&category.to_string()}"
                                        }
                                    }
                                })}
                            }

                            // Transformer list
                            div { class: "transformer-list",
                                {if filtered_transformers.is_empty() {
                                    rsx! {
                                        div { class: "no-results",
                                            "No transformations found"
                                        }
                                    }
                                } else {
                                    rsx! {
                                        {filtered_transformers.iter().map(|transformer| {
                                            let id = transformer.id();
                                            let name = transformer.name();
                                            let description = transformer.description();
                                            let is_current = current_transformer().id() == id;

                                            rsx! {
                                                div {
                                                    key: "{id}",
                                                    class: if is_current { "transformer-option active" } else { "transformer-option" },
                                                    onclick: move |evt| {
                                                        current_transformer.set(Rc::new(buup::transformer_from_id(id).unwrap()));
                                                        show_transformer_menu.set(false);
                                                        search_query.set(String::new());

                                                        // Update URL hash when changing transformer
                                                        if let Some(_window) = web_sys::window() {
                                                            let _ = js_sys::eval(&format!(
                                                                "window.location.hash = '{}';",
                                                                id
                                                            ));
                                                        }

                                                        // Stop event propagation to prevent issues
                                                        evt.stop_propagation();

                                                        #[cfg(feature = "web")]
                                                        {
                                                            // Use JS to ensure focus returns to the transformer selector
                                                            let js_code = r#"
                                                                setTimeout(() => {
                                                                    const button = document.querySelector('.current-transformer');
                                                                    if (button) {
                                                                        button.focus();
                                                                    }
                                                                }, 10);
                                                            "#;
                                                            let _ = js_sys::eval(js_code);
                                                        }
                                                    },

                                                    div { class: "option-name", "{name}" }
                                                    div { class: "option-description", "{description}" }
                                                }
                                            }
                                        })}
                                    }
                                }}
                            }
                        }
                    }
                } else {
                    rsx! {}
                }}
            }

            // Input/Output panels
            div { class: "panels",
                // Input panel
                div { class: "panel",
                    div { class: "panel-header",
                        div { class: "panel-title", "Input" }
                        div { class: "panel-actions",
                            button {
                                class: "action-button",
                                title: "Clear input",
                                onclick: move |_| input.set("".to_string()),
                                "✕"
                            }
                        }
                    }
                    div { class: "textarea-container",
                        textarea {
                            class: "textarea",
                            value: "{input}",
                            oninput: move |evt| input.set(evt.value().clone()),
                            placeholder: "{current_transformer().default_test_input()}",
                        }
                    }
                }

                // Swap button
                div { class: "swap-button-container",
                    button {
                        class: "swap-button",
                        onclick: swap_transform,
                        title: "Swap transformation",
                        "⇄"
                    }
                }

                // Output panel
                div { class: "panel",
                    div { class: "panel-header",
                        div { class: "panel-title", "Output" }
                        div { class: "panel-actions",
                            button {
                                class: "copy-button",
                                title: "Copy to clipboard",
                                onclick: copy_to_clipboard,
                                // Add data attribute for clipboard.js to use
                                "data-clipboard-text": "{output}",
                                svg {
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path {
                                        d: "M16 1H4C2.9 1 2 1.9 2 3V17H4V3H16V1ZM19 5H8C6.9 5 6 5.9 6 7V21C6 22.1 6.9 23 8 23H19C20.1 23 21 22.1 21 21V7C21 5.9 20.1 5 19 5ZM19 21H8V7H19V21Z"
                                    }
                                }
                                div {
                                    class: if show_copy_success() { "copy-success visible" } else { "copy-success" },
                                    "Copied!"
                                }
                            }
                        }
                    }
                    div { class: "textarea-container",
                        textarea {
                            class: "textarea",
                            value: "{output}" ,
                            readonly: true,
                            placeholder: "{current_transformer().transform(current_transformer().default_test_input()).unwrap_or_else(|err| err.to_string())}",
                        }
                    }
                }
            }

            // Footer
            div { class: "footer",
                span {
                    {"Made with "}
                    span { class: "heart", "❤" }
                    {" by "}
                    a {
                        href: "https://benletchford.com",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Ben Letchford"
                    }
                    {" "}
                }
                span {
                    a {
                        href: format!("{}/releases/tag/v{}", env!("CARGO_PKG_REPOSITORY"), env!("CARGO_PKG_VERSION")),
                        target: "_blank",
                        rel: "noopener noreferrer",
                        {format!("({}-{})", env!("CARGO_PKG_VERSION"), env!("BUUP_WEB_GIT_HASH"))}
                    }
                }
            }
        }
    }
}

// Move Theme struct here as it's used by styles.rs now
#[derive(Debug, Clone)] // Add Clone and Debug for potential future uses
pub struct Theme {
    pub bg: &'static str,
    pub surface: &'static str,
    pub text: &'static str,
    pub text_secondary: &'static str,
    pub border: &'static str,
    pub accent: &'static str,
    pub hover: &'static str,
}
