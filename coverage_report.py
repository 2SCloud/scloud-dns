import json

with open("coverage.json") as f:
    data = json.load(f)

md_lines = []
md_lines.append("| Filename | Function Coverage | Line Coverage | Region Coverage | Branch Coverage |")
md_lines.append("|---|---|---|---|---|")

for item in data["data"]:
    for file_cov in item["files"]:
        filename = file_cov["filename"]
        summary = file_cov["summary"]
        func_cov = f'{summary["functions"]["percent"]:.2f}% ({summary["functions"]["covered"]}/{summary["functions"]["count"]})'
        line_cov = f'{summary["lines"]["percent"]:.2f}% ({summary["lines"]["covered"]}/{summary["lines"]["count"]})'
        region_cov = f'{summary["regions"]["percent"]:.2f}% ({summary["regions"]["covered"]}/{summary["regions"]["count"]})'
        branch_cov = f'{summary["branches"]["percent"]:.2f}% ({summary["branches"]["covered"]}/{summary["branches"]["count"]})'

        md_lines.append(f"| {filename} | {func_cov} | {line_cov} | {region_cov} | {branch_cov} |")

with open("README.md", "a") as f:
    f.write("\n".join(md_lines))
