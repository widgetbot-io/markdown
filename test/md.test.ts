import { test, expect } from "vitest";
import parser from "../src/parser";

test("Normal input, no markdown content", () => {
  const input = "Hi, I am an unremarkable piece of text";
  expect(parser(input)).toStrictEqual([input]);
});

test("Bold text", () => {
  const input = "Hi, I am a **bold** piece of text";
  expect(parser(input)).toStrictEqual([
    "Hi, I am a ",
    { type: "bold", length: 8, children: ["bold"] },
    " piece of text",
  ]);
});

test("Italic text", () => {
  const input = "Hi, I am an *italic* piece of text";
  expect(parser(input)).toStrictEqual([
    "Hi, I am an ",
    { type: "italic", length: 8, children: ["italic"] },
    " piece of text",
  ]);
});

test("Bold and italic text", () => {
  const input = "Hi, I am a **bold and *italic*** piece of text";
  expect(parser(input)).toStrictEqual([
    "Hi, I am a ",
    {
      type: "bold",
      length: 6,
      children: [
        "bold and ",
        {
          type: "italic",
          length: 8,
          children: ["italic"],
        },
      ],
    },
    " piece of text",
  ]);
});
