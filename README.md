# leetcode-rust-cn

LeetCode solutions in Rust, with Chinese comment. 用 Rust 刷 LeetCode，带中文注释

## VSCode

- 插件 [LeetCode](https://marketplace.visualstudio.com/items?itemName=LeetCode.vscode-leetcode)，插件用法见其[中文文档](https://github.com/LeetCode-OpenSource/vscode-leetcode/blob/master/docs/README_zh-CN.md)
- 插件 [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
- 插件 [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

用法：写好代码后，先点击 `Test`（这相当于在网页中点击 `Run Code`）来查看运行结果，确保显示的结果正确后再点击 `Submit`（这相当于在网页中点击 `Submit`）完成提交。

我的设置：

```json
"leetcode.defaultLanguage": "rust",
"leetcode.editor.shortcuts": ["submit", "test", "solution", "description"],
// leetcode 的工作区文件夹
"leetcode.workspaceFolder": "/mnt/SHARE/Rust_code/leetcode-rust-cn",
"leetcode.filePath": {
  "rust": {
    "folder": "src/solution",
    "filename": "s${id}_${snake_case_name}.${ext}"
  }
},
"leetcode.hint.commentDescription": false,
"leetcode.hint.configWebviewMarkdown": false,
"leetcode.hint.commandShortcut": false,
"rust-analyzer.debug.engine": "vadimcn.vscode-lldb",
"rust-analyzer.rustfmt.extraArgs": ["--config", "merge_imports=true"],
"rust-analyzer.updates.channel": "nightly",
```

## 参考

- [aylei/leetcode-rust](https://github.com/aylei/leetcode-rust)
