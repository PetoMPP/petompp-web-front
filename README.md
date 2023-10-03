# Main PetoMPP webpage frontend

This is the repository containing the source code and deployments workflows for my webpage at: https://petompp.net/.

## Used technologies

For this project I decided to go with Rust + WASM instead of JS as I never developed anything in JavaScript.

The framework I used to create the app is [Yew.rs](https://yew.rs/). It is component based framework similar to React.

The tool for building and serving the website is [Trunk.rs](https://trunkrs.dev/).
This tool builds WASM binary and JS linking code alongside TailwindCSS processing.

TailwindCSS is managed directly by Trunk, but plugins have to be installed using npm as seen in dockerfile.

The most significant plugin is the [daisyUI](https://daisyui.com/) one.
It provides a decent Tailwind components collection.

## Features

### User Interface

The idea for this project was to prove myself that I can create a decently looking site.

Because of that I went through the hassle of combining npm and cargo(trunk) package management,
which may not seem difficult, but having no prior experience with npm didn't help.
The lack of experience of using npm in Docker environment (so called Linux) didn't help the most.

The hassle in my opinion was worth it, as I managed to make the UI look modern, clean, responsive, mobile friendly and consistent.

### Markdown viewer

Most pages on the website (Home, About, Contact) are built from a markdown string.

Those are being generated by a [markdown-rs](https://github.com/wooorm/markdown-rs) library, that converts markdown into HTML,
which is later set as inner of the display element.

The display element is styled with *prose* class from [Typography](https://tailwindcss.com/docs/typography-plugin) plugin for Tailwind, so it looks very nice for majority of MD features.
Also some of the <a> elements contained have to be adjusted so local hrefs do not reload a page.

### Markdown editor

Although hidden for non-admin users, there's an editor built into the app for the web pages content.
For the time being it is only accessible by navigating directly to it at https://petompp.net/editor/home-content/en.

The editor features local changes saving and immidiate update of webpage resources.
Frontend doesn't restrict unregistered users from trying to update the resources, but API shouldn't allow that.

It also features a kind of decent UI that allows to quickly switch between resources and languages,
with prompts in place to prevent you from losing any unsaved changes.

## Feedback

I am very fresh in the world of the web and any feedback, issues and overall thoughts are more then welcome and I'm happy to hear them all :)
