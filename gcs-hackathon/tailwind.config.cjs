/**
 * Minimal Tailwind config created manually because `npx tailwindcss init -p` failed.
 */
module.exports = {
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}'
  ],
  safelist: [
    'bg-gray-50',
    'bg-green-800',
    'text-gray-800',
    'from-green-800',
    'to-green-600',
    'bg-gradient-to-r'
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
