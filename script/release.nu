let manifest = open Cargo.toml
let version = $manifest.package.version
let name = $manifest.package.name
let tag = $"v($version)"
let bin_dir = $"@bin_path@"
let bin_path = $"($bin_dir)/($name)"

print $"releasing ($name) ($tag)"

if (git status --porcelain | is-not-empty) {
	exit 1
}

cargo publish --dry-run
cargo publish

try {
	gh release create $tag --title $"Release ($tag)" --generate-notes
} catch {
	print "github release failed"
}