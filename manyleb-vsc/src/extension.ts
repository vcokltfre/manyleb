import * as vscode from "vscode";
import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

class ManylebDefinitionProvider implements vscode.DefinitionProvider {
  provideDefinition(
    document: vscode.TextDocument,
    position: vscode.Position,
    token: vscode.CancellationToken,
  ): vscode.ProviderResult<vscode.Definition | vscode.LocationLink[]> {
    const wordRange = document.getWordRangeAtPosition(position);
    if (!wordRange) {
      return null;
    }

    const word = document.getText(wordRange);
    const text = document.getText();

    const objectRegex = new RegExp(`\\bobject\\s+(${word})\\s*\\{`, "g");
    let match: RegExpExecArray | null;

    while ((match = objectRegex.exec(text)) !== null) {
      const matchPosition = document.positionAt(
        match.index + match[0].indexOf(match[1]),
      );
      const matchRange = new vscode.Range(
        matchPosition,
        new vscode.Position(
          matchPosition.line,
          matchPosition.character + match[1].length,
        ),
      );

      return new vscode.Location(document.uri, matchRange);
    }

    return null;
  }
}

class ManylebRenameProvider implements vscode.RenameProvider {
  prepareRename(
    document: vscode.TextDocument,
    position: vscode.Position,
    token: vscode.CancellationToken,
  ): vscode.ProviderResult<
    vscode.Range | { range: vscode.Range; placeholder: string }
  > {
    const wordRange = document.getWordRangeAtPosition(position);
    if (!wordRange) {
      return null;
    }

    const word = document.getText(wordRange);
    return { range: wordRange, placeholder: word };
  }

  provideRenameEdits(
    document: vscode.TextDocument,
    position: vscode.Position,
    newName: string,
    token: vscode.CancellationToken,
  ): vscode.ProviderResult<vscode.WorkspaceEdit> {
    const wordRange = document.getWordRangeAtPosition(position);
    if (!wordRange) {
      return null;
    }

    const word = document.getText(wordRange);
    const text = document.getText();
    const workspaceEdit = new vscode.WorkspaceEdit();

    const objectRegex = new RegExp(`\\bobject\\s+(${word})\\b`, "g");
    let match: RegExpExecArray | null;

    while ((match = objectRegex.exec(text)) !== null) {
      const matchPosition = document.positionAt(
        match.index + match[0].indexOf(match[1]),
      );
      const matchRange = new vscode.Range(
        matchPosition,
        new vscode.Position(
          matchPosition.line,
          matchPosition.character + match[1].length,
        ),
      );
      workspaceEdit.replace(document.uri, matchRange, newName);
    }

    const typeRefRegex = new RegExp(`\\b(${word})\\b`, "g");

    while ((match = typeRefRegex.exec(text)) !== null) {
      const matchPosition = document.positionAt(match.index);
      const matchRange = new vscode.Range(
        matchPosition,
        new vscode.Position(
          matchPosition.line,
          matchPosition.character + match[1].length,
        ),
      );

      const lineText = document.lineAt(matchPosition.line).text;
      const objectPattern = new RegExp(`\\bobject\\s+${word}\\b`);
      if (!objectPattern.test(lineText)) {
        workspaceEdit.replace(document.uri, matchRange, newName);
      }
    }

    return workspaceEdit;
  }
}

export function activate(context: vscode.ExtensionContext) {
  const formatterDisposable =
    vscode.languages.registerDocumentFormattingEditProvider("manyleb", {
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
    });

  const definitionDisposable = vscode.languages.registerDefinitionProvider(
    "manyleb",
    new ManylebDefinitionProvider(),
  );

  const renameDisposable = vscode.languages.registerRenameProvider(
    "manyleb",
    new ManylebRenameProvider(),
  );

  context.subscriptions.push(
    formatterDisposable,
    definitionDisposable,
    renameDisposable,
  );
}

export function deactivate() {}
