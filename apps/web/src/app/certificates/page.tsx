import { createClient } from '@/lib/supabase/server'

export default async function CertificatesPage() {
  const supabase = await createClient()
  const { data: certs } = await supabase
    .from('certificates')
    .select('id, token_id, amount_kwh, issued_at, retired_at, buyer_address')
    .order('issued_at', { ascending: false })
    .limit(50)

  return (
    <div className="p-8">
      <h1 className="mb-6 text-2xl font-bold">Certificates</h1>
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-gray-200 text-left dark:border-gray-700">
              <th className="pb-3 font-medium">Token ID</th>
              <th className="pb-3 font-medium">Amount (kWh)</th>
              <th className="pb-3 font-medium">Issued</th>
              <th className="pb-3 font-medium">Status</th>
              <th className="pb-3 font-medium">Buyer</th>
            </tr>
          </thead>
          <tbody>
            {certs?.map((c) => (
              <tr key={c.id} className="border-b border-gray-100 dark:border-gray-800">
                <td className="py-3 font-mono text-xs">{c.token_id}</td>
                <td className="py-3">{c.amount_kwh}</td>
                <td className="py-3 text-gray-500">
                  {new Date(c.issued_at).toLocaleDateString()}
                </td>
                <td className="py-3">
                  <span
                    className={`rounded-full px-2 py-0.5 text-xs font-medium ${
                      c.retired_at
                        ? 'bg-gray-100 text-gray-500'
                        : 'bg-green-100 text-green-700'
                    }`}
                  >
                    {c.retired_at ? 'Retired' : 'Active'}
                  </span>
                </td>
                <td className="py-3 font-mono text-xs">{c.buyer_address ?? '—'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}
