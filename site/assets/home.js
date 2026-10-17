// The home page: the next boarding pass, the progress strip, the departures board and the guides.
(function () {
  var C = window.RS_COURSE, P = window.RS_PROGRESS;
  var $ = function (id) { return document.getElementById(id); };
  var state = P.load();

  var totalTasks = C.projects.reduce(function (n, p) { return n + p.tasks.length; }, 0);
  $("counts").textContent = C.weeks.length + " weeks · " + C.projects.length + " projects · " + totalTasks + " tasks";

  // ----- the flight plan: one line per week -----
  $("timeline").innerHTML = C.weeks.map(function (w) {
    var flights = C.projects.filter(function (p) { return p.week === w.n; }).map(function (p) { return p.number; });
    return "<li><b>Week " + w.n + "</b><span>" + P.esc(w.theme) + ' <span style="color:var(--muted)">· ' + flights.join(", ") + "</span></span></li>";
  }).join("");

  // ----- the guides -----
  $("guide-list").innerHTML = C.guides.map(function (g) {
    var sign = /decisions\//.test(g.path) ? "ADR" : /templates\//.test(g.path) ? "T" : /week-1/.test(g.path) ? "W1" : "i";
    return '<li><a href="' + P.guideUrl(g.path) + '"><span class="sign" aria-hidden="true">' + sign + "</span>" + P.esc(g.title) + "</a></li>";
  }).join("");

  // ----- serving the course locally -----
  $("serve-cmds").innerHTML = "";
  ["cargo xtask progress", "python3 -m http.server 8000"].forEach(function (text) {
    var row = document.createElement("div");
    row.className = "cmd";
    row.innerHTML = "<code>" + P.esc(text) + '</code><button type="button">Copy</button>';
    P.copyButton(row.querySelector("button"), text);
    $("serve-cmds").appendChild(row);
  });

  // ----- the Nairobi clock -----
  function tick() {
    try {
      $("clock").textContent = new Date().toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit", timeZone: "Africa/Nairobi" });
    } catch (e) {
      $("clock").textContent = "";
    }
  }
  tick();
  setInterval(tick, 15000);

  function render() {
    // the board
    var rows = [];
    C.weeks.forEach(function (w) {
      rows.push('<tr class="week-row"><td colspan="5">Week ' + w.n + "<span>" + P.esc(w.theme) + "</span></td></tr>");
      C.projects.filter(function (p) { return p.week === w.n; }).forEach(function (p, i) {
        var c = P.counts(state, p);
        var s = P.status(state, p);
        var flaps = p.number.split("").map(function (ch) { return '<span class="flap">' + P.esc(ch) + "</span>"; }).join("");
        rows.push(
          '<tr class="flight-row" style="animation-delay:' + (i * 60) + 'ms">' +
          '<td><a href="' + P.projectUrl(p.dir) + '" tabindex="-1" aria-hidden="true"><span class="flight">' + flaps + "</span></a></td>" +
          '<td><a href="' + P.projectUrl(p.dir) + '"><span class="dest">' + P.md(p.title) + '</span></a><span class="sr-only"> (' + p.number + ")</span></td>" +
          '<td class="col-gate"><span class="gate">' + P.esc(p.package) + "</span></td>" +
          '<td class="col-tasks"><span class="tasks">' + c.done + " / " + c.total + "</span></td>" +
          '<td><span class="status status-' + s.key + '">' + s.text + "</span></td></tr>"
        );
      });
    });
    $("flights").innerHTML = rows.join("");

    // the strip
    var m = 0, md = 0, a = 0, ad = 0, departed = 0;
    C.projects.forEach(function (p) {
      var c = P.counts(state, p);
      m += c.mandatory; md += c.mandatoryDone;
      a += c.total - c.mandatory; ad += c.done - c.mandatoryDone;
      if (P.status(state, p).key === "departed") departed++;
    });
    $("stat-mandatory").textContent = md + " / " + m;
    $("stat-advanced").textContent = ad + " / " + a;
    $("stat-projects").textContent = departed + " / " + C.projects.length;
    var pct = m ? Math.round((md / m) * 100) : 0;
    $("bar").style.width = pct + "%";
    $("meter").setAttribute("aria-valuenow", pct);
    if (state.report) {
      $("source").innerHTML = (state.reportFrom === "terminal" ? "Checked by <code>cargo xtask progress</code>, " : "From the progress.json you loaded, checked ") +
        P.esc(P.when(state)) + ". Your own ticks count too.";
    }

    // the next boarding pass
    var next = P.nextTask(state);
    var anyDone = C.projects.some(function (p) { return P.counts(state, p).done > 0; });
    if (next) {
      var url = P.projectUrl(next.project.dir);
      $("next-pass").href = url;
      $("next-title").innerHTML = P.md(next.project.title);
      $("next-flight").textContent = next.project.number;
      $("next-week").textContent = next.project.week;
      $("next-task").textContent = next.task.id;
      $("next-gate").textContent = next.project.number;
      $("cta").href = url;
      if (anyDone) {
        $("cta").textContent = "Continue with " + next.project.number + " →";
        $("resume").textContent = "Welcome back. " + md + " of " + m + " mandatory tasks done. Up next: " + next.project.number + " " + next.task.id + ", " + next.task.title.replace(/`/g, "") + ".";
        $("resume").hidden = false;
      }
    } else {
      $("next-title").textContent = "All flights departed";
      $("next-task").textContent = "Demo";
      $("resume").textContent = "Every mandatory task is done. Time for the client demo in 0x0E.";
      $("resume").hidden = false;
    }
  }

  render();
  P.loadReport(state).then(render);

  $("import").addEventListener("change", function (e) {
    var file = e.target.files && e.target.files[0];
    if (!file) return;
    P.importReport(state, file).then(render, function () {
      $("source").textContent = "That file isn't a progress.json from cargo xtask progress.";
    });
    e.target.value = "";
  });
})();
