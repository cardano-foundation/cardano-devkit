import * as vscode from "vscode";

export function activate(context: vscode.ExtensionContext) {
  console.log("Cardano devkit extension activated");

  const read = vscode.commands.registerCommand(
    "cardano-extension.readFile",
    () => {
      vscode.window.showInformationMessage("Reading File");
      const options: vscode.OpenDialogOptions = {
        canSelectMany: false,
        canSelectFolders: false,
        canSelectFiles: true,
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

          Object.keys(script).forEach((key) => {
            const title = script[key].title;
            const fields = script[key].anyOf[0]?.fields;

            if (title) {
              const code = [`export type ${script[key].title} = ConStr0<[`];

              if (fields) {
                fields.forEach((field: any) => {
                  console.log(field);
                  switch (field.title) {
                    case "oracle_address":
                      code.push(`ScriptAddress, // ${field.title}: Address`);
                      break;
                    case "operation_key":
                    case "stop_key":
                      code.push(
                        `PubKeyHash, // ${field.title}: VerificationKeyHash,`
                      );
                      break;
                  }
                });
              }

              code.push(`]>;`);
              const snippet = new vscode.SnippetString(code.join("\n"));
              vscode.window.activeTextEditor?.insertSnippet(snippet);
            }
          });
        }
      });
    }
  );

  context.subscriptions.push(read);
}

// This method is called when your extension is deactivated
export function deactivate() {}
