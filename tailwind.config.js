/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx}",
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./Components/*.tsx",

    // Or if using `src` directory:
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    fontFamily: {
      bahnschrift: ["Bahnschrift", "sans-serif"],
      helvetica: ["Helvetica", "sans-serif"],
      century_gothic: ["Century_Gothic"],
    },
    extend: {
      colors: {
        dark: "#101010",
        darklight: "#1c1b1b",
      },
    },
  },
  darkMode: "class",
  plugins: [],
};
