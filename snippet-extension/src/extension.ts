import * as vscode from "vscode";
import { scriptTypeMap } from "./data/typeMap";

export function activate(context: vscode.ExtensionContext) {
  console.log("Cardano devkit extension activated");

  const analyzeScriptJSON = vscode.commands.registerCommand(
    "cardano-extension.analyzeScriptJSON",
    () => {
      vscode.window.showInformationMessage(
        "Analyzing Cardano script file for JSON date type..."
      );
      const options: vscode.OpenDialogOptions = {
        canSelectMany: false,
        canSelectFolders: false,
        openLabel: "Open",
        filters: {
          JSON: ["json"],
        },
      };

      vscode.window.showOpenDialog(options).then((fileUri) => {
        if (fileUri && fileUri[0]) {
          const fs = require("fs");
          const path = fileUri[0].fsPath;
          const data = fs.readFileSync(path, "utf8");

          const script = JSON.parse(data);
          const definitions = script.definitions;

          let imports: string[] = []; // Types that needs to be imported
          let customs: string[] = []; // Custom types that imports are not needed

          let definitionTypes: string[] = []; // Definition types that imports are not needed
          let fullSnippet: string[] = [];

          if (definitions) {
            // Reverse the definition list to handle custom types first
            Object.keys(definitions)
              .reverse()
              .forEach((key) => {
                const title = definitions[key]?.title;
                const anyOf = definitions[key]?.anyOf;

                if (title) {
                  // Skip if already included in the blueprint
                  if (imports.includes(title) || customs.includes(title)) {
                    return;
                  }

                  // Add to imports if it is not a custom type and title only contains letters
                  const regex = /^[a-zA-Z]+$/;

                  if (
                    !imports.includes(title) &&
                    !key.includes("/") &&
                    regex.test(title)
                  ) {
                    imports.push(title);
                  } else {
                    customs.push(title);
                  }

                  if (anyOf) {
                    let types: string[] = [];
                    let definitionSnippet: string[] = [];

                    anyOf.forEach((type: any) => {
                      if (!type.title) {
                        return;
                      }

                      types.push(type.title);

                      if (definitionTypes.includes(type.title)) {
                        return;
                      }

                      definitionTypes.push(type.title);

                      if (type.fields && type.fields.length > 0) {
                        const fields = type.fields;

                        let fieldSnippet = [
                          `export type ${type.title} = ConStr0<[`,
                        ];

                        fields.forEach((field: any) => {
                          const ref = field.$ref.split("/");
                          let type = ref[ref.length - 1];
                          if (type.includes("~1")) {
                            const split = type.split("~1");
                            type = split[split.length - 1];
                          }

                          let mappedType = type;

                          if (type.includes("Tuple")) {
                            const tuple = type.split("Tuple$");
                            const tupleTypes = tuple[1].split("_");

                            mappedType = `Tuple<${tupleTypes.join(", ")}>, // ${
                              field.title
                            }`;
                          } else {
                            for (const key in scriptTypeMap) {
                              if (type.includes(key)) {
                                mappedType =
                                  scriptTypeMap[
                                    key as keyof typeof scriptTypeMap
                                  ];
                                break;
                              }
                            }
                          }

                          if (
                            !imports.includes(mappedType) &&
                            regex.test(mappedType)
                          ) {
                            imports.push(mappedType);
                          }

                          fieldSnippet.push(
                            `${mappedType}, // ${field.title}: ${type}`
                          );
                        });

                        fieldSnippet.push(`]>;`);

                        definitionSnippet = [
                          ...definitionSnippet,
                          ...fieldSnippet,
                        ];
                      }
                    });

                    if (anyOf.length > 1) {
                      definitionSnippet = [
                        `export type ${title} = ConStr0<${types.join(" | ")}>;`,
                        "",
                        ...definitionSnippet,
                      ];
                    }

                    if (definitionSnippet.length > 0) {
                      fullSnippet = [...fullSnippet, "", ...definitionSnippet];
                    }
                  }
                }
              });

            if (imports.length > 0) {
              fullSnippet = [
                `import { ConStr0, ${imports.join(
                  ", "
                )}} from "@meshsdk/core";`,
                "",
                ...fullSnippet,
              ];
            }

            const snippet = new vscode.SnippetString(fullSnippet.join("\n"));
            vscode.window.activeTextEditor?.insertSnippet(snippet);
          }
        }
      });
    }
  );

  context.subscriptions.push(analyzeScriptJSON);

  const analyzeScriptMesh = vscode.commands.registerCommand(
    "cardano-extension.analyzeScriptMesh",
    () => {
      vscode.window.showInformationMessage(
        "Analyzing Cardano script file for Mesh data type..."
      );
      const options: vscode.OpenDialogOptions = {
        canSelectMany: false,
        canSelectFolders: false,
        openLabel: "Open",
        filters: {
          JSON: ["json"],
        },
      };

      vscode.window.showOpenDialog(options).then((fileUri) => {
        if (fileUri && fileUri[0]) {
          const fs = require("fs");
          const path = fileUri[0].fsPath;
          const data = fs.readFileSync(path, "utf8");

          const script = JSON.parse(data);
          const definitions = script.definitions;

          let imports: string[] = []; // Types that needs to be imported
          let customs: string[] = []; // Custom types that imports are not needed

          let definitionTypes: string[] = []; // Definition types that imports are not needed
          let fullSnippet: string[] = [];

          if (definitions) {
            // Reverse the definition list to handle custom types first
            Object.keys(definitions)
              .reverse()
              .forEach((key) => {
                const title = definitions[key]?.title;
                const anyOf = definitions[key]?.anyOf;

                if (title) {
                  // Skip if already included in the blueprint
                  if (imports.includes(title) || customs.includes(title)) {
                    return;
                  }

                  // Add to imports if it is not a custom type and title only contains letters
                  const regex = /^[a-zA-Z]+$/;

                  if (
                    !imports.includes(title) &&
                    !key.includes("/") &&
                    regex.test(title)
                  ) {
                    imports.push(title);
                  } else {
                    customs.push(title);
                  }

                  if (anyOf) {
                    let types: string[] = [];
                    let definitionSnippet: string[] = [];

                    anyOf.forEach((type: any) => {
                      if (!type.title) {
                        return;
                      }

                      types.push(type.title);

                      if (definitionTypes.includes(type.title)) {
                        return;
                      }

                      definitionTypes.push(type.title);

                      if (type.fields && type.fields.length > 0) {
                        const fields = type.fields;

                        let fieldSnippet = [
                          `export type ${type.title} = ConStr0<[`,
                        ];

                        fields.forEach((field: any) => {
                          const ref = field.$ref.split("/");
                          let type = ref[ref.length - 1];
                          if (type.includes("~1")) {
                            const split = type.split("~1");
                            type = split[split.length - 1];
                          }

                          let mappedType = type;

                          if (type.includes("Tuple")) {
                            const tuple = type.split("Tuple$");
                            const tupleTypes = tuple[1].split("_");

                            mappedType = `Tuple<${tupleTypes.join(", ")}>, // ${
                              field.title
                            }`;
                          } else {
                            for (const key in scriptTypeMap) {
                              if (type.includes(key)) {
                                mappedType =
                                  scriptTypeMap[
                                    key as keyof typeof scriptTypeMap
                                  ];
                                break;
                              }
                            }
                          }

                          if (
                            !imports.includes(mappedType) &&
                            regex.test(mappedType)
                          ) {
                            imports.push(mappedType);
                          }

                          fieldSnippet.push(
                            `${mappedType}, // ${field.title}: ${type}`
                          );
                        });

                        fieldSnippet.push(`]>;`);

                        definitionSnippet = [
                          ...definitionSnippet,
                          ...fieldSnippet,
                        ];
                      }
                    });

                    if (anyOf.length > 1) {
                      definitionSnippet = [
                        `export type ${title} = ConStr0<${types.join(" | ")}>;`,
                        "",
                        ...definitionSnippet,
                      ];
                    }

                    if (definitionSnippet.length > 0) {
                      fullSnippet = [...fullSnippet, "", ...definitionSnippet];
                    }
                  }
                }
              });

            if (imports.length > 0) {
              fullSnippet = [
                `import { ConStr0, ${imports.join(
                  ", "
                )}} from "@meshsdk/core";`,
                "",
                ...fullSnippet,
              ];
            }

            const snippet = new vscode.SnippetString(fullSnippet.join("\n"));
            vscode.window.activeTextEditor?.insertSnippet(snippet);
          }
        }
      });
    }
  );

  context.subscriptions.push(analyzeScriptMesh);
}

// This method is called when your extension is deactivated
export function deactivate() {}
