/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./static/*.html', './src/**/*.rs'],
  theme: {
    extend: {},
  },
  // npm i -D daisyui@latest
  plugins: [require('daisyui')],
}
