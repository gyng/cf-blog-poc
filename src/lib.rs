use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
    content: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostNewForm {
    title: String,
    content: String,
    password: String,
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/", |_, _ctx| async move {
            // A greeting when accessing the root.
            Response::from_html(r#"
            <!doctype html>
                <nav>
                    <a href="/">Home</a>
                    <a href="/posts">Posts</a>
                    <a href="/feed">Feed</a>
                    <a href="/post/form">New post</a>
                </nav>"#)
        })
        .get_async("/posts", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("SELECT * FROM posts");
            let result = statement.all().await?;
            let posts = &result.results::<Post>().unwrap();
            dbg!(posts);
            Response::from_json(&result.results::<Post>().unwrap())
        })
        .get_async("/feed", |_, ctx| async move {
           
            // return html with polling from /posts
            Response::from_html(
                r#"
                <!doctype html>
                <nav>
                    <a href="/">Home</a>
                    <a href="/posts">Posts</a>
                    <a href="/feed">Feed</a>
                    <a href="/post/form">New post</a>
                </nav>
                <h1>Feed</h1>
                <div id="feed"></div>
                <script>
                    async function fetchPosts() {
                        const response = await fetch('/posts');
                        const posts = await response.json();
                        const feed = document.getElementById('feed');
                        feed.innerHTML = posts.sort((a, b) => Date.parse(b.created_at) - Date.parse(a.created_at)).map(post => `
                            <div>
                                <h2>${post.title}</h2>
                                <p>${post.content}</p>
                                <p>${post.created_at}</p>
                            </div>
                        `).join('');
                    }

                    fetchPosts();
                    setInterval(fetchPosts, 5000);
                </script>
                "#)
        })
        .get_async("/posts/:id", |_, ctx| async move {
            let id = ctx.param("id").unwrap();
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("SELECT * FROM posts WHERE id = ?1");
            let query = statement.bind(&[id.into()])?;
            let result = query.first::<Post>(None).await?;
            match result {
                Some(post) => Response::from_json(&post),
                None => Response::error("Not found", 404),
            }
        })
        .post_async("/post/form", |mut req, ctx| async move {
            // let payload = req.json::<Post>().await?;

            let payload = req.form_data().await?;

            let supersecure = "???";
            if (payload.get_field("password").expect("no password") != supersecure) {
                return Response::error("Unauthorized", 401);
            }

            let d1 = ctx.env.d1("DB")?;
            let now = chrono::offset::Utc::now().to_rfc3339();

            let statement =
                d1.prepare("INSERT INTO posts (title, content, created_at) VALUES (?1, ?2, ?3)");
            let query = statement.bind(&[
                payload.get_field("title").expect("no title").into(),
                payload.get_field("content").expect("no content").into(),
                now.into(),
            ])?;

            let result = query.run().await?;

            console_log!("result: {:?}", result.success());
            Response::ok(format!("Successfully added: {:?}", payload))
        })
        .get_async("/post/form", |_, _ctx| async move {
            Response::from_html(
                r#"
                <!doctype html>
                <nav>
                    <a href="/">Home</a>
                    <a href="/posts">Posts</a>
                    <a href="/feed">Feed</a>
                    <a href="/post/form">New post</a>
                </nav>
                <form action="/post/form" method="post" style="display: flex; flex-direction: column; gap: 16px;">
                    <label>
                      <div>Title</div>
                      <input type="text" name="title" required />
                    </label>
                    <label>
                      <div>Content</div>
                      <textarea name="content" spellcheck style="width: 100%; height: 400px;" required></textarea>
                    </label>

                    <label>
                      <div>Password</div>
                      <input type="password" name="password" required />
                    </label>
                    
                    <button type="submit">Submit</button>
                </form>
            "#,
            )
        })
        .run(req, env)
        .await
}
