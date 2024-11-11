# cardano-dev-snippet 

## 🚧 Under Construction 🚧

## Introduction

Snippets for Cardano Development

## Contribution

Add or modify the JSON files in the `snippets` folder. The file name should be the language identifier. For example, `mesh-react-hooks.json` for hooks in `@meshsdk/react`.

## Publishing the Extension
Before publishing, make sure you have the Visual Studio Code Extension Manager installed:
```bash
npm install -g @vscode/vsce
```

To generate the `.vsix` file:
```bash
vsce package
```
* You can install the extension from the `.vsix` file to test it locally.

To publish the extension to VS Code Marketplace:
```bash
vsce publish
```