const defaultTheme = require("tailwindcss/defaultTheme")

module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#262626',
        'primary-dark': '#171717',
        'primary-light': '#404040',
        secondary: '#f5f5f5',
        'secondary-light': '#fafafa',
        'secondary-dark': '#e5e5e5',
        accent: "#facc15",
        'accent-secondary': "#ef4444"
      },
      fontFamily: {
        "Josefin": ["Josefin Sans", ...defaultTheme.fontFamily.sans],
      }
    }
  },
  plugins: [],
}
