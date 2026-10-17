// The lesson page: a project's README (or a guide from docs/), rendered straight from the repo,
// with the boarding pass, the checklist and the commands around it.
(function () {
  var C = window.RS_COURSE, P = window.RS_PROGRESS;
  var $ = function (id) { return document.getElementById(id); };
  var params = new URLSearchParams(location.search);
  var state = P.load();
  var BLOB = C.repo + "/blob/main/", TREE = C.repo + "/tree/main/";

  var guide = C.guides.find(function (g) { return g.path === params.get("doc"); }) || null;
  var index = C.projects.findIndex(function (p) { return p.dir === params.get("p"); });
  if (!guide && index < 0) index = 0;
  var project = guide ? null : C.projects[index];
  var file = guide ? guide.path : project.dir + "/README.md";
  var folder = file.slice(0, file.lastIndexOf("/") + 1);

  // Extra work that isn't a task: the review (from 0x03) and the "explain it" note.
  function extras(p) {
    var list = [];
    if (p.review) list.push({ id: "review", label: "Wrote my review of <code>review/</code>, then compared it with the answer key" });
    list.push({ id: "explain", label: "Wrote this project's \"explain it\" note for a non-programmer" });
    return list;
  }

  // ----- the boarding pass -----
  if (guide) {
    document.title = guide.title + " · Learning Rust";
    $("title").textContent = guide.title;
    $("pass-kind").textContent = "Information desk";
    $("crumb-week").textContent = "Guides";
    $("crumb-week").href = "index.html#guides";
    $("crumb-here").textContent = guide.title;
    $("seat-label").firstChild.textContent = "Guide";
    $("seat").textContent = "i";
    $("summary").hidden = true;
    $("fields").hidden = true;
    $("check-panel").hidden = true;
    $("cmd-panel").hidden = true;
    $("guide-panel").hidden = false;
    $("guide-links").innerHTML = C.guides.map(function (g) {
      var here = g === guide ? ' aria-current="page" style="font-weight:700"' : "";
      return '<li><a href="' + P.guideUrl(g.path) + '"' + here + ">" + P.esc(g.title) + "</a></li>";
    }).join("") + '<li><a href="index.html#board">All projects →</a></li>';
  } else {
    document.title = project.number + ". " + project.title.replace(/`/g, "") + " · Learning Rust";
    $("title").innerHTML = P.md(project.title);
    $("summary").innerHTML = P.md(project.summary);
    $("crumb-week").textContent = "Week " + project.week;
    $("crumb-here").textContent = project.number;
    $("seat").textContent = project.number;
    renderCommands();
  }

  function renderPass() {
    var c = P.counts(state, project);
    var s = P.status(state, project);
    $("fields").innerHTML =
      "<div><dt>Week</dt><dd>" + project.week + "</dd></div>" +
      "<div><dt>Gate</dt><dd>" + P.esc(project.package) + "</dd></div>" +
      "<div><dt>Tasks</dt><dd>" + c.done + " / " + c.total + "</dd></div>" +
      '<div><dt>Status</dt><dd class="status-text-' + s.key + '">' + s.text + "</dd></div>";
  }

  // ----- the checklist -----
  function renderChecklist() {
    var items = [];
    project.tasks.forEach(function (t) {
      var c = P.checked(state, project.dir, t.id);
      var byCargo = c && c.state === "done";
      var on = P.taskDone(state, project.dir, t.id);
      items.push(
        "<li><label><input type=\"checkbox\" data-id=\"" + t.id + "\"" + (on ? " checked" : "") + (byCargo ? " disabled title=\"Checked by cargo xtask progress\"" : "") + ">" +
        '<span><span class="id">' + t.id + "</span>" + P.md(t.title) +
        '<span class="meta">' + (t.advanced ? '<span class="chip chip-adv">Advanced</span>' : "") + P.chip(c) + "</span></span></label></li>"
      );
    });
    items.push('<li class="heading">Also this week</li>');
    extras(project).forEach(function (x) {
      var on = P.ticked(state, project.dir, x.id);
      items.push('<li><label><input type="checkbox" data-id="' + x.id + '"' + (on ? " checked" : "") + "><span>" + x.label + "</span></label></li>");
    });
    $("checklist").innerHTML = items.join("");

    var all = project.tasks.length + extras(project).length;
    var done = P.counts(state, project).done + extras(project).filter(function (x) { return P.ticked(state, project.dir, x.id); }).length;
    var pct = Math.round((done / all) * 100);
    $("bar").style.width = pct + "%";
    $("meter").setAttribute("aria-valuenow", pct);
    $("pct").textContent = pct + "%";
    $("check-count").textContent = done + " / " + all;
    var c = P.counts(state, project);
    var finished = c.mandatoryDone === c.mandatory;
    $("done-msg").hidden = !finished;
    if (finished) {
      var next = C.projects[index + 1];
      $("done-msg").innerHTML = "Every mandatory task is done. " + (next ? 'Next flight: <a href="' + P.projectUrl(next.dir) + '">' + next.number + " " + P.md(next.title) + "</a>." : "That's the whole course. Time for the client demo.");
    }
    $("report-note").innerHTML = state.report
      ? "Ticks with a coloured label came from <code>cargo xtask progress</code>, " + P.esc(P.when(state)) + "."
      : "Run <code>cargo xtask progress " + project.number + "</code> and the tests tick these for you (see the home page).";
    renderPass();
    markHeadings();
  }

  $("checklist").addEventListener("change", function (e) {
    var id = e.target.getAttribute("data-id");
    if (!id) return;
    P.setTick(state, project.dir, id, e.target.checked);
    renderChecklist();
  });

  function renderCommands() {
    var cmds = [
      "cargo test -p " + project.package,
      "cargo clippy -p " + project.package + " -- -D warnings",
      "cargo xtask progress " + project.number
    ];
    $("cmds").innerHTML = "";
    cmds.forEach(function (text) {
      var row = document.createElement("div");
      row.className = "cmd";
      row.innerHTML = "<code>" + P.esc(text) + '</code><button type="button">Copy</button>';
      P.copyButton(row.querySelector("button"), text);
      $("cmds").appendChild(row);
    });
    var links = ['<li><a href="' + TREE + project.dir + '">This project on GitHub</a></li>'];
    if (project.review) links.push('<li><a href="' + TREE + project.dir + '/review">The pull request to review</a></li>');
    links.push('<li><a href="' + TREE + project.dir + '/solution">The solutions (after you try!)</a></li>');
    $("side-links").innerHTML = links.join("");

    var prev = C.projects[index - 1], next = C.projects[index + 1];
    $("pager").innerHTML =
      (prev ? '<a class="prev" href="' + P.projectUrl(prev.dir) + '"><small>← Previous flight</small><b>' + prev.number + " " + P.md(prev.title) + "</b></a>" : "") +
      (next ? '<a class="next" href="' + P.projectUrl(next.dir) + '"><small>Next flight →</small><b>' + next.number + " " + P.md(next.title) + "</b></a>" : "");
  }

  // ----- the lesson -----
  var lesson = $("lesson");
  fetch("../" + file)
    .then(function (r) {
      if (!r.ok) throw new Error(r.status);
      return r.text();
    })
    .then(function (text) {
      lesson.innerHTML = marked.parse(text);
      enhance();
      if (project) renderChecklist();
      if (location.hash) {
        var target = document.getElementById(decodeURIComponent(location.hash.slice(1)));
        if (target) target.scrollIntoView();
      }
    })
    .catch(function () {
      lesson.innerHTML =
        '<div class="notice"><h2>This lesson couldn\'t load</h2>' +
        "<p>Browsers won't let a page opened straight from your disk read other files. From the course folder, run " +
        "<code>python3 -m http.server 8000</code> and open <code>http://localhost:8000/site/</code>.</p>" +
        '<p>Or read it on GitHub: <a href="' + BLOB + file + '">' + P.esc(file) + "</a></p></div>";
      if (project) renderChecklist();
    });

  // GitHub's heading ids, so links like README.md#using-your-own-engine land in the right place.
  function slug(text) {
    return text.trim().toLowerCase().replace(/[^\p{L}\p{N}\s_-]/gu, "").replace(/\s/g, "-");
  }

  // A link in a README is relative to its folder. Another project or guide opens here; any other
  // file in the repo opens on GitHub.
  function rewrite(a) {
    var href = a.getAttribute("href");
    if (!href || /^(https?:|mailto:|#)/.test(href)) return;
    var url = new URL(href, "https://repo.invalid/" + folder);
    var path = decodeURIComponent(url.pathname.slice(1)).replace(/\/$/, "");
    var hash = url.hash;
    var p = C.projects.find(function (x) { return path === x.dir || path === x.dir + "/README.md"; });
    var g = C.guides.find(function (x) { return path === x.path; });
    if (p) a.href = P.projectUrl(p.dir) + hash;
    else if (g) a.href = P.guideUrl(g.path) + hash;
    else if (path === "" || path === "README.md") a.href = "index.html";
    else a.href = (/\.[a-z0-9]+$/i.test(path.split("/").pop()) ? BLOB : TREE) + path + hash;
  }

  function enhance() {
    // "**Week 1** · …" repeats the boarding pass.
    var first = lesson.querySelector("p");
    if (first && /^Week \d+ ·/.test(first.textContent)) first.className = "week-line";
    var seen = {};
    lesson.querySelectorAll("h2, h3, h4").forEach(function (h) {
      var id = slug(h.textContent);
      if (seen[id] !== undefined) id += "-" + ++seen[id];
      else seen[id] = 0;
      h.id = id;
      var a = document.createElement("a");
      a.className = "anchor";
      a.href = "#" + id;
      a.setAttribute("aria-label", "Link to this section");
      a.textContent = "#";
      h.appendChild(a);
    });
    lesson.querySelectorAll("a[href]").forEach(rewrite);
    lesson.querySelectorAll("table").forEach(function (t) {
      var wrap = document.createElement("div");
      wrap.className = "table-wrap";
      t.parentNode.insertBefore(wrap, t);
      wrap.appendChild(t);
    });
    lesson.querySelectorAll("pre code").forEach(function (code) {
      if (/language-toml/.test(code.className)) code.className = "language-ini";
      if (window.hljs) hljs.highlightElement(code);
      var pre = code.parentNode;
      var b = document.createElement("button");
      b.type = "button";
      b.className = "copy";
      b.textContent = "Copy";
      P.copyButton(b, code.textContent);
      pre.appendChild(b);
    });
  }

  // Each task's heading in the README gets the same label as its checklist line.
  function markHeadings() {
    if (!project) return;
    lesson.querySelectorAll("h3").forEach(function (h) {
      var old = h.querySelector(".task-chip");
      if (old) old.remove();
      var t = project.tasks.find(function (x) {
        return Array.prototype.some.call(h.querySelectorAll("code"), function (c) { return c.textContent === x.file; });
      });
      if (!t) return;
      var span = document.createElement("span");
      span.className = "task-chip";
      var c = P.checked(state, project.dir, t.id);
      span.innerHTML = c ? P.chip(c) : P.taskDone(state, project.dir, t.id) ? '<span class="chip chip-done">Ticked</span>' : "";
      h.insertBefore(span, h.querySelector(".anchor"));
    });
  }

  if (project) {
    renderChecklist();
    P.loadReport(state).then(renderChecklist);
  }
})();
