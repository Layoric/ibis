use crate::frontend::{
    api::CLIENT,
    app::{is_logged_in, site, DefaultResource},
    dark_mode::DarkMode,
};
use leptos::{component, prelude::*, view, IntoView, *};
use leptos_router::{components::A, hooks::use_navigate};

#[component]
pub fn Nav() -> impl IntoView {
    let logout_action = Action::new(move |_| async move {
        CLIENT.logout().await.unwrap();
        site().refetch();
    });
    let notification_count = Resource::new(
        || (),
        move |_| async move { CLIENT.notifications_count().await.unwrap_or_default() },
    );

    let (search_query, set_search_query) = signal(String::new());
    let mut dark_mode = expect_context::<DarkMode>();
    view! {
        <nav class="max-sm:navbar p-2.5 h-full md:fixed md:w-64 max-sm: border-b md:border-e border-slate-400 border-solid">
            <div
                id="navbar-start"
                class="max-sm:navbar-start max-sm:flex max-sm:dropdown max-sm:dropdown-bottom max-sm:dropdown-end max-sm:w-full md:h-full"
            >
                <h1 class="w-min md:hidden text-3xl font-bold font-serif">
                    {CLIENT.hostname.clone()}
                </h1>
                <div class="flex-grow md:hidden"></div>
                <button tabindex="0" class="btn btn-outline lg:hidden">
                    Menu
                </button>
                <div
                    tabindex="0"
                    class="menu dropdown-content p-2 max-sm:rounded-box max-sm:z-[1] max-sm:shadow md:h-full"
                >
                    <img src="/logo.png" class="m-auto max-sm:hidden" />
                    <h1 class="px-4 py-2 text-3xl font-bold font-serif sm:hidden">
                        {CLIENT.hostname.clone()}
                    </h1>
                    <ul>
                        <li>
                            <A href="/">"Main Page"</A>
                        </li>
                        <li>
                            <A href="/instances">"Instances"</A>
                        </li>
                        <li>
                            <A href="/articles">"Articles"</A>
                        </li>
                        <Suspense>
                            <Show when=is_logged_in>
                                <li>
                                    <A href="/create-article">"Create Article"</A>
                                </li>
                                <li>
                                    <A href="/notifications">
                                        "Notifications "
                                        <span class="indicator-item indicator-end badge badge-neutral">
                                            <Suspense>{move || notification_count.get()}</Suspense>
                                        </span>
                                    </A>
                                </li>
                            </Show>
                        </Suspense>
                        <li>
                            <form
                                class="form-control m-0 p-1"
                                on:submit=move |ev| {
                                    ev.prevent_default();
                                    let navigate = use_navigate();
                                    let query = search_query.get();
                                    if !query.is_empty() {
                                        navigate(
                                            &format!("/search?query={query}"),
                                            Default::default(),
                                        );
                                    }
                                }
                            >
                                <input
                                    type="text"
                                    class="input input-secondary input-bordered input-xs w-full rounded"
                                    placeholder="Search"
                                    prop:value=search_query
                                    on:keyup=move |ev: ev::KeyboardEvent| {
                                        let val = event_target_value(&ev);
                                        set_search_query.update(|v| *v = val);
                                    }
                                />

                                <button class="btn btn-xs btn-secondary">Go</button>
                            </form>
                        </li>
                    </ul>
                    <div class="divider"></div>
                    <Suspense>
                        <Show
                            when=is_logged_in
                            fallback=move || {
                                view! {
                                    <li>
                                        <A href="/login">"Login"</A>
                                    </li>
                                    <Show when=move || {
                                        site().with_default(|s| s.config.registration_open)
                                    }>
                                        <li>
                                            <A href="/register">"Register"</A>
                                        </li>
                                    </Show>
                                }
                            }
                        >

                            {
                                let my_profile = site()
                                    .with_default(|site| site.clone().my_profile.unwrap());
                                let profile_link = format!("/user/{}", my_profile.person.username);
                                view! {
                                    <p class="self-center pb-2">
                                        "Logged in as " <a class="link" href=profile_link>
                                            {my_profile.person.username}
                                        </a>
                                    </p>
                                    <button
                                        class="btn btn-outline btn-xs w-min self-center"
                                        on:click=move |_| {
                                            logout_action.dispatch(());
                                        }
                                    >
                                        Logout
                                    </button>
                                }
                            }

                        </Show>
                    </Suspense>
                    <div class="grow min-h-2"></div>
                    <div class="m-1 grid gap-2">
                        <label class="flex cursor-pointer gap-2">
                            <span class="label-text">Light</span>
                            <input
                                type="checkbox"
                                class="toggle"
                                prop:checked=dark_mode.is_dark
                                on:click=move |_| { dark_mode.toggle() }
                            />
                            <span class="label-text">Dark</span>
                        </label>
                        <p>"Version "{env!("CARGO_PKG_VERSION")}</p>
                        <p>
                            <a href="https://github.com/Nutomic/ibis" class="link">
                                Source Code
                            </a>
                        </p>
                    </div>
                </div>
            </div>
        </nav>
    }
}
