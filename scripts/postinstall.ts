import { createWriteStream } from "fs";
import { chmodSync } from "fs";
import { pipeline } from "stream";
import { promisify } from "util";
import { get } from "https";
import { join } from "path";
import { readFileSync } from "fs";
import { fileURLToPath } from "url";
import path from "path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const pipe = promisify(pipeline);

const REPO_OWNER = "risixdzn";
const REPO_NAME = "vaulty";

async function downloadBinary(version: string) {
    const platform = process.platform;
    const arch = process.arch;

    const binaryName = "vaulty";
    const binaryPath = join(__dirname, "../bin", binaryName);

    const url = `https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v${version}/${binaryName}-${platform}-${arch}`;

    console.log(`Downloading ${binaryName} from ${url}`);

    return new Promise<void>((resolve, reject) => {
        get(url, (response) => {
            if (response.statusCode !== 200) {
                reject(new Error(`Failed to download binary. Status code: ${response.statusCode}`));
                return;
            }

            const fileStream = createWriteStream(binaryPath);
            pipe(response, fileStream)
                .then(() => {
                    chmodSync(binaryPath, 0o755);
                    console.log(`Binary downloaded to ${binaryPath}`);
                    resolve();
                })
                .catch(reject);
        }).on("error", reject);
    });
}

(async () => {
    try {
        const packageJson = JSON.parse(readFileSync(join(__dirname, "../package.json"), "utf8"));
        const version = packageJson.version;
        if (!version) throw new Error("Package version not found.");
        await downloadBinary(version);
    } catch (error) {
        console.error("Postinstall script failed:", error);
        process.exit(1);
    }
})();
