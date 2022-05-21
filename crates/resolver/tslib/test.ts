import {handle} from "./index";

function test() {
    type fs_type = typeof import("fs");
    const fs: fs_type = require('fs');
    const filepath = "/Users/zhushijie/Desktop/github/rspack-style/crates/style/assets/arco-pro/3.less";
    let content = fs.readFileSync(filepath).toString("utf8");
    let options = {
        filename: filepath,
        paths: ['/Users/zhushijie/Desktop/github/rspack-style/crates/style/assets/arco-pro', 'node_modules']
    };
    handle(content, options);
}


test();




