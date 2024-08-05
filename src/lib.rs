use std::fmt::Display;

use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ThreadModel {
    id: u64,
    title: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ThreadNewRequest {
    title: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ThreadView {
    id: u64,
    title: String,
    created_at: String,
}

impl From<&ThreadModel> for ThreadView {
    fn from(value: &ThreadModel) -> Self {
        ThreadView {
            id: value.id,
            title: value.title.clone(),
            created_at: value.created_at.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PostModel {
    id: u64,
    thread_id: u64,
    author: String,
    content: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PostNewRequest {
    title: String,
    content: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PostView {
    id: u64,
    author: String,
    content: String,
    created_at: String,
    content_html: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FeedView {
    thread: ThreadView,
    posts: Vec<PostView>,
}

impl From<&PostModel> for PostView {
    fn from(value: &PostModel) -> Self {
        let clone = &value.content.clone();
        PostView {
            id: value.id,
            author: value.author.clone(),
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

    ol, ul {
      margin: 0;
      padding: 0;
    }

    nav {
      margin-bottom: 32px;
    }

    nav > a {
      padding: 4px;
      color: white;
      background: black;
    }

    .feed {
      width: 100%;
      max-width: 600px;
      display: grid;
      gap: 12px;
    }

    .card {
      padding: 16px;
      border: 2px solid black;
      overflow: auto;
    }

    .content {
      font-size: 1.5rem;
    }

    .content img {
      max-width: 100%;
      max-height: 400px;
      margin: 8px auto;
    }
</style>
"#;

static NAV: &str = r#"
<nav>
    <a href="/ui/threads">Threads</a>
    <a href="/admin_ui/thread/form">New thread</a>
    <a href="/admin_ui/post/form">New post</a>
    <a href="/posts.json">Posts JSON</a>
    <a href="/threads.json">Threads JSON</a>
    <a href="/threads/2/feed.json">Thread feed JSON</a>
</nav>
"#;

fn template<S: AsRef<str> + Display>(body: S) -> String {
    format!(
        r#"
        <!doctype html>
        <head>
            {}
        </head>
        <body>
            {}
            {}
        </body>
    "#,
        CSS, NAV, body
    )
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get_async("/", |_req, _ctx| async move {
            Response::from_html(r#"
            <!doctype html>
            <script>
              window.location.href = "/ui/threads";
            </script>
            {{NAV}}"#.replace("{{NAV}}", NAV))
        })

        .get_async("/posts.json", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("SELECT * FROM posts");
            let result = statement.all().await?;
            let posts: Vec<PostModel> = result.results::<PostModel>().unwrap();
            let post_responses: Vec<PostView> = posts.iter().map(|p: &PostModel|
                p.into()
            ).collect();
            Response::from_json(&post_responses)
        })
        .get_async("/threads.json", |_, ctx| async move {
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("SELECT * FROM threads");
            let result = statement.all().await?;
            let threads: Vec<ThreadModel> = result.results().unwrap();
            let threads_response: Vec<ThreadView> = threads.iter().map(|t: &ThreadModel| {
                ThreadView {
                    id: t.id,
                    title: t.title.clone(),
                    created_at: t.created_at.clone(),
                }
            }).collect();
            Response::from_json(&threads_response)
        })
        .get_async("/threads/:id/feed.json", |_, ctx| async move {
            let id: String = match ctx.param("id").unwrap().parse::<u64>() {
                Ok(id) => id.to_string(),
                Err(_) => return Response::error("Bad ID", 400),
            };

            let d1 = ctx.env.d1("DB")?;

            let thread_statement: D1PreparedStatement = d1.prepare("SELECT * FROM threads t WHERE t.id = ?1");
            let thread_query = thread_statement.bind(&[id.clone().into()])?;
            let thread_result = thread_query.first::<ThreadModel>(None).await?;
            let thread = if let Some(t) = thread_result {
                t
            } else {
                return Response::error("Not found", 404);
            };

            let posts_statement: D1PreparedStatement = d1.prepare("SELECT * FROM posts p WHERE p.thread_id = ?1 ORDER BY p.created_at DESC");
            let posts_query = posts_statement.bind(&[id.into()])?;
            let posts_result = posts_query.all().await?;
            let posts: Vec<PostModel> = posts_result.results::<PostModel>().unwrap();
            let posts_response : Vec<PostView> = posts.iter().map(|p: &PostModel| {
                p.into()
            }).collect();

            let feed_response = FeedView {
                thread: (&thread).into(),
                posts: posts_response,
            };

            Response::from_json(&feed_response)
        })

        .get_async("/ui/threads", |_, _| async move {
            Response::from_html(template(r#"
            <h1>Threads</h1>
                <ul id="threads" class="feed threads"></ul>
                <script>
                    async function fetchThreads() {
                        const response = await fetch('/threads.json');
                        const threads = await response.json();
      
                        document.getElementById('threads').innerHTML = threads.sort((a, b) => Date.parse(b.created_at) - Date.parse(a.created_at)).map(thread => `
                            <a href="/ui/threads/${thread.id}/feed">
                                <div class="card">
                                    <div class="created">${new Date(thread.created_at).toLocaleString()}</div>
                                    <h2 class="title">${thread.title}</h2>
                                </div>
                            </a>
                        `).join('');
                    }
                    fetchThreads();
                    setInterval(fetchThreads, 5000);
                </script>
            
            "#))
        })
        .get_async("/ui/threads/:id/feed", |_, ctx| async move {
            let id: String = match ctx.param("id").unwrap().parse::<u64>() {
                Ok(id) => id.to_string(),
                Err(_) => return Response::error("Bad ID", 400),
            };
            Response::from_html(template(r#"
            <h1 id="threadTitle">&nbsp;</h1>
                <p>🟢 Live <span id="refreshedAt"></span></p>
                <div id="feed" class="feed">
                </div>
                <script>
                    // https://stackoverflow.com/questions/6108819/javascript-timestamp-to-relative-time
                    function relativeTimeAgo(d1) {
                        // in miliseconds
                        var units = {
                            year  : 24 * 60 * 60 * 1000 * 365,
                            month : 24 * 60 * 60 * 1000 * 365/12,
                            day   : 24 * 60 * 60 * 1000,
                            hour  : 60 * 60 * 1000,
                            minute: 60 * 1000,
                            second: 1000
                        }

                        var rtf = new Intl.RelativeTimeFormat('en', { numeric: 'auto' });

                        var getRelativeTime = (d1, d2 = new Date()) => {
                            var elapsed = d1 - d2;

                            // "Math.abs" accounts for both "past" & "future" scenarios
                            for (var u in units) 
                                if (Math.abs(elapsed) > units[u] || u == 'second') 
                                    return rtf.format(Math.round(elapsed/units[u]), u);
                        }

                        return getRelativeTime(d1);
                    }

                    // Don't update HTML if no change
                    let lastUpdate = null;

                    async function fetchPosts() {
                        const response = await fetch('/threads/{{ID}}/feed.json');
                        const body = await response.text();
                        const feed = JSON.parse(body);
                        
                        const refreshed = document.getElementById('refreshedAt');
                        refreshed.innerText = new Date().toLocaleTimeString();
                        document.title = feed.thread.title;

                        if (lastUpdate === body) {
                            return;
                        } else {
                            lastUpdate = body;
                        }

                        document.getElementById('threadTitle').innerText = feed.thread.title;
                        document.getElementById('feed').innerHTML = feed.posts.sort((a, b) => Date.parse(b.created_at) - Date.parse(a.created_at)).map(post => `
                            <div class="card">
                                <div class="created">${relativeTimeAgo(new Date(post.created_at))}, ${new Date(post.created_at).toLocaleString()}</div>
                                <div class="content">${post.content_html}</div>
                                <div class="author">${post.author}</div>
                            </div>
                        `).join('');
                    }

                    fetchPosts();
                    setInterval(fetchPosts, 5000);
                </script>
            
            "#.replace("{{ID}}", &id)))
        })
        .post_async("/post/form_handler", |mut req, ctx| async move {
            let payload = req.form_data().await.expect("no payload");
            
            let supersecure = ctx.env.secret("PASSWORD")?.to_string();
            if payload.get_field("password").expect("no password") != supersecure {
                return Response::error("Unauthorized", 401);
            }

            let d1 = ctx.env.d1("DB")?;
            let now = chrono::offset::Utc::now().to_rfc3339();

            let statement =
                d1.prepare("INSERT INTO posts (thread_id, author, content, created_at) VALUES (?1, ?2, ?3, ?4)");
            let query = statement.bind(&[
                payload.get_field("thread_id").expect("no thread_id").into(),
                payload.get_field("author").expect("no author").into(),
                payload.get_field("content").expect("no content").into(),
                now.into(),
            ])?;

            let result = query.run().await;
            if result.is_ok() {
                Response::ok("post created")
            } else {
                Response::error("Failed to create post", 500)
            }
        })
        .post_async("/thread/form_handler", |mut req, ctx| async move {
            let payload = req.form_data().await?;

            let supersecure = ctx.env.secret("PASSWORD")?.to_string();
            if payload.get_field("password").expect("no password") != supersecure {
                return Response::error("Unauthorized", 401);
            }

            let d1 = ctx.env.d1("DB")?;
            let now = chrono::offset::Utc::now().to_rfc3339();

            let statement =
                d1.prepare("INSERT INTO threads (title, created_at) VALUES (?1, ?2)");
            let query = statement.bind(&[
                payload.get_field("title").expect("no title").into(),
                now.into(),
            ])?;

            let result = query.run().await;
            if result.is_ok() {
                Response::ok("thread created")
            } else {
                Response::error("Failed to create thread", 500)
            }
        })
        
        
        .get_async("/admin_ui/thread/form", |_, _: RouteContext<()>| async move {
            Response::from_html(template(r#"
                <h1>New thread</h1>
                <form action="/thread/form_handler" method="post" style="display: flex; flex-direction: column; gap: 16px; max-width: 800px;">
                    <label>
                        <div>Title</div>
                        <input type="text" name="title" required style="width: 100%;" />
                    </label>

                    <label>
                        <div>Password</div>
                        <input type="password" name="password" required />
                    </label>
                    
                    <button type="submit">Submit</button>
                </form>"#))
        })
        .get_async("/admin_ui/post/form", |_, _| async move {
            Response::from_html(template(r#"
                <script>
                  async function init() {
                    const response = await fetch('/threads.json');
                    let threads = await response.json();
                    threads = [{ id: "", title: '(select a thread)' }, ...threads];
                    const select = document.querySelector('select[name="thread_id"]');
                    threads.forEach(thread => {
                      const option = document.createElement('option');
                      option.value = thread.id;
                      option.innerText = thread.title;
                      select.appendChild(option);
                    });
                    select.disabled = false;
                  }
                  document.addEventListener('DOMContentLoaded', init);
                </script>
                <h1>New post</h1>
                <form action="/post/form_handler" method="post" style="display: flex; flex-direction: column; gap: 16px; max-width: 800px;">
                    <label>
                    <div>Thread</div>
                        <select name="thread_id" disabled>
                        </select>
                    </label>

                    <label>
                    <div>Author</div>
                    <input type="text" name="author" required spellcheck style="width: 100%" placeholder="Ah Beng, crime reporter"/>
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
          "#))
        })
        .run(req, env)
        .await
}
