export default function HomePage() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-8">
      <div className="max-w-2xl text-center">
        <h1 className="mb-4 text-4xl font-bold tracking-tight">☀️ SolarCert</h1>
        <p className="mb-8 text-lg text-gray-600 dark:text-gray-400">
          Renewable energy certification infrastructure on Stellar
        </p>
        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
          <a
            href="/dashboard"
            className="rounded-lg border border-gray-200 p-6 hover:border-green-500 hover:bg-green-50 dark:border-gray-700 dark:hover:bg-green-950"
          >
            <h2 className="mb-2 text-xl font-semibold">Dashboard →</h2>
            <p className="text-sm text-gray-500">Manage your cooperative, members, and meters</p>
          </a>
          <a
            href="/certificates"
            className="rounded-lg border border-gray-200 p-6 hover:border-green-500 hover:bg-green-50 dark:border-gray-700 dark:hover:bg-green-950"
          >
            <h2 className="mb-2 text-xl font-semibold">Certificates →</h2>
            <p className="text-sm text-gray-500">Browse and retire energy certificates on-chain</p>
          </a>
        </div>
      </div>
    </main>
  )
}
