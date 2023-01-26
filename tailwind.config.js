/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        fontFamily: {
            'rubik': ['Rubik', 'sans-serif'],
        }
    },
    plugins: [],
}
