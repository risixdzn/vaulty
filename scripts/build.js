const { execSync } = require("child_process");
const { existsSync, mkdirSync, copyFileSync } = require("fs");
const path = require("path");

const BINARY_NAME = "vaulty.exe";
const TARGET_DIR = "target/release";
const BIN_DIR = path.join(__dirname, "..", "bin");

console.log("Building Vaulty...");
execSync("cargo build --release", { stdio: "inherit" });

if (!existsSync(BIN_DIR)) {
    mkdirSync(BIN_DIR);
}

/* Copying binary to the ./bin dir, so it can be correctly referenced in the npm package */
const binaryPath = path.join(__dirname, "..", TARGET_DIR, BINARY_NAME);
const destinationPath = path.join(BIN_DIR, BINARY_NAME);

console.log(`Copying ${binaryPath} to ${destinationPath}...`);
copyFileSync(binaryPath, destinationPath);

console.log("Build completed successfully!");
