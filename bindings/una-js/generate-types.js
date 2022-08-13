const fs = require("node:fs/promises");
const path = require("path");
const url = require("url");
const { compile } = require("json-schema-to-typescript");

async function main() {
  console.log("Generating Typescript types from JSON Schemas.");

  let schemasPath = path.join(__dirname, ".", "schemas");
  let schemaFiles = (await fs.readdir(schemasPath)).filter((x) => x.endsWith(".json"));

  // Compile all types, stripping out duplicates. This is a bit dumb but the easiest way to
  // do it since we can't suppress generation of definition references.
  let compiledTypes = new Set();
  for (let filename of schemaFiles) {
    let filePath = path.join(schemasPath, filename);
    let schema = JSON.parse(await fs.readFile(filePath));
    let compiled = await compile(schema, schema.title, { bannerComment: "", additionalProperties: false });

    let eachType = compiled.split("export");
    for (let type of eachType) {
      if (!type) {
        continue;
      }
      compiledTypes.add("export " + type.trim());
    }
  }

  let outputPath = path.join(__dirname, "index.d.ts");
  let existing = await fs.readFile(outputPath);

  let output = existing.toString() + "\n" + Array.from(compiledTypes).join("\n\n");

  await fs.writeFile(outputPath, output);
  console.log(`Appened Typescript types to ${outputPath}`);
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
