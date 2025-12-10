import json

with open("coverage.json") as f:
    data = json.load(f)

md_lines = []
md_lines.append("| Filename | Function Coverage | Line Coverage | Region Coverage | Branch Coverage |")
md_lines.append("|---|---|---|---|---|")

for file_cov in data["files"]:
    filename = file_cov["filename"]
    func_cov = f'{file_cov["coverage"]["function"]["percent"]:.2f}% ({file_cov["coverage"]["function"]["covered"]}/{file_cov["coverage"]["function"]["total"]})'
    line_cov = f'{file_cov["coverage"]["line"]["percent"]:.2f}% ({file_cov["coverage"]["line"]["covered"]}/{file_cov["coverage"]["line"]["total"]})'
    region_cov = f'{file_cov["coverage"]["region"]["percent"]:.2f}% ({file_cov["coverage"]["region"]["covered"]}/{file_cov["coverage"]["region"]["total"]})'
    branch_cov = f'{file_cov["coverage"]["branch"]["percent"]:.2f}% ({file_cov["coverage"]["branch"]["covered"]}/{file_cov["coverage"]["branch"]["total"]})'

    md_lines.append(f"| {filename} | {func_cov} | {line_cov} | {region_cov} | {branch_cov} |")

total = data["totals"]
total_line = f"| Totals | {total['function']['percent']:.2f}% ({total['function']['covered']}/{total['function']['total']}) | {total['line']['percent']:.2f}% ({total['line']['covered']}/{total['line']['total']}) | {total['region']['percent']:.2f}% ({total['region']['covered']}/{total['region']['total']}) | {total['branch']['percent']:.2f}% ({total['branch']['covered']}/{total['branch']['total']}) |"
md_lines.append(total_line)

with open("README.md", "a") as f:
    f.write("\n".join(md_lines))
