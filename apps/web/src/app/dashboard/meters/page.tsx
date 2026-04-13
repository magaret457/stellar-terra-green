import { createClient } from '@/lib/supabase/server'

export default async function MetersPage() {
  const supabase = await createClient()
  const { data: meters } = await supabase
    .from('meters')
    .select('id, serial_number, location, capacity_kw, active')
    .order('serial_number')

  return (
    <div className="p-8">
      <h1 className="mb-6 text-2xl font-bold">Meters</h1>
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-gray-200 text-left dark:border-gray-700">
              <th className="pb-3 font-medium">Serial</th>
              <th className="pb-3 font-medium">Location</th>
              <th className="pb-3 font-medium">Capacity (kW)</th>
              <th className="pb-3 font-medium">Status</th>
            </tr>
          </thead>
          <tbody>
            {meters?.map((m) => (
              <tr key={m.id} className="border-b border-gray-100 dark:border-gray-800">
                <td className="py-3 font-mono text-xs">{m.serial_number}</td>
                <td className="py-3">{m.location}</td>
                <td className="py-3">{m.capacity_kw}</td>
                <td className="py-3">
                  <span
                    className={`rounded-full px-2 py-0.5 text-xs font-medium ${
                      m.active
                        ? 'bg-green-100 text-green-700'
                        : 'bg-gray-100 text-gray-500'
                    }`}
                  >
                    {m.active ? 'Active' : 'Inactive'}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  )
}
