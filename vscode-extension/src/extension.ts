import * as vscode from "vscode";
import { jsonTypeMap, basicTypeList, meshTypeMap } from "./data/typeMap";

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
                  // Skip if it is basic Cardano type
                  if (basicTypeList.includes(title)) {
                    return;
                  }

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

                    anyOf.forEach((type: any, index: number) => {
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

                        if (!imports.includes(`ConStr${index}`)) {
                          imports.push(`ConStr${index}`);
                        }

                        let fieldSnippet = [
                          `export type ${type.title} = ConStr${index}<[`,
                        ];

                        fields.forEach((field: any, index: number) => {
                          const ref = field.$ref.split("/");
                          let fieldType = ref[ref.length - 1];
                          if (fieldType.includes("~1")) {
                            const split = fieldType.split("~1");
                            fieldType = split[split.length - 1];
                          }

                          let mappedType = fieldType;

                          if (fieldType.includes("Tuple")) {
                            const tuple = fieldType.split("Tuple$");
                            const tupleTypes = tuple[1].split("_");

                            mappedType = `Tuple<${tupleTypes.join(", ")}>, // ${
                              field.title
                            }`;
                          } else {
                            for (const key in jsonTypeMap) {
                              if (mappedType.includes(key)) {
                                mappedType =
                                  jsonTypeMap[key as keyof typeof jsonTypeMap];
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
                            `${mappedType}, // ${field.title}: ${fieldType}`
                          );
                        });

                        fieldSnippet.push(`]>;`);

                        definitionSnippet = [
                          ...definitionSnippet,
                          ...fieldSnippet,
                        ];
                      }
                    });

                    if (definitionSnippet.length > 0) {
                      if (anyOf.length > 1) {
                        definitionSnippet = [
                          `export type ${title} = ${types.join(" | ")};`,
                          "",
                          ...definitionSnippet,
                        ];
                      }

                      fullSnippet = [...fullSnippet, "", ...definitionSnippet];
                    }
                  }
                }
              });

            if (imports.length > 0) {
              fullSnippet = [
                `import { ${imports.join(", ")} } from "@meshsdk/core";`,
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
                  // Skip if it is basic Cardano type
                  if (basicTypeList.includes(title)) {
                    return;
                  }

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
                    let MTypes: string[] = [];
                    let definitionSnippet: string[] = [];

                    anyOf.forEach((type: any, index: number) => {
                      MTypes.push(`M${type.title}`);

                      if (definitionTypes.includes(type.title)) {
                        return;
                      }

                      definitionTypes.push(type.title);

                      if (type.fields && type.fields.length > 0) {
                        const fields = type.fields;

                        if (!imports.includes(`MConStr${index}`)) {
                          imports.push(`MConStr${index}`);
                        }

                        let fieldSnippet = [
                          `export type M${type.title} = MConStr${index}<[`,
                        ];

                        fields.forEach((field: any) => {
                          const ref = field.$ref.split("/");
                          let fieldType = ref[ref.length - 1];
                          if (fieldType.includes("~1")) {
                            const split = fieldType.split("~1");
                            fieldType = split[split.length - 1];
                          }

                          let mappedType =
                            String(fieldType).charAt(0).toLowerCase() +
                            String(fieldType).slice(1);

                          if (fieldType.includes("Tuple")) {
                            const tuple = fieldType.split("Tuple$");
                            const tupleTypes = tuple[1].split("_");

                            mappedType = `MTuple<${tupleTypes.join(
                              ", "
                            )}>, // ${field.title}`;

                            if (!imports.includes("MTuple")) {
                              imports.push("MTuple");
                            }
                          } else {
                            for (const key in meshTypeMap) {
                              if (mappedType.toLowerCase().includes(key)) {
                                mappedType =
                                  meshTypeMap[key as keyof typeof meshTypeMap];
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
                            `${mappedType}, // ${field.title}: ${fieldType}`
                          );
                        });

                        fieldSnippet.push(`]>;`);

                        definitionSnippet = [
                          ...definitionSnippet,
                          ...fieldSnippet,
                        ];
                      }
                    });

                    if (definitionSnippet.length > 0) {
                      if (anyOf.length > 1) {
                        definitionSnippet = [
                          `export type M${title} = ${MTypes.join(" | ")};`,
                          "",
                          ...definitionSnippet,
                        ];
                      }

                      fullSnippet = [...fullSnippet, "", ...definitionSnippet];
                    }
                  }
                }
              });

            if (imports.length > 0) {
              fullSnippet = [
                `import { ${imports.join(", ")} } from "@meshsdk/core";`,
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
