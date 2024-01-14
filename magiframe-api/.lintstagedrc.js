module.exports = {
    "*.rs": (filenames) => [
        "cargo clippy --fix --allow-staged --allow-dirty",
        "cargo clippy -- -W clippy::unwrap_used -D warnings",
        `cargo fmt -- ${filenames.join(" ")}`,
    ],
};
