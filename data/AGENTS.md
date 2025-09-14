# Agent Instructions for Morningstar ID Integration

## Project Overview

This project involves adding Morningstar fund IDs to a CSV file containing
investment fund data. The CSV contains 143 funds with various data points,
and the goal is to add a "Morningstar ID" column populated with the
corresponding Morningstar fund identifiers.

## File Structure

- **Working file**: `data/funds.csv` (contains Morningstar data)

## CSV Structure

The CSV contains these columns:

1. Isin
2. Nombre del Fondo
3. Categoría General
4. Riesgo (1-7)
5. Volatilidad
6. Ratio Sharpe
7. **Morningstar ID**
8. **Morningstar URL**
## Morningstar ID Search Method

### Search Strategy

1. **Use ISIN codes for searching** - This is more accurate than fund names
2. **Search pattern**: `{ISIN} morningstar` using **NATIVE WebSearch tool ONLY** 
3. **⚠️ CRITICAL: ALWAYS use the native WebSearch tool - NEVER use Brave search tools (mcp__brave__*)**
4. **Look for Morningstar URLs** in search results that contain fund IDs
5. **Preferred URL format**: `https://global.morningstar.com/es/inversiones/fondos/{MORNINGSTAR_ID}/cotizacion`

### ⚠️ SEARCH TOOL REQUIREMENTS
- **MUST USE**: WebSearch tool (native Claude tool)
- **NEVER USE**: mcp__brave__brave_web_search or any mcp__brave__* tools
- **This is mandatory and has been proven to work reliably**
### Morningstar ID Format

- Morningstar IDs are 10-character alphanumeric codes
- Examples: `F0GBR04NQN`, `F000010KY6`, `F00000YM8X`
### URL Pattern

Morningstar fund pages follow this pattern:

- Spanish: `https://global.morningstar.com/es/inversiones/fondos/{MORNINGSTAR_ID}/cotizacion`
- English: `https://global.morningstar.com/en-eu/investments/funds/{MORNINGSTAR_ID}/quote`
- Alternative: `https://www.morningstar.es/es/funds/snapshot/snapshot.aspx?id={MORNINGSTAR_ID}`
## Confirmed Mappings

The following ISIN → Morningstar ID mappings have been verified:

| ISIN | Fund Name | Morningstar ID |
|------|-----------|----------------|
| LU0034353002 | DWS Floating Rate Notes | F0GBR04NQN |
| FR0000989626 | Groupama Trésorerie | F0GBR04M6M |
| LU1508158430 | BlackRock Strategic Asia Pacific Equity Absolute Return | F00000YM8X |
| ES0146309002 | Horos Value Internacional FI | F000010KY6 |
| ES0141116030 | Hamco Global Value Fund FI | F000014ACV |
| LU1190417599 | Multi Units Luxembourg - Amundi Smart Overnight Return | 0P0001UPP5 |
| LU0352158918 | UBAM - Dynamic US Dollar Bond | F000005M84 |
| FR0011088657 | Amundi Ultra Short Term Bond SRI | F00000N879 |
| LU2482630675 | European Specialist Investment Funds - M&G Total Return Credit Investment | F00001DZCX |
| IE00BFZMJT78 | Neuberger Berman Short Duration Euro Bond Fund | F000011OIS |
| LU0607220562 | Schroder International Selection Fund EURO Corporate Bond | F00000MG0P |

## Search Process

1. Use native WebSearch tool with query: `{ISIN} morningstar`
2. Look for Morningstar URLs in search results, specifically the `/inversiones/fondos/` format
3. Extract Morningstar ID from URLs (10-character alphanumeric codes)
4. Verify fund name matches in results
5. Add both the Morningstar ID and corresponding URL to the CSV

**WebSearch Success Examples:**
- IE00BFZMJT78 → F000011OIS (Neuberger Berman Short Duration Euro Bond Fund)
- FR0000989626 → F0GBR04M6M (Groupama Trésorerie)
- ES0138534054 → F00000WINH (Santander Rendimiento FI) 
- IE00B03HD191 → F0GBR052TN (Vanguard Global Stock Index Fund)
- IE0031786142 → F0GBR06TSA (Vanguard Emerging Markets Stock Index Fund)

**Notes on WebSearch method:**
- More reliable and consistent than previous search methods
- Excellent results for European funds (ISIN codes starting with FR, ES, LU, IE)
- Look specifically for URLs with `/inversiones/fondos/{ID}/cotizacion` pattern
## URL Construction

Once you have a Morningstar ID, construct the full URL using these patterns:

- **For regular funds**: `https://global.morningstar.com/es/inversiones/fondos/{MORNINGSTAR_ID}/cotizacion`
- **For ETFs**: `https://global.morningstar.com/en-eu/investments/etfs/{MORNINGSTAR_ID}/quote`
- **Alternative format**: `https://www.morningstar.es/es/funds/snapshot/snapshot.aspx?id={MORNINGSTAR_ID}`

The URL should be added to the "Morningstar URL" column (column 8)
immediately after finding and verifying the Morningstar ID.
## Important Notes

- **Multiple share classes**: Some funds may have multiple Morningstar IDs
  for different share classes
- **Missing funds**: Some funds may not be available on Morningstar
- **Regional differences**: European funds may appear on different
  Morningstar regional sites
- **Search reliability**: ISIN-based searches are more reliable than
  name-based searches
## Future Work

- 138 funds still need Morningstar IDs
- Consider batch processing for efficiency
- Prioritize high-value or frequently accessed funds
- Implement error handling for funds not found on Morningstar
## CRITICAL: How to Find Missing Morningstar IDs

**IMPORTANT LESSON LEARNED**: Bash commands for counting missing IDs are unreliable and will give wrong results.

### The ONLY Reliable Method:

1. **Use the Read tool to manually scan the CSV file in sections**
   - Use `Read` with `offset` and `limit` parameters to view chunks of the CSV
   - Example: `Read file_path="/path/to/funds.csv" offset=100 limit=44`
   - Visually scan each line for entries ending with `,,` (two commas = missing ID and URL)

2. **DO NOT rely on bash commands like:**
   - `awk`, `grep`, or `wc` commands - they consistently fail
   - Automatic counting methods - they are unreliable for this CSV format
   - Pattern matching with bash - the CSV structure confuses these tools

3. **The correct way to identify missing IDs:**
   - Look for lines ending with `,,` (two consecutive commas at the end)
   - These indicate missing both Morningstar ID (column 7) and URL (column 8)
   - Manual visual inspection is the ONLY reliable method

### Example of what missing IDs look like:
```
LU0113257694,Schroder International Selection Fund EURO Corporate Bond,RF Deuda Corporativa EUR,4,"2,66","1,2",,
```
The `,,` at the end shows missing Morningstar ID and URL.

**This method was the only one that worked after 100+ failed attempts with bash commands.**

## Files to Maintain

- Update the main working file with new IDs as they are found
- Document any issues or special cases encountered during searches
- Use git for version control and history
