import less from "less";
import LessAliasesPlugin from "./plugin";

function get_argv(key: string) {
    let list = process.argv;
    let index = list.findIndex((p) => {
        return p == "--" + key
    })
    if (index > -1) {
        return process.argv[index + 1]
    }
}

async function main() {
    let content = get_argv("content");
    let option_value = get_argv("option");
    let options = undefined;
    if (option_value) {
        options = JSON.parse(option_value);
    }
    if (content) {
        content = JSON.parse(content)?.content;
    }
    if (content && options?.filename) {
        return handle(content, options)
    }
}

export function handle(content: string, options: any) {
    if (!options.filename) {
        console.log("options.filename must not be empty");
        process.exit(1);
    }

    let callback_error = (err: string) => {
        console.log("resolve", options.filename, "-> has error \n", err);
        process.exit(1);
    }

    less.render(content, {
      paths: [
        ...(options?.paths || ['node_modules']),
        ...(options?.root ? [options.root] : []),
      ],
      plugins: [new LessAliasesPlugin(options.filename, callback_error)]
    }).then(res => {
      process.stdout.write(res.css);
    }).catch(ex => {
      console.log(ex);
      setTimeout(() => {
        process.exit(1);  
      }, 500);
    })
}



main();


