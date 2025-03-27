# Cardano Devkit VSCode Extension

## 🚧 Under Construction 🚧

## Introduction

VSCode extension for Cardano Development

## Contribution

Add or modify the JSON files in the `snippets` folder. The file name should be the language identifier. For example, `mesh-react-hooks.json` for hooks in `@meshsdk/react`.

## Pre-requisites

Before publishing, make sure you have the Visual Studio Code Extension Manager installed:

```sh
npm install -g @vscode/vsce
```

## Test locally

1. Go to the this folder:

```sh
cd vscode-extension
```

2. To generate the `.vsix` file:

```sh
vsce package
```

3. Open the Extensions view by clicking on the square icon in the Sidebar on the Activity Bar, or use the `Ctrl+Shift+X` shortcut. Install the extension by clicking on the `...` icon and selecting `Install from VSIX...`. Select `cardano-devkit-vscode-X.X.X.vsix` file.


## Publish


1. Go to the this folder:

```sh
cd vscode-extension
```

2. To generate the `.vsix` file:

```sh
vsce package
```

3. To publish the extension to VS Code Marketplace:

```sh
vsce publish
```
