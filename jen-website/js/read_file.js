const fs = require("fs");

export function read_file(path) {
    return fs.readFileSync(path, { encoding: "utf8" });
}
