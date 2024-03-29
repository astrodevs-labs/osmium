import { promises as fs } from 'fs';
import { dirname } from 'path';

async function readFileContent(filePath) {
  try {
    const content = await fs.readFile(filePath, 'utf8');
    return content;
  } catch (error) {
    throw error;
  }
}

async function saveToFile(filepath, content) {
  try {
    const directory = dirname(filepath);
    await fs.mkdir(directory, { recursive: true });
    await fs.writeFile(filepath, content, 'utf8');
  } catch (err) {
    console.error('Error creating the file:', err);
  }
}

const parseJson = (rule) => {
  let content = "";

  const categoryFormated = rule.category.replace("-", "_");
  content += `![](https://img.shields.io/badge/${categoryFormated}-green)\t`;
  content += `![](https://img.shields.io/badge/Default%20Severity-${rule.severity}-yellow)\n\n`;
  content += "## Description\n";
  content += `${rule.description}\n\n`;
  content += "## Options\n";
  if (rule.options.length) {
    content += "description | default\n";
    content += "------------ | -------------\n";
    for (const option of rule.options) {
      content += `${option.description} | ${option.default}\n`;
    }
  } else {
    content += "This rule does not require any options.\n";
  }

  content += "## Example Config\n";
  if (rule.example_config) {
    const exampleConfig = JSON.stringify(JSON.parse(rule.example_config),null,2);
    content += "```json\n";
    content += `${exampleConfig}\n`;
    content += "```\n\n";
  }
  content += "## Examples\n";
  content += "### Good\n";
  if (rule.examples.good.length) {
    for (const example of rule.examples.good) {
      content += `${example.description}\n`;
      content += "```solidity\n";
      content += `${example.code}\n`;
      content += "```\n\n";
    }
  } else {
    content += "This rule does not have good examples.\n";
  }
  content += "### Bad\n";
  if (rule.examples.bad.length) {
    for (const example of rule.examples.bad) {
      content += `${example.description}\n`;
      content += "```solidity\n";
      content += `${example.code}\n`;
      content += "```\n\n";
    }
  } else {
    content += "This rule does not have bad examples.\n";
  }
  content += "## References\n";
  content += `* [Rule source](${rule.source_link})\n`;
  content += `* [Test](${rule.test_link})\n`;
  return content;
}

async function createMarkdownFilesFromJsonArray(path) {
  const fileContent = await readFileContent(path);
  const jsonContent = JSON.parse(fileContent);
  for (const key in jsonContent) {
    let content = '';
    const rule = jsonContent[key];
    const body = parseJson(rule);
    content += `# ${rule.id}\n\n` + body;
    saveToFile(`./${rule.category}/${rule.id}.md`, content);
  }
}

async function main() {
  const args = process.argv.slice(2);
  let filepath = "./docTree.json"

  if (args.length !== 0) {
    filepath = args[0];
  }

  try {
    await fs.access(filepath, fs.constants.F_OK);
    createMarkdownFilesFromJsonArray(filepath);
  } catch (err) {
    console.error('The file does not exist.\n', err);
  }
}

main();