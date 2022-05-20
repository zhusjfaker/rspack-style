import {build} from "esbuild";

build({
    entryPoints: ["tslib/index.ts"],
    outfile: "dist/main.js",
    minify: false,
    platform: "node",
    target: "es2020",
    bundle: true,
    sourcemap: true,
    loader: {
        '.tsx': 'tsx',
        '.ts': 'ts',
        '.js': 'js',
        '.node': 'binary'
    },
    external:[
      "pnpapi"
    ]
}).catch((ex) => {
    console.log(ex);
    process.exit(1)
});