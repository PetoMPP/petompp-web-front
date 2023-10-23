use crate::{
    data::locales::{store::LocalesStore, tk::TK},
    pages::page_base::PageBase,
    router::route::Route,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(NewPost)]
pub fn new_post() -> Html {
    let navigator = use_navigator().unwrap();
    let (locales_store, _) = use_store::<LocalesStore>();
    let onclick = Callback::from(move |_| navigator.push(&Route::BlogRoot));
    html! {
        <PageBase>
            <a class={"lg:mb-6 mb-4"} href={"javascript:void(0);"} {onclick}>{locales_store.get(TK::BackToBlogPosts)}</a>
            <div class="flex flex-col items-center justify-center">
                <h1 class="text-4xl font-bold">{"New Post"}</h1>
                <div class="flex flex-col items-center justify-center">
                    <input type="text" placeholder="Title" class="w-1/2 p-2 border-2 border-gray-300 rounded-lg"/>
                    <input type="text" placeholder="Tags" class="w-1/2 p-2 border-2 border-gray-300 rounded-lg"/>
                    <textarea placeholder="Content" class="w-1/2 p-2 border-2 border-gray-300 rounded-lg"/>
                    <button class="w-1/2 p-2 border-2 border-gray-300 rounded-lg">{"Submit"}</button>
                </div>
            </div>
        </PageBase>
    }
}
