# Agent Instructions for Morningstar ID Integration

## Project Overview

This project involves adding Morningstar fund IDs to a CSV file containing
investment fund data. The CSV contains 143 funds with various data points,
and the goal is to add a "Morningstar ID" column populated with the
corresponding Morningstar fund identifiers.

## File Structure

- **Working file**: `Excel Pizarritos - General.csv` (contains Morningstar data)

## CSV Structure

The CSV contains these columns:

1. Isin
2. Nombre del Fondo
3. Precio Actual
4. Rentabilidad Año en Curso (%)
5. Categoría General
6. Riesgo (1-7)
7. Volatilidad
8. Ratio Sharpe
9. Finect
10. Fecha de Actualización Rentabilidad
11. Usuario
12. **Morningstar ID** (newly added)
13. **Morningstar URL** (newly added)
## Morningstar ID Search Method

### Search Strategy

1. **Use ISIN codes for searching** - This is more accurate than fund names
2. **Search pattern**: `{ISIN} morningstar` in Google
3. **Look for Morningstar URLs** in search results that contain fund IDs
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

1. Navigate to Google search: `https://www.google.com/search?q={ISIN}+morningstar`
2. Look for Morningstar results in search results
3. Extract the Morningstar ID from URLs (usually in `/fondos/{ID}/` or
   `?id={ID}` patterns)
4. Verify the ID corresponds to the correct fund by checking fund name
   matches
5. Add both the Morningstar ID and the corresponding URL to the CSV
## URL Construction

Once you have a Morningstar ID, construct the full URL using these patterns:

- **For regular funds**: `https://global.morningstar.com/es/inversiones/fondos/{MORNINGSTAR_ID}/cotizacion`
- **For ETFs**: `https://global.morningstar.com/en-eu/investments/etfs/{MORNINGSTAR_ID}/quote`
- **Alternative format**: `https://www.morningstar.es/es/funds/snapshot/snapshot.aspx?id={MORNINGSTAR_ID}`

The URL should be added to the "Morningstar URL" column (column 13)
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
## Files to Maintain

- Update the main working file with new IDs as they are found
- Document any issues or special cases encountered during searches
- Use git for version control and history
