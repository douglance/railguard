/**
 * Custom Shiki theme based on Atom One Dark with International Orange (#FF4F00) accents.
 * Uses TextMate theme format for Shiki compatibility.
 */

import type { ThemeRegistration } from "shiki";

export const orangeDarkTheme: ThemeRegistration = {
  name: "orange-dark",
  type: "dark",
  colors: {
    // Editor colors
    "editor.background": "#0a0a0a",
    "editor.foreground": "#e6e6e6",
    "editor.lineHighlightBackground": "#1a1a1a",
    "editor.selectionBackground": "#ff4f0040",
    "editorCursor.foreground": "#FF4F00",
    "editorWhitespace.foreground": "#3a3a3a",
    "editorIndentGuide.background": "#2a2a2a",
    "editorIndentGuide.activeBackground": "#4a4a4a",
    "editorLineNumber.foreground": "#5a5a5a",
    "editorLineNumber.activeForeground": "#FF4F00",
  },
  tokenColors: [
    // Comments - muted gray
    {
      scope: [
        "comment",
        "punctuation.definition.comment",
        "comment.block",
        "comment.line",
        "comment.block.documentation",
      ],
      settings: {
        foreground: "#7f7f7f",
        fontStyle: "italic",
      },
    },
    // Keywords - International Orange
    {
      scope: [
        "keyword",
        "keyword.control",
        "keyword.operator.new",
        "keyword.operator.expression",
        "keyword.operator.logical",
        "keyword.operator.delete",
        "keyword.operator.typeof",
        "keyword.operator.void",
        "keyword.operator.instanceof",
        "storage",
        "storage.type",
        "storage.modifier",
      ],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // Control flow keywords - bright orange
    {
      scope: [
        "keyword.control.conditional",
        "keyword.control.loop",
        "keyword.control.flow",
        "keyword.control.import",
        "keyword.control.export",
        "keyword.control.from",
        "keyword.control.default",
      ],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // Strings - lighter orange/amber
    {
      scope: [
        "string",
        "string.quoted",
        "string.quoted.single",
        "string.quoted.double",
        "string.quoted.template",
        "string.template",
      ],
      settings: {
        foreground: "#ff8c4d",
      },
    },
    // String interpolation
    {
      scope: [
        "punctuation.definition.template-expression",
        "punctuation.section.embedded",
      ],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // Numbers - lighter orange
    {
      scope: [
        "constant.numeric",
        "constant.numeric.integer",
        "constant.numeric.float",
        "constant.numeric.hex",
        "constant.numeric.octal",
        "constant.numeric.binary",
      ],
      settings: {
        foreground: "#ff8c4d",
      },
    },
    // Constants and booleans
    {
      scope: [
        "constant.language",
        "constant.language.boolean",
        "constant.language.null",
        "constant.language.undefined",
      ],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // Functions - pure white for contrast
    {
      scope: [
        "entity.name.function",
        "meta.function-call",
        "support.function",
        "meta.function-call entity.name.function",
      ],
      settings: {
        foreground: "#ffffff",
      },
    },
    // Function parameters - muted warm
    {
      scope: ["variable.parameter", "meta.parameter"],
      settings: {
        foreground: "#d4b896",
        fontStyle: "italic",
      },
    },
    // Variables - light peach
    {
      scope: [
        "variable",
        "variable.other",
        "variable.other.readwrite",
        "variable.other.object",
      ],
      settings: {
        foreground: "#ffcc99",
      },
    },
    // Object properties - saturated amber
    {
      scope: [
        "variable.other.property",
        "variable.other.object.property",
        "meta.object-literal.key",
      ],
      settings: {
        foreground: "#ff9955",
      },
    },
    // Classes and types - light cream
    {
      scope: [
        "entity.name.class",
        "entity.name.type",
        "entity.name.type.class",
        "support.class",
        "entity.other.inherited-class",
      ],
      settings: {
        foreground: "#ffeedd",
      },
    },
    // Type annotations - muted orange
    {
      scope: [
        "entity.name.type.alias",
        "entity.name.type.interface",
        "entity.name.type.enum",
        "entity.name.type.module",
        "support.type",
        "support.type.primitive",
      ],
      settings: {
        foreground: "#cc6633",
      },
    },
    // Operators
    {
      scope: [
        "keyword.operator",
        "keyword.operator.assignment",
        "keyword.operator.arithmetic",
        "keyword.operator.comparison",
        "keyword.operator.relational",
      ],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // Punctuation - muted brown
    {
      scope: [
        "punctuation",
        "punctuation.definition",
        "punctuation.separator",
        "punctuation.terminator",
        "meta.brace",
      ],
      settings: {
        foreground: "#8a7060",
      },
    },
    // Brackets and braces - warm tan
    {
      scope: [
        "punctuation.definition.block",
        "punctuation.section.block",
        "meta.brace.curly",
        "meta.brace.round",
        "meta.brace.square",
      ],
      settings: {
        foreground: "#aa8866",
      },
    },
    // HTML/JSX tags - orange
    {
      scope: [
        "entity.name.tag",
        "punctuation.definition.tag",
        "support.class.component",
      ],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // HTML/JSX attributes
    {
      scope: [
        "entity.other.attribute-name",
        "entity.other.attribute-name.html",
        "entity.other.attribute-name.jsx",
      ],
      settings: {
        foreground: "#ff8c4d",
        fontStyle: "italic",
      },
    },
    // CSS selectors
    {
      scope: [
        "entity.name.tag.css",
        "entity.other.attribute-name.class.css",
        "entity.other.attribute-name.id.css",
      ],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // CSS properties
    {
      scope: ["support.type.property-name.css", "meta.property-name.css"],
      settings: {
        foreground: "#ffcc99",
      },
    },
    // CSS values
    {
      scope: [
        "support.constant.property-value.css",
        "meta.property-value.css",
        "constant.numeric.css",
      ],
      settings: {
        foreground: "#ff8c4d",
      },
    },
    // Regex
    {
      scope: [
        "string.regexp",
        "constant.other.character-class.regexp",
        "keyword.operator.quantifier.regexp",
      ],
      settings: {
        foreground: "#ff8c4d",
      },
    },
    // Escape characters
    {
      scope: ["constant.character.escape"],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // Markdown headings
    {
      scope: [
        "markup.heading",
        "markup.heading.markdown",
        "entity.name.section.markdown",
      ],
      settings: {
        foreground: "#FF4F00",
        fontStyle: "bold",
      },
    },
    // Markdown bold
    {
      scope: ["markup.bold"],
      settings: {
        foreground: "#ffffff",
        fontStyle: "bold",
      },
    },
    // Markdown italic
    {
      scope: ["markup.italic"],
      settings: {
        foreground: "#ff8c4d",
        fontStyle: "italic",
      },
    },
    // Markdown links
    {
      scope: [
        "markup.underline.link",
        "string.other.link.title.markdown",
        "string.other.link.description.markdown",
      ],
      settings: {
        foreground: "#ff8c4d",
      },
    },
    // Markdown code
    {
      scope: ["markup.inline.raw", "markup.fenced_code.block"],
      settings: {
        foreground: "#cc9966",
      },
    },
    // JSON keys
    {
      scope: ["support.type.property-name.json"],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // JSON values
    {
      scope: ["string.quoted.double.json"],
      settings: {
        foreground: "#ff8c4d",
      },
    },
    // YAML keys
    {
      scope: ["entity.name.tag.yaml"],
      settings: {
        foreground: "#FF4F00",
      },
    },
    // Decorators/Annotations
    {
      scope: [
        "meta.decorator",
        "punctuation.decorator",
        "entity.name.function.decorator",
      ],
      settings: {
        foreground: "#cc6633",
        fontStyle: "italic",
      },
    },
    // Import/Export paths
    {
      scope: ["string.quoted.module-ref"],
      settings: {
        foreground: "#ff8c4d",
      },
    },
    // this/self keywords
    {
      scope: ["variable.language.this", "variable.language.self"],
      settings: {
        foreground: "#FF4F00",
        fontStyle: "italic",
      },
    },
    // Support/built-in
    {
      scope: ["support.variable", "support.constant"],
      settings: {
        foreground: "#cc6633",
      },
    },
  ],
};

export const orangeLightTheme: ThemeRegistration = {
  name: "orange-light",
  type: "light",
  colors: {
    "editor.background": "#fafafa",
    "editor.foreground": "#1a1a1a",
    "editor.lineHighlightBackground": "#f0f0f0",
    "editor.selectionBackground": "#ff4f0030",
    "editorCursor.foreground": "#cc3300",
    "editorWhitespace.foreground": "#d0d0d0",
    "editorIndentGuide.background": "#e0e0e0",
    "editorIndentGuide.activeBackground": "#c0c0c0",
    "editorLineNumber.foreground": "#999999",
    "editorLineNumber.activeForeground": "#cc3300",
  },
  tokenColors: [
    // Comments
    {
      scope: [
        "comment",
        "punctuation.definition.comment",
        "comment.block",
        "comment.line",
      ],
      settings: {
        foreground: "#8a8a8a",
        fontStyle: "italic",
      },
    },
    // Keywords - dark orange
    {
      scope: [
        "keyword",
        "keyword.control",
        "keyword.operator.new",
        "keyword.operator.expression",
        "storage",
        "storage.type",
        "storage.modifier",
      ],
      settings: {
        foreground: "#cc3300",
      },
    },
    // Strings - dark amber
    {
      scope: ["string", "string.quoted", "string.template"],
      settings: {
        foreground: "#995522",
      },
    },
    // Numbers
    {
      scope: ["constant.numeric"],
      settings: {
        foreground: "#aa4400",
      },
    },
    // Constants
    {
      scope: ["constant.language"],
      settings: {
        foreground: "#cc3300",
      },
    },
    // Functions - dark gray
    {
      scope: ["entity.name.function", "support.function"],
      settings: {
        foreground: "#1a1a1a",
      },
    },
    // Variables
    {
      scope: ["variable", "variable.other"],
      settings: {
        foreground: "#333333",
      },
    },
    // Classes and types
    {
      scope: ["entity.name.class", "entity.name.type", "support.class"],
      settings: {
        foreground: "#1a1a1a",
      },
    },
    // Type annotations
    {
      scope: [
        "entity.name.type.alias",
        "entity.name.type.interface",
        "support.type",
      ],
      settings: {
        foreground: "#884400",
      },
    },
    // Operators
    {
      scope: ["keyword.operator"],
      settings: {
        foreground: "#cc3300",
      },
    },
    // Punctuation
    {
      scope: ["punctuation", "meta.brace"],
      settings: {
        foreground: "#666666",
      },
    },
    // HTML/JSX tags
    {
      scope: ["entity.name.tag", "punctuation.definition.tag"],
      settings: {
        foreground: "#cc3300",
      },
    },
    // Attributes
    {
      scope: ["entity.other.attribute-name"],
      settings: {
        foreground: "#995522",
        fontStyle: "italic",
      },
    },
    // JSON keys
    {
      scope: ["support.type.property-name.json"],
      settings: {
        foreground: "#cc3300",
      },
    },
    // Markdown headings
    {
      scope: ["markup.heading", "entity.name.section.markdown"],
      settings: {
        foreground: "#cc3300",
        fontStyle: "bold",
      },
    },
  ],
};
