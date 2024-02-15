use crate::frontend::article_title;
use crate::frontend::components::article_nav::ArticleNav;
use crate::frontend::markdown::markdown_parser;
use crate::frontend::pages::article_resource;
use leptos::*;

#[component]
pub fn ReadArticle() -> impl IntoView {
    let article = article_resource();

    view! {
        <ArticleNav article=article/>
        <Suspense fallback=|| view! {  "Loading..." }> {
            let parser = markdown_parser();
            move || article.get().map(|article|
            view! {
                <div class="item-view">
                    <h1>{article_title(&article.article)}</h1>
                    <div inner_html={parser.parse(&article.article.text).render()}/>
                </div>
            })
        }
        </Suspense>
    }
}
