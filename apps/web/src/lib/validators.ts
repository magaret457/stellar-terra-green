import { z } from 'zod'

export const meterReadingSchema = z.object({
  meter_id: z.string().uuid(),
  kwh: z.number().positive(),
  timestamp: z.string().datetime(),
  api_key: z.string().min(32),
})

export type MeterReading = z.infer<typeof meterReadingSchema>
