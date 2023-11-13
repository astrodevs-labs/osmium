const fs = require('fs').promises;

let filename = '';

async function readFileContent(filePath) {
  try {
    const content = await fs.readFile(filePath, 'utf8');
    return content;
  } catch (error) {
    throw error;
  }
}

function saveToFile(path, content) {
  fs.writeFile(path, content, 'utf8', (err) => {
    if (err) {
      console.error('Error creating the file:', err);
      return;
    }
  });
}

function parseJSON(obj, depth) {
  let content = '';
  for (const prop in obj) {
    const value = obj[prop];
    if (prop === 'id') {
      filename = value;
    }
    else if (prop === 'code') {
      const blockCodeDelimiter = "```";
      content += `${prop}: ${blockCodeDelimiter}${value}${blockCodeDelimiter}\n`;
    }
    else if (typeof value === 'object') {
      content += `${'#'.repeat(depth + 1)} ${prop}\n\n`;
      content += parseJSON(value, depth + 1);
    } else {
      content += `${prop}: ${value}\n\n`;
    }
  }
  return content;
}

async function createMarkdownFilesFromJsonArray(path) {
  const fileContent = await readFileContent(path);
  const jsonContent = JSON.parse(fileContent);
  depth = 1;
  for (const key in jsonContent) {
    let content = '';
    const value = jsonContent[key];
    const body = parseJSON(value, depth);
    content += `# ${filename}\n\n` + body;
    saveToFile(`./${filename}.md`, content);
  }
}


const args = process.argv.slice(2);
filepath = "./docTree.json"

if (args.length !== 0) {
  filepath = args[0];
}

fs.access(filepath, fs.constants.F_OK)
  .then(() => {
    createMarkdownFilesFromJsonArray(filepath);
  })
  .catch((err) => {
    console.error('Le fichier n\'existe pas.', err);
  });