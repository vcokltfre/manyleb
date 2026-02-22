import * as vscode from "vscode";
import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

export function activate(context: vscode.ExtensionContext) {
  const disposable = vscode.languages.registerDocumentFormattingEditProvider(
    "manyleb",
    {
      async provideDocumentFormattingEdits(
        document: vscode.TextDocument,
      ): Promise<vscode.TextEdit[]> {
        const filePath = document.uri.fsPath;

        try {
          await execAsync(`manyleb format "${filePath}"`);

          return [];
        } catch (error) {
          vscode.window.showErrorMessage(
            `Failed to format manyleb file: ${error}`,
          );

          return [];
        }
      },
    },
  );

  context.subscriptions.push(disposable);
}

export function deactivate() {}
