#!/usr/bin/env node

/**
 * Build tree-sitter grammars for multiple platforms.
 * This tool can fetch grammars and compile them using zig for cross-platform support.
 */

const fs = require('fs');
const path = require('path');
const { spawn, execSync } = require('child_process');
const os = require('os');

// Platform configurations
const PLATFORMS = {
  'linux-x86_64-glibc': {
    zig_target: 'x86_64-linux-gnu',
    rust_target: 'x86_64-unknown-linux-gnu',
  },
  'linux-x86_64-musl': {
    zig_target: 'x86_64-linux-musl',
    rust_target: 'x86_64-unknown-linux-musl',
  },
  'linux-aarch64-glibc': {
    zig_target: 'aarch64-linux-gnu',
    rust_target: 'aarch64-unknown-linux-gnu',
  },
  'linux-aarch64-musl': {
    zig_target: 'aarch64-linux-musl',
    rust_target: 'aarch64-unknown-linux-musl',
  },
  'windows-x86_64': {
    zig_target: 'x86_64-windows-gnu',
    rust_target: 'x86_64-pc-windows-gnu',
  },
  'windows-aarch64': {
    zig_target: 'aarch64-windows-gnu',
    rust_target: 'aarch64-pc-windows-gnu',
  },
  'macos-x86_64': {
    zig_target: 'x86_64-macos',
    rust_target: 'x86_64-apple-darwin',
  },
  'macos-aarch64': {
    zig_target: 'aarch64-macos',
    rust_target: 'aarch64-apple-darwin',
  },
};

function getCurrentPlatform() {
  const system = os.platform();
  const machine = os.arch();

  let platformName;
  if (system === 'darwin') {
    platformName = 'macos';
  } else if (system === 'win32') {
    platformName = 'windows';
  } else {
    platformName = system;
  }

  let arch;
  if (machine === 'x64') {
    arch = 'x86_64';
  } else if (machine === 'arm64') {
    arch = 'aarch64';
  } else {
    return null;
  }

  // Check for musl on Linux
  if (platformName === 'linux') {
    try {
      const lddOutput = execSync('ldd --version 2>&1', { encoding: 'utf8' });
      if (lddOutput.includes('musl')) {
        return `${platformName}-${arch}-musl`;
      } else {
        return `${platformName}-${arch}-glibc`;
      }
    } catch {
      return `${platformName}-${arch}-glibc`;
    }
  }

  return `${platformName}-${arch}`;
}

function grammarCacheDir(cacheDir, grammar) {
  if (!grammar.rev) {
    throw new Error(`Missing rev for grammar ${grammar.name}`);
  }
  return path.join(cacheDir, grammar.name, grammar.rev);
}

function checkZig() {
  try {
    const result = execSync('zig version', { encoding: 'utf8' });
    console.log(`Found zig: ${result.trim()}`);
    return true;
  } catch {
    throw new Error('Zig not found. Please install zig from https://ziglang.org/download/');
  }
}

async function runCommand(cmd, args, options = {}) {
  return new Promise((resolve, reject) => {
    const child = spawn(cmd, args, { ...options, stdio: 'pipe' });
    let stdout = '';
    let stderr = '';
    let finished = false;
    let timeoutId = null;

    function finalize(err) {
      if (finished) return;
      finished = true;
      if (timeoutId) {
        clearTimeout(timeoutId);
        timeoutId = null;
      }
      if (err) return reject(err);
      resolve({ stdout, stderr });
    }

    if (child.stdout) {
      child.stdout.on('data', (data) => {
        stdout += data.toString();
      });
    }

    if (child.stderr) {
      child.stderr.on('data', (data) => {
        stderr += data.toString();
      });
    }

    child.on('close', (code) => {
      if (code === 0) {
        finalize(null);
      } else {
        const error = new Error(`Command failed: ${cmd} ${args.join(' ')}`);
        error.code = code;
        error.stdout = stdout;
        error.stderr = stderr;
        finalize(error);
      }
    });

    child.on('error', (err) => finalize(err));

    // Set timeout if specified
    if (options.timeout) {
      timeoutId = setTimeout(() => {
        try { child.kill(); } catch {}
        finalize(new Error(`Command timeout after ${options.timeout}ms`));
      }, options.timeout);
    }
  });
}

async function fetchGrammar(grammar, cacheDir) {
  const name = grammar.name;
  const repo = grammar.repo;
  const grammarDir = grammarCacheDir(cacheDir, grammar);

  // Check if directory exists and has content
  if (fs.existsSync(grammarDir)) {
    const gitDir = path.join(grammarDir, '.git');
    if (fs.existsSync(gitDir) && fs.readdirSync(grammarDir).length > 0) {
      return { success: true, message: `${name} - already cached` };
    } else {
      // Directory exists but is empty or corrupted, remove it
      fs.rmSync(grammarDir, { recursive: true, force: true });
      console.log(`  Removing corrupted cache for ${name}`);
    }
  }

  fs.mkdirSync(path.dirname(grammarDir), { recursive: true });

  let repoUrl = repo;
  if (!repo.startsWith('http')) {
    repoUrl = `https://github.com/${repo}`;
  }

  console.log(`  Starting fetch: ${name} from ${repoUrl}`);

  try {
    // If we need a specific revision, we can't use --depth 1
    if (grammar.rev) {
      // Clone full repo with timeout
      await runCommand('git', ['clone', repoUrl, grammarDir], { timeout: 300000 }); // 5 minute timeout

      // Checkout specific revision
      try {
        await runCommand('git', ['checkout', grammar.rev], { cwd: grammarDir, timeout: 60000 });
      } catch (error) {
        // Clean up the clone if checkout fails
        fs.rmSync(grammarDir, { recursive: true, force: true });
        return { success: false, message: `${name} - ERROR: Revision ${grammar.rev.substring(0, 8)} not found: ${error.stderr}` };
      }
    } else {
      // Use shallow clone for branch or default
      const args = ['clone', '--depth', '1'];
      
      if (grammar.branch) {
        args.push('-b', grammar.branch);
      }
      
      args.push(repoUrl, grammarDir);
      await runCommand('git', args, { timeout: 300000 }); // 5 minute timeout
    }

    return { success: true, message: `${name} - cloned successfully` };
  } catch (error) {
    // Clean up partial clone
    if (fs.existsSync(grammarDir)) {
      fs.rmSync(grammarDir, { recursive: true, force: true });
    }
    
    if (error.message.includes('timeout')) {
      return { success: false, message: `${name} - ERROR: Clone timeout after 5 minutes` };
    }
    
    return { success: false, message: `${name} - ERROR: ${error.stderr || error.message}` };
  }
}

// Prefix symbols in an object file to avoid conflicts when combining grammars
async function prefixSymbolsInObject(objFile, grammarName) {
  const prefix = `ts_${grammarName}_`;
  const redefines = [];

  // Common tree-sitter external scanner functions that need prefixing
  const externalScannerFuncs = [
    'tree_sitter_javascript_external_scanner_create',
    'tree_sitter_javascript_external_scanner_destroy',
    'tree_sitter_javascript_external_scanner_scan',
    'tree_sitter_javascript_external_scanner_serialize',
    'tree_sitter_javascript_external_scanner_deserialize',
    'tree_sitter_typescript_external_scanner_create',
    'tree_sitter_typescript_external_scanner_destroy',
    'tree_sitter_typescript_external_scanner_scan',
    'tree_sitter_typescript_external_scanner_serialize',
    'tree_sitter_typescript_external_scanner_deserialize',
  ];

  // Internal symbols that commonly conflict between grammars
  const internalSymbols = [
    'sym_identifier_character_set_1',
    'sym_identifier_character_set_2',
    'sym_identifier_character_set_3',
  ];

  for (const func of externalScannerFuncs) {
    redefines.push(`--redefine-sym`, `${func}=${prefix}${func}`);
  }

  for (const sym of internalSymbols) {
    redefines.push(`--redefine-sym`, `${sym}=${prefix}${sym}`);
  }

  if (redefines.length > 0) {
    try {
      await runCommand('objcopy', redefines.concat(objFile));
    } catch (error) {
      // objcopy might fail if symbol doesn't exist or isn't global - that's ok
    }
  }
}

async function compileGrammar(grammar, cacheDir, platformDir, platformConfig) {
  const name = grammar.name;
  const grammarDir = grammarCacheDir(cacheDir, grammar);

  if (!fs.existsSync(grammarDir)) {
    return { success: false, message: `${name} - missing directory (expected ${path.relative(process.cwd(), grammarDir)})` };
  }

  // Determine source directory
  let srcDir;
  if (grammar.path) {
    srcDir = path.join(grammarDir, grammar.path, 'src');
  } else {
    srcDir = path.join(grammarDir, 'src');
  }

  const parserC = path.join(srcDir, 'parser.c');

  // If src directory doesn't exist, check if we can generate it
  if (!fs.existsSync(srcDir)) {
    // For grammars with a path, check for grammar.js in the subdirectory
    let grammarJs;
    if (grammar.path) {
      grammarJs = path.join(grammarDir, grammar.path, 'grammar.js');
    } else {
      grammarJs = path.join(grammarDir, 'grammar.js');
    }

    if (fs.existsSync(grammarJs)) {
      // Determine the working directory for generate command
      const genCwd = grammar.path ? path.join(grammarDir, grammar.path) : grammarDir;

      try {
        // Try with npx first
        await runCommand('npx', ['tree-sitter', 'generate'], { cwd: genCwd });
        // After generation, check if src directory was created
        if (!fs.existsSync(srcDir)) {
          return { success: false, message: `${name} - src directory not created after generate` };
        }
      } catch {
        // Try without npx
        try {
          await runCommand('tree-sitter', ['generate'], { cwd: genCwd });
          if (!fs.existsSync(srcDir)) {
            return { success: false, message: `${name} - src directory not created after generate` };
          }
        } catch {
          return { success: false, message: `${name} - no src directory and can't generate (install tree-sitter-cli)` };
        }
      }
    } else {
      return { success: false, message: `${name} - no src directory` };
    }
  }

  // If parser.c doesn't exist (even after potentially generating src dir), try to generate it
  if (!fs.existsSync(parserC)) {
    // For grammars with a path, check for grammar.js in the subdirectory
    let grammarJs;
    if (grammar.path) {
      grammarJs = path.join(grammarDir, grammar.path, 'grammar.js');
    } else {
      grammarJs = path.join(grammarDir, 'grammar.js');
    }

    if (fs.existsSync(grammarJs)) {
      // Determine the working directory for generate command
      const genCwd = grammar.path ? path.join(grammarDir, grammar.path) : grammarDir;

      try {
        // Try with npx first
        await runCommand('npx', ['tree-sitter', 'generate'], { cwd: genCwd });
        if (!fs.existsSync(parserC)) {
          return { success: false, message: `${name} - failed to generate parser.c` };
        }
      } catch {
        // Try without npx
        try {
          await runCommand('tree-sitter', ['generate'], { cwd: genCwd });
          if (!fs.existsSync(parserC)) {
            return { success: false, message: `${name} - failed to generate parser.c` };
          }
        } catch {
          return { success: false, message: `${name} - no parser.c and can't generate (install tree-sitter-cli)` };
        }
      }
    } else {
      return { success: false, message: `${name} - no parser.c` };
    }
  }

  // Check for scanner files
  const scannerC = path.join(srcDir, 'scanner.c');
  const scannerCc = path.join(srcDir, 'scanner.cc');
  const scannerCpp = path.join(srcDir, 'scanner.cpp');

  const sources = [parserC];
  let isCpp = false;

  if (fs.existsSync(scannerCc)) {
    sources.push(scannerCc);
    isCpp = true;
  } else if (fs.existsSync(scannerCpp)) {
    sources.push(scannerCpp);
    isCpp = true;
  } else if (fs.existsSync(scannerC)) {
    sources.push(scannerC);
  }

  // Output file
  const libName = `libtree-sitter-parsers-${name}.a`;
  const outputFile = path.join(platformDir, libName);

  // Compile object files - use grammar name as prefix to avoid conflicts
  const objFiles = [];
  
  for (const source of sources) {
    // Determine if this specific file is C++
    const sourceIsCpp = path.extname(source) === '.cc' || path.extname(source) === '.cpp';

    // Build command for this specific file - always use Zig
    const cmd = sourceIsCpp ? ['zig', 'c++'] : ['zig', 'cc'];
    cmd.push(
      '-target', platformConfig.zig_target,
      '-O3',
      '-fno-lto',
      '-c'
    );

    // Common flags
    cmd.push(
      '-I', srcDir,
      '-I', grammarDir,
      '-fPIC',
      '-fno-exceptions'
    );

    cmd.push(
      '-funroll-loops',
      '-fomit-frame-pointer',
      '-ffast-math',
      '-finline-functions',
      '-ffunction-sections',
      '-fdata-sections',
      '-fvisibility=hidden'
    );

    // Add grammar-specific defines to avoid symbol conflicts
    cmd.push(
      `-Dstring_new=ts_${name}_string_new`,
      `-Dscan_comment=ts_${name}_scan_comment`,
      `-Dserialize=ts_${name}_serialize`,
      `-Ddeserialize=ts_${name}_deserialize`,
      `-Dscan=ts_${name}_scan`
    );

    // Grammar-specific compile flags
    if (name === 'just') {
      // just grammar requires assertions to be enabled
      cmd.push('-UNDEBUG');
    }

    if (sourceIsCpp) {
      cmd.push('-std=c++14');
    } else {
      // For C files, use gnu11 to support static_assert and GNU extensions
      cmd.push('-std=gnu11');
    }

    const objFile = path.join(platformDir, `${name}_${path.basename(source, path.extname(source))}.o`);
    const compileCmd = cmd.concat([source, '-o', objFile]);

    try {
      await runCommand(compileCmd[0], compileCmd.slice(1));

      // Rename symbols to avoid conflicts when combining grammars
      await prefixSymbolsInObject(objFile, name);

      objFiles.push(objFile);
    } catch (error) {
      // Clean up any object files
      for (const obj of objFiles) {
        try { fs.unlinkSync(obj); } catch {}
      }
      const errorDetails = error.stderr || error.stdout || error.message || 'Unknown error';
      const fullCommand = compileCmd.join(' ');
      return { success: false, message: `${name} - compile error:\nCommand: ${fullCommand}\nError: ${errorDetails}` };
    }
  }

  // Create static library using zig ar
  const arCmd = ['zig', 'ar', 'rcs', outputFile].concat(objFiles);

  try {
    await runCommand(arCmd[0], arCmd.slice(1));

    // Clean up object files
    for (const obj of objFiles) {
      fs.unlinkSync(obj);
    }

    // Note if this grammar uses C++ (for build.rs metadata)
    if (isCpp) {
      fs.writeFileSync(path.join(platformDir, `${name}.cpp`), '');
    }

    return { success: true, message: `${name} - compiled successfully` };
  } catch (error) {
    // Clean up
    for (const obj of objFiles) {
      try { fs.unlinkSync(obj); } catch {}
    }
    try { fs.unlinkSync(outputFile); } catch {}
    const errorDetails = error.stderr || error.stdout || error.message || 'Unknown error';
    const fullCommand = arCmd.join(' ');
    return { success: false, message: `${name} - ar error:\nCommand: ${fullCommand}\nError: ${errorDetails}` };
  }
}

function generateMetadata(compiledGrammars, grammarsConfig, platformDir) {
  const metadataFile = path.join(platformDir, 'grammars.json');
  // Save full grammar objects for compiled grammars
  const compiledGrammarObjects = grammarsConfig.filter(g => compiledGrammars.includes(g.name));
  fs.writeFileSync(
    metadataFile,
    JSON.stringify(compiledGrammarObjects.sort((a, b) => a.name.localeCompare(b.name)), null, 2)
  );
}

// Parallel processing utilities
async function runInParallel(items, workerFn, maxWorkers) {
  const results = [];
  const queue = [...items];
  const workers = [];

  for (let i = 0; i < Math.min(maxWorkers, items.length); i++) {
    workers.push(processQueue());
  }

  async function processQueue() {
    while (queue.length > 0) {
      const item = queue.shift();
      const result = await workerFn(item);
      results.push(result);
      
      // Progress reporting
      const completed = results.length;
      const total = items.length;
      console.log(`  [${completed}/${total}] ${result.message}`);
      
      if (!result.success) {
        throw new Error(result.message);
      }
    }
  }

  await Promise.all(workers);
  return results;
}

// Command-line argument parsing
function parseArgs() {
  const args = process.argv.slice(2);
  const options = {
    fetchOnly: false,
    compileOnly: false,
    platform: null,
    allPlatforms: false,
    jobs: os.cpus().length
  };

  for (let i = 0; i < args.length; i++) {
    switch (args[i]) {
      case '--fetch-only':
        options.fetchOnly = true;
        break;
      case '--compile-only':
        options.compileOnly = true;
        break;
      case '--platform':
        options.platform = args[++i];
        break;
      case '--all-platforms':
        options.allPlatforms = true;
        break;
      case '-j':
      case '--jobs':
        options.jobs = parseInt(args[++i]);
        break;
      case '-h':
      case '--help':
        console.log(`Usage: ${path.basename(process.argv[1])} [options]

Options:
  --fetch-only        Only fetch grammars, do not compile
  --compile-only      Only compile, assume grammars are fetched
  --platform PLATFORM Target platform (default: current platform)
  --all-platforms     Build for all platforms (requires zig)
  -j, --jobs N        Number of parallel jobs (default: CPU count)
  -h, --help          Show this help message`);
        process.exit(0);
    }
  }

  return options;
}

async function main() {
  const options = parseArgs();

  // Repo root (this script lives in ./scripts)
  const repoRoot = path.resolve(__dirname, '..');

  // Input: enriched mapping file (use as-is)
  const grammarsJson = path.join(repoRoot, 'grammars-mapping-filetypes.json');

  // Cache/output locations
  const cacheDir = process.env.SYNTASTICA_PARSERS_CLONE_DIR || path.join(repoRoot, '.cache/syntastica/parsers/clone');
  const precompiledDir = process.env.SYNTASTICA_PARSERS_CACHE_DIR || path.join(repoRoot, '.cache/syntastica/parsers/cache');

  if (!fs.existsSync(grammarsJson)) {
    console.error(`Error: ${grammarsJson} not found`);
    process.exit(1);
  }

  // Load grammar mappings (enriched list)
  const mappings = JSON.parse(fs.readFileSync(grammarsJson, 'utf8'));

  // De-dupe by grammar name
  const byName = new Map();
  for (const m of mappings) {
    if (!m || !m.grammar || !m.grammar_repo || !m.grammar_rev) continue;
    if (!byName.has(m.grammar)) {
      byName.set(m.grammar, {
        name: m.grammar,
        repo: m.grammar_repo,
        rev: m.grammar_rev,
        branch: m.grammar_branch || undefined,
        path: m.grammar_path || undefined,
      });
    }
  }

  const grammars = Array.from(byName.values());
  console.log(`Found ${grammars.length} grammars`);

  // Fetch grammars if needed
  if (!options.compileOnly) {
    console.log('\n=== Fetching grammars ===');
    if (!fs.existsSync(cacheDir)) {
      fs.mkdirSync(cacheDir, { recursive: true });
    }

    // Check which grammars need fetching
    const needFetch = [];
    for (const grammar of grammars) {
      const grammarDir = grammarCacheDir(cacheDir, grammar);
      if (!fs.existsSync(grammarDir) || !fs.existsSync(path.join(grammarDir, '.git'))) {
        needFetch.push(grammar.name);
      }
    }

    if (needFetch.length > 0) {
      console.log(`  Need to fetch: ${needFetch.join(', ')}`);
    }

    console.log(`  Processing ${grammars.length} grammars with ${options.jobs} parallel jobs...`);

    try {
      await runInParallel(grammars, (grammar) => fetchGrammar(grammar, cacheDir), options.jobs);
    } catch (error) {
      console.error(error.message);
      process.exit(1);
    }
  }

  if (options.fetchOnly) {
    console.log('\nFetch complete!');
    return;
  }

  // Check that Zig is installed
  checkZig();

  // Determine platforms to build
  let platformsToBuild;
  if (options.allPlatforms) {
    platformsToBuild = Object.keys(PLATFORMS);
  } else if (options.platform) {
    if (!PLATFORMS[options.platform]) {
      console.error(`Error: Unknown platform ${options.platform}`);
      console.error(`Available platforms: ${Object.keys(PLATFORMS).join(', ')}`);
      process.exit(1);
    }
    platformsToBuild = [options.platform];
  } else {
    const current = getCurrentPlatform();
    if (current && PLATFORMS[current]) {
      platformsToBuild = [current];
    } else {
      console.error('Error: Could not detect current platform or it\'s not in the supported list');
      console.error(`Available platforms: ${Object.keys(PLATFORMS).join(', ')}`);
      process.exit(1);
    }
  }

  // Compile for each platform
  for (const platformName of platformsToBuild) {
    console.log(`\n=== Building for ${platformName} ===`);

    const platformDir = path.join(precompiledDir, platformName);
    if (!fs.existsSync(platformDir)) {
      fs.mkdirSync(platformDir, { recursive: true });
    }

    const platformConfig = PLATFORMS[platformName];
    if (!platformConfig) {
      console.error(`Error: No configuration found for platform ${platformName}`);
      process.exit(1);
    }

    const compiledGrammars = [];
    const failedGrammars = [];

    console.log(`  Compiling ${grammars.length} grammars with ${options.jobs} parallel jobs...`);

    const results = await runInParallel(
      grammars,
      (grammar) => compileGrammar(grammar, cacheDir, platformDir, platformConfig),
      options.jobs
    );

    for (let i = 0; i < results.length; i++) {
      if (results[i].success) {
        compiledGrammars.push(grammars[i].name);
      } else {
        failedGrammars.push(grammars[i].name);
      }
    }

    // Generate metadata
    generateMetadata(compiledGrammars, grammars, platformDir);

    // Combine all static libraries into a single archive
    if (compiledGrammars.length > 0) {
      console.log(`\n  Combining ${compiledGrammars.length} libraries into single archive...`);

      // Collect all library files
      const libFiles = [];
      for (const grammarName of compiledGrammars) {
        const libFile = path.join(platformDir, `libtree-sitter-parsers-${grammarName}.a`);
        if (fs.existsSync(libFile)) {
          libFiles.push(libFile);
        }
      }

      if (libFiles.length > 0) {
        // Create combined library name
        const combinedLib = path.join(precompiledDir, `libtree-sitter-parsers-all-${platformName}.a`);

        // First, extract all object files from all archives
        const tempObjDir = path.join(platformDir, 'temp_objects');
        if (!fs.existsSync(tempObjDir)) {
          fs.mkdirSync(tempObjDir, { recursive: true });
        }

        for (let i = 0; i < libFiles.length; i++) {
          // Extract to a unique subdirectory to avoid name conflicts
          const extractDir = path.join(tempObjDir, `lib_${i}`);
          if (!fs.existsSync(extractDir)) {
            fs.mkdirSync(extractDir, { recursive: true });
          }

          // Extract objects using zig ar
          await runCommand('zig', ['ar', 'x', libFiles[i]], { cwd: extractDir });
        }

        // Collect all object files
        const allObjects = [];
        const dirs = fs.readdirSync(tempObjDir);
        for (const dir of dirs) {
          const objDir = path.join(tempObjDir, dir);
          if (fs.statSync(objDir).isDirectory()) {
            const objects = fs.readdirSync(objDir)
              .filter(f => f.endsWith('.o'))
              .map(f => path.join(objDir, f));
            allObjects.push(...objects);
          }
        }

        // Create the combined archive using zig ar
        const arCmd = ['zig', 'ar', 'rcs', combinedLib].concat(allObjects);

        try {
          await runCommand(arCmd[0], arCmd.slice(1));
          console.log(`  Created combined archive: ${path.basename(combinedLib)}`);

          // Strip debug symbols from combined archive
          console.log(`  Stripping debug symbols...`);
          await runCommand('strip', ['--strip-debug', combinedLib]);

          // Clean up temporary files
          fs.rmSync(tempObjDir, { recursive: true, force: true });

          // Remove individual library files
          for (const libFile of libFiles) {
            fs.unlinkSync(libFile);
          }

          // Move metadata file to precompiled directory with platform suffix
          const metadataSrc = path.join(platformDir, 'grammars.json');
          const metadataDst = path.join(precompiledDir, `grammars-${platformName}.json`);
          if (fs.existsSync(metadataSrc)) {
            fs.renameSync(metadataSrc, metadataDst);
          }

          // Remove the now-empty platform directory
          fs.rmSync(platformDir, { recursive: true, force: true });

        } catch (error) {
          console.error(`  ERROR: Failed to create combined archive: ${error.stderr}`);
          fs.rmSync(tempObjDir, { recursive: true, force: true });
        }
      }
    }

    console.log(`\nPlatform ${platformName} summary:`);
    console.log(`  Compiled: ${compiledGrammars.length} grammars`);
    if (failedGrammars.length > 0) {
      console.log(`  Failed: ${failedGrammars.length} grammars`);
      console.log(`    ${failedGrammars.join(', ')}`);
    }
    if (compiledGrammars.length > 0) {
      console.log(`  Output: ${precompiledDir}/libtree-sitter-parsers-all-${platformName}.a`);
    }
  }

  console.log('\nBuild complete!');
  console.log('\nTo use the precompiled grammars:');
  console.log('1. Make sure your Cargo.toml uses: build = "build.rs"');
  console.log('2. The build.rs will automatically detect and use the precompiled binaries');
}

// Run the main function
main().catch(error => {
  console.error('Error:', error);
  process.exit(1);
});
