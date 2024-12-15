/** @type {import('tailwindcss').Config} */

module.exports = {
  content: ["./src/**/*.{tsx}", 
            "./.tuono/client-main.tsx"
  ],
  theme: {
    extend: {},
  },
  plugins: [require("tailwindcss")],
}

