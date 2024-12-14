# Cardano DevKit 🪚🔩⛓

<p align="left">
<img alt="Tests" src="https://github.com/cardano-foundation/cardano-devkit/actions/workflows/test.yml/badge.svg?branch=main" />
<img alt="Release" src="https://github.com/cardano-foundation/cardano-devkit/actions/workflows/release.yml/badge.svg?branch=main" />
<img alt="Publish" src="https://github.com/cardano-foundation/cardano-devkit/actions/workflows/publish.yml/badge.svg?branch=main" />
<a href="https://conventionalcommits.org"><img alt="conventionalcommits" src="https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits" /></a>
<a href="https://opensource.org/licenses/MIT"><img alt="License" src="https://img.shields.io/badge/License-MIT-green.svg" /></a>
<a href="https://discord.gg/4WVNHgQ7bP"><img alt="Discord" src="https://img.shields.io/discord/1022471509173882950?label=Discord"></a>
</p>

Welcome to **Cardano DevKit!** This project is designed to be the go-to toolkit for setting up a local Cardano network. Whether the goal is to test, showcase, scaffold, or template, Cardano DevKit bundles a variety of community tools, based on the powerful [Yaci DevKit](https://github.com/bloxbean/yaci-devkit), to streamline and support the development process.

About the product: [product specs](PRODUCT.md)

## 🚧 Under Construction 🚧

Please note that **Cardano DevKit** is currently under construction. We are actively working on adding more features and improving the existing ones. Stay tuned for updates! 

## 🚀 Getting Started

To run the tauri app locally, please clone the repository and run the following commands:

```
npm i
npm run tauri dev
```

If you want to run cardano-devkit cli commands locally, you can run the following commands:

```
cd system
cargo run -- help
```

## 📦 Building the App

To build the tauri app, you can run the following command:

```
npm run bundle
```

This will create a platform-specific exectuable. This executable is able to run as an application with ui, or as a cli tool.

## Contributing 🤝

All contributions are welcome! Please feel free to open a new thread on the issue tracker or submit a new pull request.

Please read [Contributing](CONTRIBUTING.md) in advance. Thank you for contributing! 💙

## 📓 Additional Documents
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Security](SECURITY.md)

## License 📄

This project is licensed under the Apache 2.0 License. See the [`LICENSE`](LICENSE) file for more details.

*Happy Building!* 🛠️