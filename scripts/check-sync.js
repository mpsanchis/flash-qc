#!/usr/bin/env node

/**
 * Validates that TypeScript references and package dependencies are in sync.
 *
 * Rules enforced:
 * 1. If a library is at root, modules must use the same version
 * 2. If a library is in a module, it must also be declared at root
 * 3. TypeScript packages must be referenced in root tsconfig.json
 * 4. Local tsconfig must reference other modules it imports from
 */

import { readFileSync, existsSync } from "fs";
import { resolve, dirname, basename } from "path";
import { glob } from "glob";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, "..");

let hasErrors = false;

function error(msg) {
  console.error(`❌ ${msg}`);
  hasErrors = true;
}

function success(msg) {
  console.log(`✅ ${msg}`);
}

function info(msg) {
  console.log(`ℹ️  ${msg}`);
}

function readJson(path) {
  try {
    return JSON.parse(readFileSync(path, "utf-8"));
  } catch {
    return null;
  }
}

// Load root package.json and tsconfig.json
const rootPkg = readJson(resolve(ROOT, "package.json"));
const rootTsconfig = readJson(resolve(ROOT, "tsconfig.json"));

if (!rootPkg) {
  error("Could not read root package.json");
  process.exit(1);
}

if (!rootTsconfig) {
  error("Could not read root tsconfig.json");
  process.exit(1);
}

// Get all root dependencies (both deps and devDeps)
const rootDeps = {
  ...rootPkg.dependencies,
  ...rootPkg.devDependencies,
};

// Get tsconfig references as normalized paths
const tsconfigRefs = new Set(
  (rootTsconfig.references || []).map((ref) =>
    resolve(ROOT, ref.path).replace(/\/$/, ""),
  ),
);

// Find all workspace packages
const workspacePackages = await glob(
  ["frontend", "plugins/*"].map((p) => resolve(ROOT, p)),
  { onlyDirectories: true, absolute: true },
);

// Track which packages have tsconfig.json (TypeScript packages)
const tsPackages = [];

console.log("\n🔍 Checking package dependency sync...\n");

for (const pkgDir of workspacePackages) {
  const pkgJsonPath = resolve(pkgDir, "package.json");
  const tsconfigPath = resolve(pkgDir, "tsconfig.json");
  const pkgName = basename(pkgDir);

  if (!existsSync(pkgJsonPath)) {
    info(`${pkgName}: No package.json, skipping`);
    continue;
  }

  const pkg = readJson(pkgJsonPath);
  if (!pkg) {
    error(`${pkgName}: Could not read package.json`);
    continue;
  }

  const hasTsconfig = existsSync(tsconfigPath);
  if (hasTsconfig) {
    tsPackages.push({ path: pkgDir, name: pkgName, tsconfig: tsconfigPath });
  }

  // Check all dependencies in this package
  const allDeps = {
    ...pkg.dependencies,
    ...pkg.devDependencies,
  };

  for (const [dep, version] of Object.entries(allDeps)) {
    // Skip workspace protocol dependencies (internal packages)
    if (version.startsWith("workspace:")) {
      continue;
    }

    // Rule 1: If dep is at root, versions must match
    if (rootDeps[dep]) {
      if (rootDeps[dep] !== version) {
        error(
          `${pkgName}: ${dep}@${version} differs from root@${rootDeps[dep]}`,
        );
      }
    } else {
      // Rule 2: If dep is not at root, it should be declared there
      error(`${pkgName}: ${dep}@${version} is not declared at root`);
    }
  }
}

console.log("\n🔍 Checking TypeScript references...\n");

// Rule 3: TypeScript packages must be in root tsconfig references
for (const { path: pkgPath, name } of tsPackages) {
  if (tsconfigRefs.has(pkgPath)) {
    success(`${name}: Referenced in root tsconfig.json`);
  } else {
    error(`${name}: Has tsconfig.json but not referenced in root tsconfig`);
  }
}

// Rule 4: Check local tsconfig references for cross-module imports
console.log("\n🔍 Checking cross-module import references...\n");

// Build a map of package names to paths for workspace packages
const workspacePackageMap = new Map();
for (const pkgDir of workspacePackages) {
  const pkg = readJson(resolve(pkgDir, "package.json"));
  if (pkg?.name) {
    workspacePackageMap.set(pkg.name, pkgDir);
  }
}

for (const { path: pkgPath, name, tsconfig: tsconfigPath } of tsPackages) {
  const pkg = readJson(resolve(pkgPath, "package.json"));
  const tsconfig = readJson(tsconfigPath);

  if (!pkg || !tsconfig) continue;

  const allDeps = {
    ...pkg.dependencies,
    ...pkg.devDependencies,
  };

  // Get local tsconfig references
  const localRefs = new Set(
    (tsconfig.references || []).map((ref) =>
      resolve(pkgPath, ref.path).replace(/\/$/, ""),
    ),
  );

  // Check if any dependency is a workspace package
  for (const [dep, version] of Object.entries(allDeps)) {
    if (version.startsWith("workspace:") || workspacePackageMap.has(dep)) {
      const depPath = workspacePackageMap.get(dep);
      if (depPath && !localRefs.has(depPath)) {
        error(
          `${name}: Imports ${dep} but doesn't reference it in tsconfig.json`,
        );
      }
    }
  }
}

console.log("");

if (hasErrors) {
  console.error("❌ Sync check failed!\n");
  process.exit(1);
} else {
  console.log("✅ All checks passed!\n");
  process.exit(0);
}
