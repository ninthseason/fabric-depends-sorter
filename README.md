⚠ Because dependent relationship in fabric.mod.json is not reliable, this tool is almost not working.

⚠ 由于 fabric.mod.json 文件中由模组作者标注的依赖关系常常不全面，该工具几乎不可用。

# fabric-depends-sorter

English:

The tool can rename mods in directory with `<SerialNumber>_<OriginName>.jar` where `SerialNumber` is the toposort result(in dependency graph) of the mod.

Then you can manually perform a binary search to identify the conflicting modules without worrying about dependency issues.

中文说明:

该工具会将指定目录中的模组重命名为`<SerialNumber>_<OriginName>.jar`，其中`SerialNumber`为该模组于依赖图中的拓扑排序序号。

接下来你可以手动使用二分查找以定位产生冲突的模组，无需考虑模组间依赖的问题。

换言之，你可以安全移除序号更高的模组而无需担心存在序号更低的模组依赖于它。


## Usage

```bash
Usage: fabric-depends-sorter.exe [OPTIONS] <DIRECTORY>

Arguments:
  <DIRECTORY>  mods directory

Options:
  -x, --threads <THREADS>  Number of threads [default: 1]
  -i, --ignores <IGNORES>  Ignore mods [default: []]
  -h, --help               Print help
  -V, --version            Print version
```

## Example

```bash
$./fabric-depends-sorter -x16 -i fabric-api.jar ./
```
