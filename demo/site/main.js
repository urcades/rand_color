import init, {
  convert_rgb_to_hsl,
  generate_all_spaces,
  generate_colors,
} from "./pkg/rand_color_demo_wasm.js";

const elements = {
  alpha: document.querySelector("#alpha"),
  allSpaces: document.querySelector("#all-spaces"),
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

renderGeneratedColors();
renderAllSpaces();
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
      row([
        swatch(color.css_preview),
        text(color.space),
        code(color.crate_output),
        code(color.css_preview),
        text(color.components),
      ]),
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
        code(color.css_preview),
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

function code(value) {
  const element = document.createElement("code");
  element.textContent = value;
  return element;
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
