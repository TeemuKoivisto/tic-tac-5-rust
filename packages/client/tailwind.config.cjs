const { screens } = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx,svelte}'],
  theme: {
    extend: {
      colors: {
        'primary-lighter': '#eff5ff',
        'primary-light': '#ccdffe',
        primary: '#2979fa',
        'primary-dark': '#356bff', // bg-blue-700 '#1c4ed8',
        'primary-darker': '#2161c8',

        secondary: '#f2f7ff',

        'gray-custom': '#f7f9fc',
      },
    },
    screens: {
      xs: '480px',
      ...screens,
    },
  },
  plugins: [],
}
