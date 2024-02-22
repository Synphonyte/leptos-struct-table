/** @type {import('tailwindcss').Config} */
const colors = require( 'tailwindcss/colors' );

module.exports = {
  content: {
    files: [ '*.html', './src/**/*.rs' ],
  },
  darkMode: 'class',
  plugins: [
    require( '@tailwindcss/forms' ),
  ],
};