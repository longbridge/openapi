from pathlib import Path
from jinja2 import FileSystemLoader, Environment
from semver import VersionInfo

env = Environment(loader=FileSystemLoader("./misc/templates"))
template = env.get_template("docs.html")

root = Path("./gh-pages")

rust_versions = []
python_versions = []
nodejs_versions = []

for item in (root / "rust").iterdir():
    rust_versions.append(item.name)
for item in (root / "python").iterdir():
    python_versions.append(item.name)
for item in (root / "nodejs").iterdir():
    nodejs_versions.append(item.name)

rust_versions.sort(key=lambda x: VersionInfo.parse(x), reverse=True)
python_versions.sort(key=lambda x: VersionInfo.parse(x), reverse=True)

result = template.render(
    rust_versions=rust_versions,
    python_versions=python_versions,
    nodejs_versions=nodejs_versions,
)
print(result)
