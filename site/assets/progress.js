// Progress, shared by every page. Two sources, both kept only in this browser:
// - ticks you set yourself: { "0x01-values-control": { "t00": true, "review": true } }
// - the last `cargo xtask progress` report (progress.json): what the tests and clippy say.
// A task counts as done if either says so. No accounts, no server.
(function () {
  var KEY = "learning-rust-progress";
  var C = window.RS_COURSE;

  function load() {
    try {
      var state = JSON.parse(localStorage.getItem(KEY) || "{}");
      if (!state || typeof state !== "object") state = {};
      if (!state.ticks || typeof state.ticks !== "object") state.ticks = {};
      return state;
    } catch (e) {
      return { ticks: {} };
    }
  }
  function save(state) {
    try { localStorage.setItem(KEY, JSON.stringify(state)); } catch (e) { /* private window: progress just isn't kept */ }
  }

  function isReport(data) {
    return data && typeof data === "object" && data.projects && typeof data.projects === "object";
  }

  // When the site is served from the course folder, `cargo xtask progress` has left progress.json
  // one level up. On a hosted copy there's none, and the last import is used instead.
  function loadReport(state) {
    return fetch("../progress.json", { cache: "no-store" })
      .then(function (r) { return r.ok ? r.json() : null; })
      .catch(function () { return null; })
      .then(function (data) {
        if (isReport(data)) {
          state.report = data;
          state.reportFrom = "terminal";
          save(state);
        }
        return state;
      });
  }

  function importReport(state, file) {
    return file.text().then(function (text) {
      var data = JSON.parse(text);
      if (!isReport(data)) throw new Error("not a progress.json");
      state.report = data;
      state.reportFrom = "file";
      save(state);
      return state;
    });
  }

  // What `cargo xtask progress` said about one task, or null.
  function checked(state, dir, id) {
    var p = state.report && state.report.projects && state.report.projects[dir];
    return (p && p.tasks && p.tasks[id]) || null;
  }
  function ticked(state, dir, id) {
    return !!(state.ticks[dir] && state.ticks[dir][id]);
  }
  function setTick(state, dir, id, on) {
    state.ticks[dir] = state.ticks[dir] || {};
    if (on) state.ticks[dir][id] = true;
    else delete state.ticks[dir][id];
    save(state);
  }
  function taskDone(state, dir, id) {
    var c = checked(state, dir, id);
    return (c && c.state === "done") || ticked(state, dir, id);
  }

  function counts(state, project) {
    var mandatory = project.tasks.filter(function (t) { return !t.advanced; });
    var done = project.tasks.filter(function (t) { return taskDone(state, project.dir, t.id); });
    var mandatoryDone = mandatory.filter(function (t) { return taskDone(state, project.dir, t.id); });
    return { done: done.length, total: project.tasks.length, mandatory: mandatory.length, mandatoryDone: mandatoryDone.length };
  }

  // A project's line on the departures board.
  function status(state, project) {
    var c = counts(state, project);
    var p = state.report && state.report.projects && state.report.projects[project.dir];
    if (c.mandatory && c.mandatoryDone === c.mandatory) return { key: "departed", text: "Departed" };
    if (p && p.compiles === false) return { key: "delayed", text: "Delayed" };
    var failing = project.tasks.some(function (t) {
      var x = checked(state, project.dir, t.id);
      return x && x.state === "failing";
    });
    if (c.done > 0 || failing) return { key: "boarding", text: "Boarding" };
    return { key: "scheduled", text: "Scheduled" };
  }

  // The first mandatory task not done, in course order. Null once everything is.
  function nextTask(state) {
    for (var i = 0; i < C.projects.length; i++) {
      var p = C.projects[i];
      for (var j = 0; j < p.tasks.length; j++) {
        var t = p.tasks[j];
        if (!t.advanced && !taskDone(state, p.dir, t.id)) return { project: p, task: t };
      }
    }
    return null;
  }

  var STATE_WORDS = {
    done: "Tests pass",
    lint: "Clippy warnings",
    failing: "Tests failing",
    todo: "To do",
    "needs-db": "Needs Postgres",
    broken: "Doesn't compile"
  };
  function chip(c) {
    if (!c) return "";
    var detail = c.state === "failing" ? " " + c.passed + "/" + c.tests : c.state === "lint" ? " (" + c.clippy + ")" : "";
    return '<span class="chip chip-' + c.state + '" title="What cargo xtask progress found">cargo: ' + STATE_WORDS[c.state] + detail + "</span>";
  }

  function when(state) {
    if (!state.report || !state.report.checked) return "";
    var d = new Date(state.report.checked * 1000);
    return d.toLocaleString(undefined, { weekday: "short", hour: "2-digit", minute: "2-digit", day: "numeric", month: "short" });
  }

  function projectUrl(dir) { return "lesson.html?p=" + encodeURIComponent(dir); }
  function guideUrl(path) { return "lesson.html?doc=" + encodeURIComponent(path); }

  // Escape text before it goes into innerHTML; `code` in titles becomes <code>.
  function esc(text) {
    return String(text).replace(/[&<>"']/g, function (c) {
      return { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c];
    });
  }
  function md(text) { return esc(text).replace(/`([^`]+)`/g, "<code>$1</code>"); }

  function copyButton(button, text) {
    button.addEventListener("click", function () {
      var done = function () {
        button.textContent = "Copied";
        setTimeout(function () { button.textContent = "Copy"; }, 1400);
      };
      if (navigator.clipboard) navigator.clipboard.writeText(text).then(done, function () {});
    });
  }

  // The square "back to top" sign, on every page.
  function backToTop() {
    var a = document.querySelector(".to-top");
    if (!a) return;
    var update = function () { a.classList.toggle("show", window.scrollY > 700); };
    window.addEventListener("scroll", update, { passive: true });
    update();
  }
  document.addEventListener("DOMContentLoaded", backToTop);

  window.RS_PROGRESS = {
    load: load, save: save, loadReport: loadReport, importReport: importReport, checked: checked,
    ticked: ticked, setTick: setTick, taskDone: taskDone, counts: counts, status: status,
    nextTask: nextTask, chip: chip, when: when, projectUrl: projectUrl, guideUrl: guideUrl,
    esc: esc, md: md, copyButton: copyButton
  };
})();
