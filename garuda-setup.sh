#!/bin/bash -v

# ╭──────────────────────────────────╮
# │ Independent, system level things │
# ╰──────────────────────────────────╯
cargo run -- install --using pacman packages/git.toml
cargo run -- install --using pacman packages/rofi.toml
cargo run -- install --using pacman packages/ripgrep.toml
cargo run -- install --using pacman packages/1password.toml
cargo run -- install --using paru packages/chruby-fish.toml
cargo run -- install --using pacman packages/eza.toml
cargo run -- install --using pacman packages/fastfetch.toml
cargo run -- install --using pacman packages/kanshi.toml
cargo run -- install --using pacman packages/mcfly.toml
cargo run -- install --using pacman packages/starship.toml
cargo run -- install --using pacman packages/topgrade.toml

cargo run -- install --using pacman packages/fzf.toml
cargo run -- install --using pacman packages/gh.toml
cargo run -- install --using pacman packages/git-delta.toml
cargo run -- install --using pacman packages/jq.toml
cargo run -- install --using pacman packages/just.toml
cargo run -- install --using pacman packages/tokei.toml

# ╭──────────────────╮
# │ Package Managers │
# ╰──────────────────╯
cargo run -- install --using pacman packages/curl.toml
cargo run -- install --using curl packages/cargo-binstall.toml
cargo run -- install --using pacman packages/fnm.toml
cargo run -- install --using paru packages/ruby-install.toml
cargo run -- install --using pacman packages/rye.toml

# ╭─────────────╮
# │ Other tools │
# ╰─────────────╯
cargo run -- install --using pacman packages/fzf.toml
cargo run -- install --using pacman packages/gh.toml
cargo run -- install --using pacman packages/git-delta.toml
cargo run -- install --using pacman packages/jq.toml
cargo run -- install --using pacman packages/just.toml
cargo run -- install --using pacman packages/tokei.toml

# ╭───────────╮
# │ Dev tools │
# ╰───────────╯
cargo run -- install --using pacman packages/bacon.toml
cargo run -- install --using npm packages/bash-language-server.toml
cargo run -- install --using pacman packages/bat.toml
cargo run -- install --using cargo packages/cargo-outdated.toml
cargo run -- install --using cargo packages/cargo-watch.toml
cargo run -- install --using pacman packages/clang-format.toml
cargo run -- install --using pacman packages/clangd.toml
cargo run -- install --using rye packages/cmake-language-server.toml
cargo run -- install --using pacman packages/deno.toml
cargo run -- install --using npm packages/docker-langserver.toml
cargo run -- install --using pacman packages/dua-cli.toml
cargo run -- install --using paru packages/efm-langserver.toml
cargo run -- install --using npm packages/emmet-ls.toml
cargo run -- install --using pacman packages/lazygit.toml
cargo run -- install --using pacman packages/lua-language-server.toml
cargo run -- install --using pacman packages/marksman.toml
cargo run -- install --using npm packages/prettier.toml
cargo run -- install --using cargo packages/prosemd-lsp.toml
cargo run -- install --using rye packages/pyright-langserver.toml
cargo run -- install --using rye packages/ruff-lsp.toml
cargo run -- install --using pacman packages/shellcheck.toml
cargo run -- install --using gem packages/solargraph.toml
cargo run -- install --using pacman packages/taplo.toml
cargo run -- install --using pacman packages/tmux.toml
cargo run -- install --using pacman packages/tree-sitter.toml
cargo run -- install --using npm packages/typescript-language-server.toml
cargo run -- install --using pacman packages/typos.toml
cargo run -- install --using paru packages/vale.toml
cargo run -- install --using npm packages/vim-language-server.toml
cargo run -- install --using npm packages/yaml-language-server.toml
cargo run -- install --using rye packages/yamllint.toml

cargo run -- install --using pacman packages/nvim.toml
