type ASTNodeType = "root" | "bold" | "italic";

type ASTNode =
  | {
      type: ASTNodeType;
      length: number;
      children: ASTNode[];
    }
  | string;

type Context = ASTNodeType[];

function parser(content: string, context: Context = []): ASTNode[] {
  const nodes: ASTNode[] = [];

  let buffers = [];

  for (let charIndex = 0; charIndex < content.length; ++charIndex) {
    const currentScope = context[context.length - 1];

    switch (content[charIndex]) {
      case "*":
        if (currentScope === "italic") {
          const children = parser(
            buffers.pop(),
            context.slice(0, context.length - 2)
          );
          const length =
            children.reduce((acc, next) => acc + next.length, 0) +
            "*".length * 2;
          nodes.push({
            type: "italic",
            length,
            children,
          });
          // charIndex += length + "**".length;
          context.pop();
        } else if (content[charIndex + 1] === "*") {
          if (currentScope === "bold") {
            const children = parser(
              buffers.pop(),
              context.slice(0, context.length - 2)
            );
            const length =
              children.reduce((acc, next) => acc + next.length, 0) +
              "**".length * 2;
            nodes.push({
              type: "bold",
              length,
              children,
            });
            // charIndex += length;
            context.pop();
          } else {
            context.push("bold");
            buffers.push("");
          }
          charIndex += 1;
        } else {
          context.push("italic");
          buffers.push("");
        }
        continue;
    }

    if (currentScope !== undefined) {
      buffers[buffers.length - 1] += content[charIndex];
      continue;
    }

    const lastNode = nodes[nodes.length - 1];

    if (typeof lastNode === "string") {
      nodes[nodes.length - 1] += content[charIndex];
    } else {
      nodes.push(content[charIndex]);
    }
  }

  return nodes;
}

export default parser;
