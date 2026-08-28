import { rmSync } from "fs";

const targets = [
    "dist/",
    "src-tauri/gen/",
    "src-tauri/target/"
];

targets.forEach(element => {
    console.log("Deleting ", element);
    rmSync(element, { recursive: true, force: true });
});
