use std::ops::Add;

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse},
};

pub async fn wrap_page<B>(req: Request<B>, next: Next<B>) -> Result<impl IntoResponse, StatusCode> {
    // Get response
    let response = next.run(req).await;

    // Extract body
    let (_parts, body) = response.into_parts();
    let body = String::from_utf8(
        hyper::body::to_bytes(body)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .to_vec(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut page = String::new();
    // Add stuff
    page.push_str(
        r#"
<!DOCTYPE html>

<html lang="en">
<head>
<meta charset="UTF-8" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<script
src="https://kit.fontawesome.com/1e87033130.js"
crossorigin="anonymous"
async
></script>

<link rel="stylesheet" href="/css/new.css" />
<link rel="stylesheet" href="/css/header.css" />
<link rel="stylesheet" href="/css/footer.css" />

<title>Page Title</title>
</head>
<body id="home">
  <header>
    <div class="header-wrapper">
      <input type="checkbox" id="nav-toggle" class="nav-toggle" />
      <nav class="nav">
        <ul class="menu-main">
          <li class="main-link">
            <a href="/">Home</a>
            <div class="menu-sub">
              <ul>
                <li>
                  <a href="/#projects">My Projects</a>
                </li>
                <!--<li><a href="/#about">About Me</a></li>-->
                <li><a href="/#resources">Other</a></li>
              </ul>
            </div>
          </li>
          <li class="main-link">
            <a href="/bible/">Bible</a>
            <div class="menu-sub">
              <ul>
                <h3>Old Testament</h3>
                <li>
                  <a href="/bible/Malachi">Malachi</a>
                </li>
              </ul>
              <ul>
                <h3>New Testament</h3>
                <li><a href="/bible/Mark">Mark</a></li>
                <li>
                  <a href="/bible/Colossians"
                  >Colossians</a
                  >
                </li>
                <li>
                  <a href="/bible/Revelation/"
                  >Revelation</a
                  >
                </li>
              </ul>
              <ul>
                <h3>Other Resources</h3>
                <li>
                  <a href="/lectionary">Lectionary</a>
                </li>
                <li>
                  <a href="/songsheet">Song Sheet</a>
                </li>
              </ul>
            </div>
          </li>
          <!--
<li><a href="/songsheet">Song Sheet</a>
<div class="menu-sub">
<ul>
<h3>Download</h3>
<li><a href="/songsheetpdfs/SongSheet.pdf">Song Sheet</a></li>
<li>
<a href="/songsheetpdfs/SongSheetWithChords.pdf">
Song Sheet with chords
</a>
</li>
</ul>
</div>
</li>
-->
          <li class="main-link"><a href="/recipes/">Recipes</a></li>
          <li class="main-link">
            <a href="/#resources">Other</a>
            <div class="menu-sub">
              <ul>
                <li><a href="https://cloud.daniellaing.com">Nextcloud</a></li>
                <li><a href="https://cal.daniellaing.com">Calendar</a></li>
              </ul>
            </div>
          </li>
        </ul>
      </nav>
      <label for="nav-toggle" class="nav-toggle-label">
        <span id="hamburger1"></span>
        <span id="hamburger2"></span>
        <span id="hamburger3"></span>
      </label>
    </div>
  </header>
  <main>
    "#,
    );

    // Add body
    page.push_str(&body);

    // Add footer
    page.push_str(
        r#"
        </main>
        <!--- Footer --->
        <footer>
            <a href="mailto:contact@daniellaing.com" class="footer_email">
                contact@daniellaing.com
            </a>
            <ul class="social-list">
                <li class="social-list_item">
                    <a
                        href="https://github.com/Bodleum"
                        target="_blank"
                        class="social-list_link"
                    >
                        <i class="fa-brands fa-github"></i>
                    </a>
                </li>
                <li class="social-list_item">
                    <a
                        href="https://gitlab.com/Bodleum"
                        target="_blank"
                        class="social-list_link"
                    >
                        <i class="fa-brands fa-gitlab"></i>
                    </a>
                </li>
                <li class="social-list_item">
                    <a
                        href="https://www.instagram.com/_thebakerdan/"
                        target="_blank"
                        class="social-list_link"
                    >
                        <i class="fa-brands fa-instagram"></i>
                    </a>
                </li>
            </ul>
        </footer>
    </body>

</html>"#,
    );

    Ok(Html(page))
}
