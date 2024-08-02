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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PostResponse {
    id: u32,
    title: String,
    content: String,
    created_at: String,
    content_html: String,
}

impl From<&Post> for PostResponse {
    fn from(value: &Post) -> Self {
        let clone = &value.content.clone();
        PostResponse {
            id: value.id,
            title: value.title.clone(),
            content: value.content.clone(),
            created_at: value.created_at.clone(),
            content_html: markdown::to_html(clone),
        }
    }
}

static CSS: &str = r#"
  <style>
    html {
      font-family: monospace;
      padding: 16px;
    }

    nav > a {
      padding: 4px;
      color: white;
      background: black;
    }

    .feed {
      max-width: 500px;
      display: grid;
      gap: 12px;
    }

    .card {
        padding: 16px;
        border: 2px solid black;
        overflow: auto;
    }

    .card .title, .card .content {
        font-size: 1.5rem;
    }

    form {
        margin-top: 24px;
    }

    img {
        max-width: 100%;
    }
</style>
"#;

static NAV: &str = r#"
<nav>
    <a href="/feed">Feed</a>
    <a href="/post/form">New post</a>
    <a href="/posts">Posts JSON</a>
</nav>
"#;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/", |_req, _ctx| async move {
            Response::from_html(r#"
            <!doctype html>
            <script>
              window.location.href = "/feed";
            </script>
            {{NAV}}"#.replace("{{NAV}}", NAV))
        })
        .get_async("/posts", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("SELECT * FROM posts");
            let result = statement.all().await?;
            let posts: Vec<Post> = result.results::<Post>().unwrap();
            let post_responses: Vec<PostResponse> = posts.iter().map(|p: &Post|
                p.into()
            ).collect();
            Response::from_json(&post_responses)
        })
        .get_async("/feed", |_, _ctx| async move {
            Response::from_html(
                r#"
                <!doctype html>
                <head>
                    {{CSS}}
                </head>
                <body>
                {{NAV}}
                <h1>Feed</h1>
                <p>🟢 Live <span id="refreshedAt"></span></p>
                <div id="feed" class="feed">
                </div>
                <script>
                    async function fetchPosts() {
                        const response = await fetch('/posts');
                        const posts = await response.json();
                        const feed = document.getElementById('feed');
                        feed.innerHTML = posts.sort((a, b) => Date.parse(b.created_at) - Date.parse(a.created_at)).map(post => `
                            <div class="card">
                                <h2 class="title">${post.title}</h2>
                                <p class="content">${post.content_html}</p>
                                <p class="created">${post.created_at}</p>
                            </div>
                        `).join('');
                        const refreshed = document.getElementById('refreshedAt');
                        refreshed.innerText = new Date().toLocaleTimeString();
                    }

                    fetchPosts();
                    setInterval(fetchPosts, 5000);
                </script>
                </body>
                "#.replace("{{CSS}}", CSS)
                .replace("{{NAV}}", NAV))
        })
        .get_async("/posts/:id", |_, ctx| async move {
            let id = ctx.param("id").unwrap();
            let d1 = ctx.env.d1("DB")?;
            let statement: D1PreparedStatement = d1.prepare("SELECT * FROM posts WHERE id = ?1");
            let query = statement.bind(&[id.into()])?;
            let result = query.first::<Post>(None).await?;
            match result {
                Some(post) => Response::from_json(&post),
                None => Response::error("Not found", 404),
            }
        })
        .post_async("/post/form", |mut req, ctx| async move {
            let payload = req.form_data().await?;

            let supersecure = ctx.env.secret("PASSWORD")?.to_string();
            if payload.get_field("password").expect("no password") != supersecure {
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
                <head>
                    {{CSS}}
                </head>
                <body>
                {{NAV}}
                <form action="/post/form" method="post" style="display: flex; flex-direction: column; gap: 16px; max-width: 800px;">
                    <label>
                      <div>Title</div>
                      <input type="text" name="title" required spellcheck style="width: 100%" />
                    </label>
                    <label>
                      <div>Content (Markdown supported)</div>
                      <textarea name="content" spellcheck style="width: 100%; min-height: 200px;" required></textarea>
                    </label>

                    <label>
                      <div>Password</div>
                      <input type="password" name="password" required />
                    </label>
                    
                    <button type="submit">Submit</button>
                </form>
                </body>
            "#.replace("{{CSS}}", CSS).replace("{{NAV}}", NAV),
            )
        })
        .run(req, env)
        .await
}
