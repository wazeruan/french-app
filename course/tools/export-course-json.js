#!/usr/bin/env node
/* Convert the existing generated course.js data into the JSON consumed by the Rust app. */
"use strict";

const fs = require("node:fs");
const path = require("node:path");

const courseDir = path.resolve(__dirname, "..");
const source = fs.readFileSync(path.join(courseDir, "course.js"), "utf8");
const match = /^\s*const\s+COURSE\s*=\s*([\s\S]*);?\s*$/.exec(source);
if (!match) throw new Error("course.js does not contain the expected COURSE object");

const course = JSON.parse(match[1].replace(/;\s*$/, ""));
if (!Array.isArray(course.units) || course.units.length !== 25) {
  throw new Error("Expected the complete 25-unit course");
}

const destination = path.join(courseDir, "course.json");
fs.writeFileSync(destination, JSON.stringify(course, null, 2) + "\n");
console.log(`Exported ${course.units.length} units to ${destination}`);
