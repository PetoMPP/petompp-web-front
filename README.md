# Main PetoMPP webpage frontend

This is the repository containing the source code and deployments workflows for my webpage's frontend at: https://peto-main.azurewebsites.net/.

## Used technologies

For this project I decided to go with Rust + WASM instead of JS as I never developed anything in JavaScript.

The framework I used to create the app is [Yew.rs](https://yew.rs/).
It is component based framework similar to React.js, as seen on an example component
```rust
#[function_component(Logo)]
pub fn logo() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <button class={"btn btn-ghost text-xl"} {onclick}>{"PetoMPP.NET"}</button>
    }
}
```

The tool for building and serving the website is [Trunk.rs](https://trunkrs.dev/).
This tool builds WASM binary and JS linking code alongside TailwindCSS processing and runs a webserver to serve the website.

TailwindCSS is managed directly by Trunk, but plugins have to be installed using npm as seen in dockerfile.

The most significant plugin is the [daisyUI](https://daisyui.com/) one.
It provides a decent Tailwind components collection.

## Features

### User Interface

The idea for this project was to prove myself that I can create a decently looking site.

Because of that I went through the hassle of combining npm and cargo(trunk) package management systems,
which may not seem difficult, but having no prior experience with npm didn't help.
The lack of experience of using npm in Docker environment (so called Linux) didn't help the most.

The hassle in my opinion was worth it, as I managed to make the UI look modern, clean, responsive, mobile-friendly and consistent.

### Markdown viewer

Most pages on the website (Home, About, Contact) are built from a markdown string.

Those are being generated by a [markdown-rs](https://github.com/wooorm/markdown-rs) library, that converts markdown into HTML,
which is later set as inner of the display element. It is all handled by [Markdown](/src/components/atoms/markdown.rs) component
```rust
#[function_component(Markdown)]
pub fn markdown_display(props: &MarkdownDisplayProps) -> Html {
    let navigator = use_navigator().unwrap();
    let html = markdown::to_html_with_options(
        props.markdown.as_str(),
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: props.allowhtml,
                ..markdown::CompileOptions::default()
            },
            parse: markdown::ParseOptions::gfm(),
        },
    )
    .unwrap();
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&html);
    let interactive = props.interactive;
    let class = match interactive {
        Some(()) => "prose w-full max-w-full",
        None => "prose w-full max-w-full pointer-events-none",
    };
    use_effect_deps!(|interactive| {
        if interactive.is_some() {
            make_links_clickable(navigator.clone());
        }
        || {}
    });

    html! {
        <div {class} id={ID}>
            {Html::VRef(div.into())}
        </div>
    }
}
```

The display element is styled with *prose* class from [Typography](https://tailwindcss.com/docs/typography-plugin) plugin for Tailwind, so it looks very nice for majority of MD features.
Also some of the `<a>` elements contained have to be adjusted so local hrefs do not reload a page in `make_links_clickable` function.

### Resources editor

Although hidden for non-admin users, there's an editor built into the app for the pages content and blog posts.
For the time being it is only accessible by navigating directly to it at https://peto-main.azurewebsites.net/editor.

The editor features caching local changes saving and update of webpage resources and blog posts, including their metadata like tags, summary and splash image.
In Blog meta editor user can select and upload images on Azure blob stroage with a nice UI.

Frontend doesn't restrict unregistered users from trying to update the resources, but API shouldn't allow that.

It also features a kind of decent UI that allows to quickly switch between resources and languages,
with prompts in place to prevent you from losing any unsaved changes.

The editor is also capable of adding images from users to the text by uploading them to blob storage.
It is implemented as a dragdrop and onpaste events, in the future there'll be the way do to such uploads from mobile.

## Feedback

I am very fresh in the world of the web and any feedback, issues and overall thoughts are more then welcome and I'm happy to hear them all :)
