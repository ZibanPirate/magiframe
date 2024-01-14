module.exports = {
    "*.ts": () => "@biomejs/biome format --write",
    "*.md": "prettier --write",
    "*.json": "prettier --write",
    "package.json": ["syncpack format", "prettier --write"],
};
