type ASTNodeType = "root" | "bold" | "italic";

type ASTNode =
  | {
      type: ASTNodeType;
      length: number;
      children: ASTNode[];
    }
  | string;

type Context = {
  bold: boolean;
  italic: boolean;
};

function parseNormalMd(
  content: string,
  token: string,
  type: "bold" | "italic",
  context: Partial<Context> = {}
): ASTNode {
  content = content.substring(0, content.indexOf(token));
  const children = parser(content, { ...context, [type]: true });
  const length = children.reduce((acc, next) => acc + next.length, 0) + token.length * 2;

  return {
    type,
    length,
    children,
  };
}

function parser(content: string, context: Partial<Context> = {}): ASTNode[] {
  const nodes: ASTNode[] = [];

  let escaped = false;

  for (let charIndex = 0; charIndex < content.length; ++charIndex) {
    switch (content[charIndex]) {
      case '"':
        escaped = true;
        break;
      case "*":
        if (content[charIndex + 1] === "*") {
          const boldNode = parseNormalMd(
            content.substring(charIndex + 2),
            "**",
            "bold",
            context
          );
          charIndex += boldNode.length - 1;
          nodes.push(boldNode);
        } else {
          const italicNode = parseNormalMd(
            content.substring(charIndex + 1),
            "*",
            "italic",
            context
          );
          charIndex += italicNode.length - 1;
          nodes.push(italicNode);
        }
        break;
      default: {
        const lastNode = nodes[nodes.length - 1];

        if (typeof lastNode === "string") {
          nodes[nodes.length - 1] += content[charIndex];
        } else {
          nodes.push(content[charIndex]);
        }
      }
    }
  }

  return nodes;
}

export default parser;
