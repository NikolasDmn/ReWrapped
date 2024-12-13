const { default: daisyui } = require("daisyui");


module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  darkMode: "media", // 'media' or 'class'
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        spotify: {
          "primary": "#1DB954", // Spotify's iconic green
          "secondary": "#212121", // Dark blackish background
          "accent": "#535353", // Muted grey for subtle accents
          "neutral": "#121212", // Deep black for cards and containers
          "base-100": "#000000", // Pure black for body background
          "info": "#1DB954", // Reusing the green for info highlights
          "success": "#1DB954", // Spotify green = success
          "warning": "#ECB22E", // Golden yellow for warnings
          "error": "#E62429", // Bright error red
          "text-base": "#FFFFFF", // White default text color
        },
      },
      "dark", // Include the "dark" theme as a fallback option
    ],
  },
};

























































































































































