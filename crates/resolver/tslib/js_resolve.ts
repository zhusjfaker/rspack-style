import { CachedInputFileSystem, create } from 'enhanced-resolve';
import { ImportKind } from 'esbuild';
import fs from 'fs';

export const jsExtensions = ['.jsx', '.tsx', '.js', '.ts', '.vue', '.json'];
/**
 * supports require js plugin in less file
 */
export const cssExtensions = ['.less', '.css', '.sass', '.scss', '.js'];
export const createResolver = (options: {
  platform: NonNullable<'node' | 'browser'>;
  resolveType: 'css' | 'js';
  root: string;
  mainFields?: string[];
  mainFiles?: string[];
  alias?: Record<string, string>;
  preferRelative?: boolean;
  extensions?: string[];
}) => {
  const resolveCache = new Map<string, string>();
  const resolveOptions = {
    aliasFields: options.platform === 'browser' ? ['browser'] : [],
    FileSystem: new CachedInputFileSystem(fs, 4000),
    mainFields: options.mainFields ?? ['module', 'browser', 'main'],
    mainFiles: options.mainFiles,
    extensions: options.extensions,
    preferRelative: options.preferRelative,
    addMatchAll: false,
    plugins: [],
    alias: options.alias,
  };
  // conditionNames follow webpack options
  // cjs
  const resolveSync = create.sync({
    ...resolveOptions,
    conditionNames: ['require', 'module', '...'],
  });
  const esmResolveSync = create.sync({
    ...resolveOptions,
    conditionNames: ['import', 'module', '...'],
  });
  const node_resolve = (id: string, dir: string, kind?: ImportKind) => {
    if (resolveCache.get(id + dir + kind)) {
      return resolveCache.get(id + dir + kind) as string;
    }
    let result: string;
    try {
      if (options.resolveType === 'js') {
        if (kind === 'import-statement' || kind === 'dynamic-import') {
          result = esmResolveSync({}, dir, id) as string;
        } else {
          result = resolveSync({}, dir, id) as string;
        }
      } else {
        try {
          result = resolveSync({}, dir, id) as string;
        } catch (err) {
          result = resolveSync({}, dir, id.replace(/^~/, '')) as string;
        }
      }
      resolveCache.set(id + dir + kind, result);
      return result;
    } catch (err: any) {
      throw err;
    }
  };
  return node_resolve;
};

const css_resolve = (root: any) => {
  return createResolver({
    alias: {},
    extensions: cssExtensions,
    mainFields: ['module', 'browser', 'main'],
    mainFiles: ['index'],
    preferRelative: true,
    resolver: 'native-resolve',
    resolveType: 'css',
    root,
  } as any);
};

export default css_resolve;
