/** Truncates a Stellar address: GABCD...WXYZ */
export function formatAddress(address: string, chars = 4): string {
  if (address.length <= chars * 2 + 3) return address
  return `${address.slice(0, chars)}...${address.slice(-chars)}`
}

/** Formats a kWh value with unit */
export function formatKwh(kwh: number): string {
  if (kwh >= 1_000_000) return `${(kwh / 1_000_000).toFixed(2)} GWh`
  if (kwh >= 1_000) return `${(kwh / 1_000).toFixed(2)} MWh`
  return `${kwh.toFixed(2)} kWh`
}
