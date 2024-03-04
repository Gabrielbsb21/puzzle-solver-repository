import { exec } from "child_process";
import path from "path";
const fileName = process.argv[2];

if (!fileName) {
  console.log("Please provide a file name.");
  process.exit(1);
}

// Assume que os arquivos TS estão dentro da pasta 'src' a partir da raiz do projeto
const filePath = path.resolve(__dirname, 'src', fileName);

exec(`npx ts-node "${filePath}"`, (error, stdout, stderr) => {
  if (error) {
    console.error(`exec error: ${error}`);
    return;
  }
  console.log(stdout);
  console.error(stderr);
});


