# Changelog

## [0.1.1](https://github.com/nvim-neorg/neorg-dirman/compare/v0.1.0...v0.1.1) (2023-03-12)


### Bug Fixes

* `destroy_files` function memory leak ([#4](https://github.com/nvim-neorg/neorg-dirman/issues/4)) ([92f028f](https://github.com/nvim-neorg/neorg-dirman/commit/92f028fa6d8b57c35cda5e61f37869b7bd8477fe))
* **ci:** check out the repository before doing anything else ([9196b59](https://github.com/nvim-neorg/neorg-dirman/commit/9196b597d6b162c51adc9cc84bb7015a216c8b81))

## 0.1.0 (2023-03-11)


### Features

* add `workspace_files` C function ([e437c9f](https://github.com/nvim-neorg/neorg-dirman/commit/e437c9f9a73ab51be068ec0749f8ea6dbe70c97b))
* add ffi.lua for debugging ([482b044](https://github.com/nvim-neorg/neorg-dirman/commit/482b044463f704071eff34fb8f6a82ef5cac1c25))
* add rust.yml workflow ([46b67d7](https://github.com/nvim-neorg/neorg-dirman/commit/46b67d7d663b244bafca5c75ff1b3449add75168))
* create initial C bindings ([9fd44ae](https://github.com/nvim-neorg/neorg-dirman/commit/9fd44aeffccc7e6b1b0c399bb85cfa64b8ccb6dd))
* enhance release profile to generate tiny library, add `release_nostrip` variant ([b0774ff](https://github.com/nvim-neorg/neorg-dirman/commit/b0774ff2d1051296ee357597c6d9868d772fdbc5))
* give `Workspace`s the `files()` function ([352039b](https://github.com/nvim-neorg/neorg-dirman/commit/352039ba9ad55b0a9b9284c72ba2f0d09469bb55))
* initial commit ([d12932a](https://github.com/nvim-neorg/neorg-dirman/commit/d12932a24e00d11486eb941e9b136bd7f686ee60))


### Bug Fixes

* `crate-type` under wrong section ([03d1b00](https://github.com/nvim-neorg/neorg-dirman/commit/03d1b002988a7fb44d409b38bec46ab06ea5d606))
* broken c interop ([#2](https://github.com/nvim-neorg/neorg-dirman/issues/2)) ([674c47a](https://github.com/nvim-neorg/neorg-dirman/commit/674c47a06f4fde5f1b13c3c746be3ba9d158b926))
* **tests:** `files()` function of traverse workspace ([#1](https://github.com/nvim-neorg/neorg-dirman/issues/1)) ([92d6639](https://github.com/nvim-neorg/neorg-dirman/commit/92d6639baafce9efe3d9ff4b28ad0d0483d366d8))
* use `c_char` instead of regular `char` ([0dedd3e](https://github.com/nvim-neorg/neorg-dirman/commit/0dedd3e33807c3645ef42a1ef5da31fc1a676f15))
