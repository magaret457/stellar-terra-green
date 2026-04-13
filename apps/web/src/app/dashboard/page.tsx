import { createClient } from '@/lib/supabase/server'

export default async function DashboardPage() {
  const supabase = await createClient()
  const { data: stats } = await supabase.rpc('get_cooperative_stats')

  return (
    <div className="p-8">
      <h1 className="mb-6 text-2xl font-bold">Cooperative Dashboard</h1>
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
        <StatCard label="Total Members" value={stats?.member_count ?? '—'} />
        <StatCard label="Active Meters" value={stats?.meter_count ?? '—'} />
        <StatCard
          label="Certificates Issued"
          value={stats?.certificates_issued ? `${stats.certificates_issued} kWh` : '—'}
        />
      </div>
    </div>
  )
}

function StatCard({ label, value }: { label: string; value: string | number }) {
  return (
    <div className="rounded-lg border border-gray-200 p-6 dark:border-gray-700">
      <p className="text-sm text-gray-500">{label}</p>
      <p className="mt-1 text-3xl font-bold">{value}</p>
    </div>
  )
}
