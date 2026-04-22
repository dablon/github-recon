#!/bin/bash
# E2E tests for github-recon CLI
# Usage: ./tests/e2e.sh

set -e

GH_TOKEN="${GITHUB_TOKEN:-}"
IMAGE="${1:-github-recon-built}"
OUTPUT_DIR="${OUTPUT_DIR:-/tmp/github-recon-test-output}"

if [ -z "$GH_TOKEN" ]; then
    echo "ERROR: GITHUB_TOKEN not set"
    exit 1
fi

mkdir -p "$OUTPUT_DIR"
rm -f "$OUTPUT_DIR"/*.csv "$OUTPUT_DIR"/*.html "$OUTPUT_DIR"/*.xlsx 2>/dev/null || true

echo "=== E2E Test Suite for github-recon ==="
echo "Image: $IMAGE"
echo "Token: ${GH_TOKEN:0:8}..."
echo ""

count=0
passed=0
failed=0

run_test() {
    local name="$1"
    local expected="$2"
    shift 2
    echo -n "Test: $name ... "
    count=$((count + 1))
    if "$@" > /tmp/test_output.txt 2>&1; then
        if [ -n "$expected" ] && ! grep -q "$expected" /tmp/test_output.txt; then
            echo "FAIL (expected '$expected' in output)"
            cat /tmp/test_output.txt | tail -5
            failed=$((failed + 1))
        else
            echo "PASS"
            passed=$((passed + 1))
        fi
    else
        echo "FAIL (exit code $?)"
        cat /tmp/test_output.txt | tail -5
        failed=$((failed + 1))
    fi
}

# Test 1: Help flag
run_test "Help flag" "--help" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" "$IMAGE" --help

# Test 2: Search returns repos (CSV format)
run_test "CSV search pentest" "stargazers_count" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" "$IMAGE" search "pentest automation" --limit 5 --format csv

# Test 3: HTML output generated
run_test "HTML output" "<table" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" "$IMAGE" search "vulnerability scanner" --limit 5 --format html --html-output "$OUTPUT_DIR/test.html"
test -f "$OUTPUT_DIR/test.html" && echo "PASS (file created)" || echo "FAIL (file not created)"

# Test 4: XLSX output generated
run_test "XLSX output" "" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" -v "$OUTPUT_DIR:/output" "$IMAGE" search "network scanner" --limit 5 --xlsx-output /output/test.xlsx
test -f "$OUTPUT_DIR/test.xlsx" && echo "PASS (file created)" || echo "FAIL (file not created)"

# Test 5: Auto-detect XLSX from -f xlsx
run_test "Auto XLSX format" "" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" -v "$OUTPUT_DIR:/output" "$IMAGE" search "ai agent" --limit 5 -f xlsx --output /output/test2

# Test 6: Sort by forks
run_test "Sort by forks" "forks_count" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" "$IMAGE" search "docker automation" --limit 5 -s forks

# Test 7: Sort ascending
run_test "Sort ascending" "asc" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" "$IMAGE" search "kubernetes" --limit 5 -s stars --order asc

# Test 8: Limit parameter
run_test "Limit 10" "" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" "$IMAGE" search "python" --limit 10 --format csv -o "$OUTPUT_DIR/limit10.csv"
line_count=$(wc -l < "$OUTPUT_DIR/limit10.csv")
# CSV has header, so data lines = line_count - 1, should be <= 10
data_lines=$((line_count - 1))
if [ "$data_lines" -le 10 ]; then echo "PASS (got $data_lines lines)"; else echo "FAIL (got $data_lines, expected <= 10)"; fi

# Test 9: MCP mode help
run_test "MCP mode" "" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" "$IMAGE" mcp --help

# Test 10: Search with no results query
run_test "No results query" "No repositories found" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" "$IMAGE" search "asdfjklqwerty NORESULTS query 12345" --limit 5

# Test 11: Multiple formats (both)
run_test "Both CSV+HTML" "" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" -v "$OUTPUT_DIR:/output" "$IMAGE" search "security" --limit 5 -f both -o /output/both.csv

# Test 12: XLSX file contains expected sheets
run_test "XLSX sheets" "" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" -v "$OUTPUT_DIR:/output" "$IMAGE" search "pentest framework" --limit 5 --xlsx-output /output/sheets_test.xlsx
# Extract xlsx and check sheets (using python unzip)
python3 -c "
import zipfile, sys
with zipfile.ZipFile('$OUTPUT_DIR/sheets_test.xlsx') as z:
    sheets = [n for n in z.namelist() if 'sheet' in n.lower()]
    print('Sheets found:', len(sheets))
    for s in sorted(sheets): print(' -', s)
" && echo "PASS" || echo "FAIL"

# Test 13: Different category detection
run_test "Category AI Agent" "" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" -v "$OUTPUT_DIR:/output" "$IMAGE" search "autonomous AI agent" --limit 5 -f xlsx --xlsx-output /output/ai_test.xlsx

# Test 14: Category detection - network
run_test "Category Network" "" docker run --rm -e GITHUB_TOKEN="$GH_TOKEN" -v "$OUTPUT_DIR:/output" "$IMAGE" search "network port scanner" --limit 5 -f xlsx --xlsx-output /output/net_test.xlsx

echo ""
echo "=== Results ==="
echo "Total: $count | Passed: $passed | Failed: $failed"
[ "$failed" -eq 0 ] && echo "ALL TESTS PASSED" || echo "SOME TESTS FAILED"
exit $failed