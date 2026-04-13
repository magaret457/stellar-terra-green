import { NextRequest, NextResponse } from 'next/server'
import { createClient } from '@/lib/supabase/server'
import { mintCertificates } from '@/lib/stellar/mint'
import { meterReadingSchema } from '@/lib/validators'

/**
 * POST /api/meters/readings
 * Accepts a smart meter reading and mints proto-certificates on-chain.
 */
export async function POST(req: NextRequest) {
  const body = await req.json()
  const parsed = meterReadingSchema.safeParse(body)

  if (!parsed.success) {
    return NextResponse.json({ error: parsed.error.flatten() }, { status: 400 })
  }

  const { meter_id, kwh, timestamp, api_key } = parsed.data

  const supabase = await createClient()

  // Validate API key against meter record
  const { data: meter, error: meterError } = await supabase
    .from('meters')
    .select('id, active, cooperative_id')
    .eq('id', meter_id)
    .eq('api_key', api_key)
    .single()

  if (meterError || !meter) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 })
  }

  if (!meter.active) {
    return NextResponse.json({ error: 'Meter is inactive' }, { status: 403 })
  }

  // Persist reading
  const { error: insertError } = await supabase.from('meter_readings').insert({
    meter_id,
    kwh,
    timestamp,
  })

  if (insertError) {
    return NextResponse.json({ error: 'Failed to save reading' }, { status: 500 })
  }

  // Mint proto-certificates on Stellar (1 token = 1 kWh)
  const txHash = await mintCertificates({ cooperativeId: meter.cooperative_id, kwh })

  return NextResponse.json({ success: true, tx_hash: txHash }, { status: 201 })
}
