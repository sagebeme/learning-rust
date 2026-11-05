#!/usr/bin/env bash
# Puts together the course website for GitHub Pages, in the folder given (default _site):
#   site/                the website itself
#   0x*/README.md        each project's lesson, which the website fetches as it is
#   docs/**/*.md         the guides, fetched the same way
#   index.html           sends the address with no path to site/, like netlify.toml and vercel.json do
# Nothing else from the repository is published. Every path the website uses is relative, so it
# works at https://<owner>.github.io/<repo>/ as well as at a domain's root. To try it locally:
#   tools/build-pages.sh /tmp/pages/learning-rust && python3 -m http.server -d /tmp/pages 8000
# then open http://localhost:8000/learning-rust/
set -euo pipefail

cd "$(dirname "$0")/.."
out="${1:-_site}"

rm -rf "$out"
mkdir -p "$out"
cp -R site "$out/site"
for project in 0x*/; do
  mkdir -p "$out/$project"
  cp "$project/README.md" "$out/$project/README.md"
done
find docs -name '*.md' | while read -r guide; do
  mkdir -p "$out/$(dirname "$guide")"
  cp "$guide" "$out/$guide"
done
cat > "$out/index.html" <<'HTML'
<!doctype html>
<meta charset="utf-8">
<title>Learning Rust</title>
<meta http-equiv="refresh" content="0; url=site/">
<link rel="canonical" href="site/">
<p><a href="site/">Learning Rust</a></p>
HTML

echo "Built $out: the website, $(ls -d "$out"/0x*/ | wc -l) projects and $(find "$out/docs" -name '*.md' | wc -l) guides"
