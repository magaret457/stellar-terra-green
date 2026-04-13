import { NextRequest, NextResponse } from 'next/server'
import { createClient } from '@/lib/supabase/server'
import { retireCertificate } from '@/lib/stellar/retire'

/**
 * POST /api/certificates/[id]/retire
 * Burns the on-chain token and marks the certificate as retired.
 */
export async function POST(req: NextRequest, { params }: { params: Promise<{ id: string }> }) {
  const { id } = await params
  const { buyer_address } = await req.json()

  if (!buyer_address) {
    return NextResponse.json({ error: 'buyer_address is required' }, { status: 400 })
  }

  const supabase = await createClient()

  const { data: cert } = await supabase
    .from('certificates')
    .select('id, token_id, retired_at')
    .eq('id', id)
    .single()

  if (!cert) return NextResponse.json({ error: 'Not found' }, { status: 404 })
  if (cert.retired_at) return NextResponse.json({ error: 'Already retired' }, { status: 409 })

  const txHash = await retireCertificate({ tokenId: cert.token_id, buyerAddress: buyer_address })

  await supabase
    .from('certificates')
    .update({ retired_at: new Date().toISOString(), buyer_address })
    .eq('id', id)

  return NextResponse.json({ success: true, tx_hash: txHash })
}
