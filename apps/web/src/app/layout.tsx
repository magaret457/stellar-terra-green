import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'SolarCert — Renewable Energy Certification on Stellar',
  description:
    'Cooperative management dashboard and on-chain certification infrastructure for renewable energy on Stellar.',
  openGraph: {
    title: 'SolarCert',
    description: 'Renewable energy certification infrastructure on Stellar',
    url: 'https://solarcert.vercel.app',
    siteName: 'SolarCert',
    locale: 'en_US',
    type: 'website',
  },
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
