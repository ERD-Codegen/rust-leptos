use leptos::*;
use leptos_router::*;

use super::buttons::{ButtonFav, ButtonFollow};

pub type ArticleSignal = RwSignal<crate::models::ArticlePreview>;

type ArticlesType<S, T = Result<Vec<crate::models::ArticlePreview>, ServerFnError>> =
    Resource<S, T>;

#[component]
pub fn ArticlePreviewList<S: 'static + std::clone::Clone>(
    username: crate::auth::UsernameSignal,
    articles: ArticlesType<S>,
) -> impl IntoView {
    // TODO: When the user logouts in the main screen, there's a request to articles... WHY?
    let articles_view = move || {
        articles.with(move |x| {
            x.clone().map(move |res| {
                view! {
                    <For
                        each=move || res.clone().into_iter().enumerate()
                        key=|(i, _)| *i
                        view=move |(_, article): (usize, crate::models::ArticlePreview)| {
                            let article = create_rw_signal(article);
                            view! {
                                <ArticlePreview article=article username=username />
                            }
                        }
                    />
                }
            })
        })
    };

    view! {
        <Suspense fallback=move || view! {<p>"Loading Articles"</p> }>
            <ErrorBoundary fallback=|_| {
                view! { <p class="error-messages text-xs-center">"Something went wrong."</p>}
            }>
                {articles_view}
            </ErrorBoundary>
        </Suspense>
    }
}

#[component]
fn ArticlePreview(username: crate::auth::UsernameSignal, article: ArticleSignal) -> impl IntoView {
    view! {
        <div class="article-preview">
            <ArticleMeta username=username article=article is_preview=true />
            // {{self::metadata(article=article, is_preview=true)}}
            <A href=move || format!("/article/{}", article.with(|x| x.slug.clone())) class="preview-link">
                <h1>{move || article.with(|x| x.title.to_string())}</h1>
                <p>{move || article.with(|x| x.description.to_string())}</p>
                <span class="btn">"Read more..."</span>
                <Show
                    when=move || article.with(|x| !x.tags.is_empty())
                    fallback=|| view! {<span>"No tags"</span>}
                >
                    <ul class="tag-list">
                        <i class="ion-pound"></i>
                        <For
                            each=move || article.with(|x| x.tags.clone().into_iter().enumerate())
                            key=|(i, _)| *i
                            view=move |(_, tag): (usize, String)| {
                                view!{<li class="tag-default tag-pill tag-outline">{tag}</li>}
                            }
                        />
                    </ul>
                </Show>
            </A>
        </div>
    }
}

#[component]
fn ArticleMeta(
    username: crate::auth::UsernameSignal,
    article: ArticleSignal,
    is_preview: bool,
) -> impl IntoView {
    let editor_ref = move || format!("/editor/{}", article.with(|x| x.slug.to_string()));
    let profile_ref = move || {
        format!(
            "/profile/{}",
            article.with(|x| x.author.username.to_string())
        )
    };

    view! {
        <div class="article-meta">
            <A href=profile_ref><img src=move || article.with(|x| x.author.image.clone().unwrap_or_default()) /></A>
            <div class="info">
                <A href=profile_ref class="author">{move || article.with(|x| x.author.username.to_string())}</A>
                <span class="date">{move || article.with(|x| x.created_at.to_string())}</span>
            </div>
            <Show
                when=move || is_preview
                fallback=move || {
                    view! {
                        <Show
                            when=move || {username.get().unwrap_or_default() == article.with(|x| x.author.username.to_string())}
                            fallback=move || {
                                let following = article.with(|x| x.author.following);
                                let (author, _) = create_signal(article.with(|x| x.author.username.to_string()));
                                view!{
                                <Show when=move || username.with(Option::is_some) fallback=|| ()>
                                    <ButtonFav username=username article=article />
                                    <ButtonFollow logged_user=username author following />
                                </Show>
                            }}
                        >
                            <A class="btn btn-sm btn-outline-secondary" href=editor_ref>
                                <i class="ion-compose"></i>" Edit article"
                            </A>
                            <form method="post" action="{{routes.article ~ '/' ~ article.slug ~ '/delete'}}" style="display: inline-block;">
                                <button type="submit" class="btn btn-sm btn-outline-secondary">
                                    <i class="ion-trash-a"></i>" Delete article"
                                </button>
                            </form>
                        </Show>
                    }
                }
            >
                <ButtonFav username=username article=article />
            </Show>
        </div>
    }
}
