import './globals.css'
import type { Metadata } from 'next'
import { Chivo_Mono } from 'next/font/google'

export const chivoMono = Chivo_Mono({
  subsets: ['latin'],
  weight: ['400', '700'],
  style: 'normal',
});

export const metadata: Metadata = {
  title: 'Cowsay',
  description: '',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
