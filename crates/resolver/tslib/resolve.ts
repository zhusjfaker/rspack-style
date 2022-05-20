const { resolveFile } = require('../binding');

function resolve(filepath: string, import_path: string) {
  let haserr = false;
  let res;
  if (
    import_path.substring(0, 1) == '/' ||
    import_path.substring(0, 2) == './' ||
    import_path.substring(0, 3) == '../'
  ) {
    res = resolveFile(filepath, import_path);
  } else {
    try {
      res = resolveFile(filepath, import_path);
    } catch (ex) {
      haserr = true;
    }
    if (haserr) {
      res = resolveFile(filepath, `./${import_path}`);
    }
  }
  return res;
}

export { resolve };
