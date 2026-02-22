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

class ManylebCompletionProvider implements vscode.CompletionItemProvider {
  provideCompletionItems(
    document: vscode.TextDocument,
    position: vscode.Position,
    token: vscode.CancellationToken,
    context: vscode.CompletionContext,
  ): vscode.ProviderResult<vscode.CompletionItem[] | vscode.CompletionList> {
    const lineText = document.lineAt(position.line).text;
    const linePrefix = lineText.substring(0, position.character);
    const text = document.getText();

    const completions: vscode.CompletionItem[] = [];

    const topLevelKeywords = [
      {
        label: "version",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Specify API version",
      },
      {
        label: "title",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Specify API title",
      },
      {
        label: "description",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Specify API description",
      },
      {
        label: "object",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Define an object type",
      },
      {
        label: "route",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Define an API route",
      },
    ];

    const routeKeywords = [
      {
        label: "description",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Route description",
      },
      {
        label: "tag",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Add a tag",
      },
      {
        label: "body",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Request body type",
      },
      {
        label: "response",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Response definition",
      },
      {
        label: "param",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Path parameter",
      },
      {
        label: "query",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Query parameter",
      },
    ];

    const objectKeywords = [
      {
        label: "prop",
        kind: vscode.CompletionItemKind.Keyword,
        detail: "Define a property",
      },
    ];

    const httpMethods = [
      {
        label: "get",
        kind: vscode.CompletionItemKind.Method,
        detail: "HTTP GET",
      },
      {
        label: "post",
        kind: vscode.CompletionItemKind.Method,
        detail: "HTTP POST",
      },
      {
        label: "put",
        kind: vscode.CompletionItemKind.Method,
        detail: "HTTP PUT",
      },
      {
        label: "patch",
        kind: vscode.CompletionItemKind.Method,
        detail: "HTTP PATCH",
      },
      {
        label: "delete",
        kind: vscode.CompletionItemKind.Method,
        detail: "HTTP DELETE",
      },
      {
        label: "head",
        kind: vscode.CompletionItemKind.Method,
        detail: "HTTP HEAD",
      },
      {
        label: "options",
        kind: vscode.CompletionItemKind.Method,
        detail: "HTTP OPTIONS",
      },
    ];

    const builtInTypes = [
      {
        label: "string",
        kind: vscode.CompletionItemKind.TypeParameter,
        detail: "String type",
      },
      {
        label: "str",
        kind: vscode.CompletionItemKind.TypeParameter,
        detail: "String type (alias)",
      },
      {
        label: "integer",
        kind: vscode.CompletionItemKind.TypeParameter,
        detail: "Integer type",
      },
      {
        label: "int",
        kind: vscode.CompletionItemKind.TypeParameter,
        detail: "Integer type (alias)",
      },
      {
        label: "float",
        kind: vscode.CompletionItemKind.TypeParameter,
        detail: "Float type",
      },
      {
        label: "boolean",
        kind: vscode.CompletionItemKind.TypeParameter,
        detail: "Boolean type",
      },
      {
        label: "bool",
        kind: vscode.CompletionItemKind.TypeParameter,
        detail: "Boolean type (alias)",
      },
      {
        label: "any",
        kind: vscode.CompletionItemKind.TypeParameter,
        detail: "Any type",
      },
      {
        label: "null",
        kind: vscode.CompletionItemKind.TypeParameter,
        detail: "Null type",
      },
    ];

    const objectRegex = /\bobject\s+(\w+)\s*\{/g;
    let match: RegExpExecArray | null;
    const definedObjects: vscode.CompletionItem[] = [];

    while ((match = objectRegex.exec(text)) !== null) {
      const objectName = match[1];
      definedObjects.push({
        label: objectName,
        kind: vscode.CompletionItemKind.Class,
        detail: "User-defined type",
      });
    }

    const isInRouteBlock = this.isInBlock(text, position, "route");
    const isInObjectBlock = this.isInBlock(text, position, "object");
    const isAfterRouteKeyword = /\broute\s+$/.test(linePrefix);
    const isAfterTypeKeyword =
      /\b(prop|body|response\s+\d+|param|query)\s+\w+\s+$/.test(linePrefix);

    if (isAfterRouteKeyword) {
      completions.push(
        ...httpMethods.map((m) => {
          const item = new vscode.CompletionItem(m.label, m.kind);
          item.detail = m.detail;
          return item;
        }),
      );
    } else if (isAfterTypeKeyword) {
      completions.push(
        ...builtInTypes.map((t) => {
          const item = new vscode.CompletionItem(t.label, t.kind);
          item.detail = t.detail;
          return item;
        }),
      );
      completions.push(...definedObjects);
    } else if (isInRouteBlock) {
      completions.push(
        ...routeKeywords.map((k) => {
          const item = new vscode.CompletionItem(k.label, k.kind);
          item.detail = k.detail;
          return item;
        }),
      );
      completions.push(...definedObjects);
    } else if (isInObjectBlock) {
      completions.push(
        ...objectKeywords.map((k) => {
          const item = new vscode.CompletionItem(k.label, k.kind);
          item.detail = k.detail;
          return item;
        }),
      );
      completions.push(
        ...builtInTypes.map((t) => {
          const item = new vscode.CompletionItem(t.label, t.kind);
          item.detail = t.detail;
          return item;
        }),
      );
      completions.push(...definedObjects);
    } else {
      completions.push(
        ...topLevelKeywords.map((k) => {
          const item = new vscode.CompletionItem(k.label, k.kind);
          item.detail = k.detail;
          return item;
        }),
      );
    }

    return completions;
  }

  private isInBlock(
    text: string,
    position: vscode.Position,
    blockType: string,
  ): boolean {
    const lines = text.split("\n");
    let inBlock = false;
    let braceCount = 0;

    for (let i = 0; i <= position.line; i++) {
      const line = lines[i];

      const blockRegex = new RegExp(`\\b${blockType}\\b.*\\{`);
      if (blockRegex.test(line)) {
        inBlock = true;
      }

      for (const char of line) {
        if (char === "{") {
          braceCount++;
        } else if (char === "}") {
          braceCount--;
          if (braceCount === 0) {
            inBlock = false;
          }
        }
      }
    }

    return inBlock && braceCount > 0;
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

  const completionDisposable = vscode.languages.registerCompletionItemProvider(
    "manyleb",
    new ManylebCompletionProvider(),
    " ", // Trigger on space
  );

  context.subscriptions.push(
    formatterDisposable,
    definitionDisposable,
    renameDisposable,
    completionDisposable,
  );
}

export function deactivate() {}
