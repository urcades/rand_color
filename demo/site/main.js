import init, {
  convert_rgb_to_hsl,
  generate_all_spaces,
  generate_avatar,
  generate_colors,
} from "./pkg/rand_color_demo_wasm.js";

const avatarSampleKeys = [
  "7f35a4db-9d18-4696-bf5a-fc4e835ef9bd",
  "rand_color",
  "wasm-demo",
  "flower-computer",
];

const elements = {
  alpha: document.querySelector("#alpha"),
  allSpaces: document.querySelector("#all-spaces"),
  avatarGenerate: document.querySelector("#avatar-generate"),
  avatarKey: document.querySelector("#avatar-key"),
  avatarOutput: document.querySelector("#avatar-output"),
  avatarSamples: document.querySelector("#avatar-samples"),
  avatarSnippet: document.querySelector("#avatar-snippet"),
  blue: document.querySelector("#blue"),
  conversion: document.querySelector("#conversion"),
  convert: document.querySelector("#convert"),
  convertSnippet: document.querySelector("#convert-snippet"),
  count: document.querySelector("#count"),
  generate: document.querySelector("#generate"),
  green: document.querySelector("#green"),
  red: document.querySelector("#red"),
  results: document.querySelector("#results"),
  seed: document.querySelector("#seed"),
  space: document.querySelector("#space"),
  spaceResults: document.querySelector("#space-results"),
  snippet: document.querySelector("#snippet"),
};

await init();

elements.generate.addEventListener("click", renderGeneratedColors);
elements.allSpaces.addEventListener("click", renderAllSpaces);
elements.convert.addEventListener("click", renderConversion);
elements.avatarGenerate.addEventListener("click", renderAvatar);

renderGeneratedColors();
renderAllSpaces();
renderAvatar();
renderAvatarSamples();
renderConversion();

function renderGeneratedColors() {
  const generated = generate_colors(
    elements.space.value,
    numberValue(elements.count),
    numberValue(elements.seed),
  );

  elements.snippet.textContent = generated.snippet;
  elements.results.replaceChildren(
    ...generated.colors.map((color) =>
      row([swatch(color.css_preview), code(color.crate_output), text(color.components)]),
    ),
  );
}

function renderAllSpaces() {
  const colors = generate_all_spaces(numberValue(elements.seed));
  elements.spaceResults.replaceChildren(
    ...colors.map((color) =>
      row([
        swatch(color.css_preview),
        text(color.space),
        code(color.crate_output),
        previewCss(color),
      ]),
    ),
  );
}

function renderConversion() {
  const result = convert_rgb_to_hsl(
    clamp(numberValue(elements.red), 0, 255),
    clamp(numberValue(elements.green), 0, 255),
    clamp(numberValue(elements.blue), 0, 255),
    clamp(floatValue(elements.alpha), 0, 1),
  );

  elements.convertSnippet.textContent = result.snippet;
  elements.conversion.replaceChildren(
    row([text("Input RGB"), swatch(result.input_css), code(result.input)]),
    row([text("Converted HSL"), text(""), code(result.hsl)]),
    row([text("Round trip RGB"), swatch(result.round_trip_css), code(result.round_trip)]),
    row([text("Round-trip delta"), text(""), text(result.delta)]),
  );
}

function renderAvatar() {
  const avatar = generate_avatar(elements.avatarKey.value);

  elements.avatarSnippet.textContent = avatar.snippet;
  elements.avatarOutput.replaceChildren(
    avatarSvg(avatar, 160, "avatar-main"),
    avatarDetails(avatar),
  );
}

function renderAvatarSamples() {
  elements.avatarSamples.replaceChildren(
    ...avatarSampleKeys.map((key, index) => {
      const avatar = generate_avatar(key);

      return row([
        avatarSvg(avatar, 64, `avatar-sample-${index}`),
        code(avatar.key),
        code(avatar.seed),
        avatarPalette(avatar),
      ]);
    }),
  );
}

function avatarSvg(avatar, size, idPrefix) {
  const clipId = `${idPrefix}-${avatar.seed}`;
  const svg = svgElement("svg", {
    "aria-label": `Avatar for ${avatar.key}`,
    height: size,
    role: "img",
    viewBox: "0 0 100 100",
    width: size,
  });
  const defs = svgElement("defs");
  const clipPath = svgElement("clipPath", { id: clipId });
  clipPath.append(svgElement("circle", { cx: 50, cy: 50, r: 50 }));
  defs.append(clipPath);

  const clipped = svgElement("g", { "clip-path": `url(#${clipId})` });
  clipped.append(svgElement("rect", { fill: avatar.background, height: 100, width: 100 }));

  for (const cell of avatar.cells) {
    clipped.append(
      svgElement("rect", {
        fill: avatar[cell.fill],
        height: 18,
        width: 18,
        x: cell.x * 20 + 1,
        y: cell.y * 20 + 1,
      }),
    );
  }

  svg.append(
    defs,
    clipped,
    svgElement("circle", {
      cx: 50,
      cy: 50,
      fill: "none",
      r: 48,
      stroke: avatar.accent,
      "stroke-width": 4,
    }),
  );

  return svg;
}

function avatarDetails(avatar) {
  const table = document.createElement("table");
  const tbody = document.createElement("tbody");
  tbody.append(
    row([text("Key"), code(avatar.key)]),
    row([text("Seed"), code(avatar.seed)]),
    row([text("Background"), code(avatar.background)]),
    row([text("Foreground"), code(avatar.foreground)]),
    row([text("Accent"), code(avatar.accent)]),
  );
  table.append(tbody);
  return table;
}

function avatarPalette(avatar) {
  const container = document.createElement("span");
  container.append(
    smallSwatch(avatar.background),
    text(" "),
    smallSwatch(avatar.foreground),
    text(" "),
    smallSwatch(avatar.accent),
  );
  return container;
}

function row(cells) {
  const tr = document.createElement("tr");
  tr.append(...cells.map((child) => cell(child)));
  return tr;
}

function cell(child) {
  const td = document.createElement("td");
  td.append(child);
  return td;
}

function swatch(cssColor) {
  const output = document.createElement("output");
  output.value = "";
  output.title = cssColor;
  output.style.background = cssColor;
  output.style.border = "1px solid currentColor";
  output.style.display = "inline-block";
  output.style.height = "3rem";
  output.style.width = "5rem";
  return output;
}

function smallSwatch(cssColor) {
  const output = swatch(cssColor);
  output.style.height = "1.5rem";
  output.style.width = "1.5rem";
  return output;
}

function svgElement(name, attributes = {}) {
  const element = document.createElementNS("http://www.w3.org/2000/svg", name);

  for (const [attribute, value] of Object.entries(attributes)) {
    element.setAttribute(attribute, value);
  }

  return element;
}

function code(value) {
  const element = document.createElement("code");
  element.textContent = value;
  return element;
}

function previewCss(color) {
  if (color.css_preview === color.crate_output) {
    return text("same as output");
  }

  return code(color.css_preview);
}

function text(value) {
  return document.createTextNode(value);
}

function numberValue(input) {
  return Number.parseInt(input.value, 10) || 0;
}

function floatValue(input) {
  return Number.parseFloat(input.value) || 0;
}

function clamp(value, min, max) {
  return Math.min(Math.max(value, min), max);
}
